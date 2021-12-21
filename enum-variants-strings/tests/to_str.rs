use enum_variants_strings::EnumVariantsStrings;

#[derive(Debug, PartialEq, EnumVariantsStrings)]
enum Variants {
    X,
    Y(),
    #[enum_variants_strings_mappings("z", "zee")]
    Z(i32),
    HelloWorld {
        x: String,
        y: String,
    },
}

#[test]
fn to() {
    assert_eq!(Variants::X.to_str(), "x");
    assert_eq!(Variants::Z(54).to_str(), "zee");
    assert_eq!(
        Variants::HelloWorld {
            x: "abc".into(),
            y: "xyz".into()
        }
        .to_str(),
        "hello_world"
    );
}
