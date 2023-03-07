#![doc = include_str!("../README.md")]

pub use enum_variants_strings_derive::EnumVariantsStrings;

pub trait EnumVariantsStrings: Sized {
    /// Returns a instance of variant of Self which matches input if exists, else returns possible matches
    fn from_str(input: &str) -> Result<Self, &[&str]>;

    /// Returns the string representation of selfs variant
    fn to_str(&self) -> &'static str;
}
