# Enum variants strings

[![Crates](https://img.shields.io/crates/v/enum-variants-strings)](https://crates.io/crates/enum-variants-strings)
[![Docs](https://docs.rs/enum-variants-strings/badge.svg)](https://docs.rs/enum-variants-strings/)

Generates conversions of enums from strings and into strings based on variant identifiers

```rust
use enum_variants_strings::EnumVariantsStrings;

#[derive(Debug, PartialEq, EnumVariantsStrings)]
enum Variants {
    X,
    Y(i32),
    #[enum_variants_strings_mappings("z", "zee")]
    Z {
        x: String,
        y: String,
    },
}

fn main() {
    assert_eq!(Variants::from_str("x"), Ok(Variants::X));
    assert_eq!(Variants::from_str("y"), Ok(Variants::Y(0)));
    assert_eq!(
        Variants::from_str("z"),
        Ok(Variants::Z {
            x: String::default(),
            y: String::default(),
        })
    );

    assert_eq!(Variants::X.to_str(), "x");
    assert_eq!(
        Variants::Z {
            x: "abc".into(),
            y: "xyz".into()
        }
        .to_str(),
        "zee"
    );
}
```

### Identifier mapping

**By default variant identifier/names are transformed to their snake case version**

This can be changed via `#[enum_variants_strings_transform(transform = ...)]`

```rust
#[derive(Debug, PartialEq, EnumVariantsStrings)]
#[enum_variants_strings_transform(transform = "none")]
enum EnumA {
    Foo,
    Bar,
}
```

There are several transforms

- `"none"`, no mapping from source
- `"upper_case"`, uppercase of identifier in source
- `"lower_case"`, lowercase of identifier in source
- `"snake_case"`, (default)
