#![doc = include_str!("../README.md")]

use std::iter;

use either_n::Either2;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::ParseStream, parse_macro_input, parse_quote, punctuated::Punctuated, Arm, Data,
    DeriveInput, Lit, LitStr, Meta, MetaNameValue, NestedMeta, Token,
};

/// Name of the attribute that is used for specifying a mapping other than it snake cased name
/// ```rs
/// enum X {
///     #[enum_variant_from_strings("z", "zed", "zee")]
///     Z
/// }
/// ```
const CUSTOM_VARIANT_STRING_MAPPING: &str = "enum_variants_strings_mappings";

/// For specifying the custom transform
const CUSTOM_VARIANT_STRING_TRANSFORM: &str = "enum_variants_strings_transform";

enum Transform {
    SnakeCase,
    UpperCase,
    LowerCase,
    KebabCase,
    None,
}

impl Default for Transform {
    fn default() -> Self {
        Transform::SnakeCase
    }
}

/// Ironically this is what this proc macro should generate
impl Transform {
    pub(crate) fn from_str(s: &str) -> Result<Self, UnknownCustomTransformError> {
        match s {
            "snake_case" => Ok(Self::SnakeCase),
            "upper_case" => Ok(Self::UpperCase),
            "lower_case" => Ok(Self::LowerCase),
            "kebab_case" => Ok(Self::KebabCase),
            "none" => Ok(Self::None),
            s => Err(UnknownCustomTransformError { transform: s }),
        }
    }

    pub(crate) fn apply_transform(&self, s: &str) -> String {
        match self {
            Transform::SnakeCase | Transform::KebabCase => {
                let mut peekable = s.chars().peekable();
                let mut string = String::new();
                let divider = if matches!(self, Transform::SnakeCase) {
                    '_'
                } else {
                    '-'
                };
                while let Some(character) = peekable.next() {
                    if let '_' | '-' = character {
                        string.push(divider);
                        continue;
                    }

                    string.extend(character.to_lowercase());
                    if character.is_lowercase()
                        && peekable
                            .peek()
                            .copied()
                            .map(|c| c.is_uppercase() || c.is_numeric())
                            .unwrap_or(false)
                    {
                        string.push(divider);
                    }
                }
                string
            }
            Transform::UpperCase => s.to_uppercase(),
            Transform::LowerCase => s.to_lowercase(),
            Transform::None => s.to_owned(),
        }
    }
}

struct UnknownCustomTransformError<'a> {
    transform: &'a str,
}

#[allow(clippy::from_over_into)]
impl<'a> Into<TokenStream> for UnknownCustomTransformError<'a> {
    fn into(self) -> TokenStream {
        let message = format!("Unknown transform '{}'", self.transform);
        quote!(compile_error!(#message)).into()
    }
}

#[proc_macro_derive(
    EnumVariantsStrings,
    attributes(enum_variants_strings_mappings, enum_variants_strings_transform)
)]
pub fn enum_variants_strings(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Enum(r#enum) = &input.data {
        let ident = &input.ident;

        let mapping: Option<Result<String, ()>> = input.attrs.iter().find_map(|attr| {
            attr.path
                .is_ident(CUSTOM_VARIANT_STRING_TRANSFORM)
                .then(|| {
                    if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                        if let (
                            1,
                            Some(NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                path,
                                lit: Lit::Str(lit_str),
                                ..
                            }))),
                        ) = (meta_list.nested.len(), meta_list.nested.first())
                        {
                            if path.is_ident("transform") {
                                return Ok(lit_str.value());
                            }
                        }
                    }
                    Err(())
                })
        });

        let mapping = match mapping.transpose() {
            Ok(mapping) => mapping,
            Err(_) => {
                return quote!(
                    compile_error!("Invalid usage of \"enum_variants_strings_transform\", check docs for usage");
                ).into();
            }
        };

        let transform = match mapping.as_deref().map(Transform::from_str).transpose() {
            Ok(transform) => transform.unwrap_or_default(),
            Err(err) => {
                return err.into();
            }
        };

        let (mut to_string_arms, mut from_string_arms) = (
            Vec::<Arm>::with_capacity(r#enum.variants.len()),
            Vec::<Arm>::with_capacity(r#enum.variants.len()),
        );
        let mut possible_matches = Vec::<String>::new();

        for variant in r#enum.variants.iter() {
            // A list of LitStr which match the variant
            let variant_names = if let Some(attr) = variant
                .attrs
                .iter()
                .find(|attr| attr.path.is_ident(CUSTOM_VARIANT_STRING_MAPPING))
            {
                let parse_args_result = attr.parse_args_with(|stream: ParseStream| {
                    stream.parse_terminated(|stream: ParseStream| stream.parse::<LitStr>())
                });
                let args: Punctuated<LitStr, Token![,]> = match parse_args_result {
                    Ok(args) => args,
                    Err(_) => {
                        return quote!(compile_error!(
                            "Failed to parse string arguments in custom mapping"
                        ))
                        .into();
                    }
                };

                Either2::One(args.into_iter())
            } else {
                Either2::Two(iter::once(LitStr::new(
                    &transform.apply_transform(&variant.ident.to_string()),
                    Span::call_site(),
                )))
            };

            let variant_name = &variant.ident;

            // Build default constructor for field.
            let variant_default_body = match &variant.fields {
                syn::Fields::Named(named) => {
                    let fields = named.named.iter().map(|field| {
                        let field_ident = &field.ident;
                        quote!(#field_ident: Default::default())
                    });
                    quote!( Self::#variant_name { #(#fields),* } )
                }
                syn::Fields::Unnamed(unnamed) => {
                    let fields = unnamed
                        .unnamed
                        .iter()
                        .map(|_field| quote!(Default::default()));
                    quote!( Self::#variant_name ( #(#fields),* ) )
                }
                syn::Fields::Unit => quote!( Self::#variant_name ),
            };

            possible_matches.extend(
                variant_names
                    .clone()
                    .into_iter()
                    .map(|lit_str| lit_str.value()),
            );

            // If multiple output names use last
            let last_str = variant_names.clone().last().unwrap();

            let to_string_pattern = match &variant.fields {
                syn::Fields::Named(_) => {
                    quote!( Self::#variant_name {..} )
                }
                syn::Fields::Unnamed(_) => {
                    quote!( Self::#variant_name (..) )
                }
                syn::Fields::Unit => quote!( Self::#variant_name ),
            };

            to_string_arms.push(parse_quote! {
                #to_string_pattern => #last_str
            });

            from_string_arms.push(parse_quote! {
                #(#variant_names)|* => Ok(#variant_default_body)
            });
        }

        quote! {
            impl ::enum_variants_strings::EnumVariantsStrings for #ident {
                fn from_str(input: &str) -> Result<Self, &[&str]> {
                    match input {
                        #(#from_string_arms),*,
                        _ => Err(&[#(#possible_matches),*])
                    }
                }

                fn to_str(&self) -> &'static str {
                    match self {
                        #(#to_string_arms),*
                    }
                }
            }
        }
        .into()
    } else {
        quote!(
            compile_error!("Can only implement 'EnumVariantsStrings' on a enum");
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::Transform;

    #[test]
    fn snake_case_test() {
        let transform = Transform::SnakeCase;
        assert_eq!(&transform.apply_transform("SomeStruct"), "some_struct");
        assert_eq!(&transform.apply_transform("SomeTLA"), "some_tla");
        assert_eq!(
            &transform.apply_transform("Member_With_underscore"),
            "member_with_underscore"
        );
        assert_eq!(&transform.apply_transform("Field6"), "field_6");
    }

    #[test]
    fn kebab_case_test() {
        let transform = Transform::KebabCase;
        assert_eq!(&transform.apply_transform("SomeStruct"), "some-struct");
        assert_eq!(&transform.apply_transform("SomeTLA"), "some-tla");
        assert_eq!(
            &transform.apply_transform("Member_With_underscore"),
            "member-with-underscore"
        );
        assert_eq!(&transform.apply_transform("Field6"), "field-6");
    }
}
