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
fn from() {
    assert_eq!(Variants::from_str("x"), Ok(Variants::X));
    assert_eq!(Variants::from_str("y"), Ok(Variants::Y()));
    assert_eq!(Variants::from_str("z"), Ok(Variants::Z(0)));
    assert_eq!(Variants::from_str("zee"), Ok(Variants::Z(0)));
    assert_eq!(
        Variants::from_str("hello_world"),
        Ok(Variants::HelloWorld {
            x: String::default(),
            y: String::default(),
        })
    );
}

#[test]
fn err() {
    assert!(Variants::from_str("bad").is_err());
}
