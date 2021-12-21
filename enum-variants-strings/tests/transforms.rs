use enum_variants_strings::EnumVariantsStrings;

#[derive(Debug, PartialEq, EnumVariantsStrings)]
#[enum_variants_strings_transform(transform = "none")]
enum EnumA {
    Foo,
    Bar,
}

#[test]
fn none_transform() {
    assert_eq!(EnumA::Foo.to_str(), "Foo");
    assert_eq!(EnumA::Bar.to_str(), "Bar");
    assert_eq!(EnumA::from_str("Bar"), Ok(EnumA::Bar));
    assert_eq!(EnumA::from_str("bar"), Err(()));
}

#[derive(Debug, PartialEq, EnumVariantsStrings)]
#[enum_variants_strings_transform(transform = "lower_case")]
enum EnumB {
    Foo,
    Bar,
}

#[test]
fn lower_case_transform() {
    assert_eq!(EnumB::Foo.to_str(), "foo");
    assert_eq!(EnumB::Bar.to_str(), "bar");
    assert_eq!(EnumB::from_str("bar"), Ok(EnumB::Bar));
    assert_eq!(EnumB::from_str("Bar"), Err(()));
}

#[derive(Debug, PartialEq, EnumVariantsStrings)]
#[enum_variants_strings_transform(transform = "upper_case")]
enum EnumC {
    Foo,
    Bar,
}

#[test]
fn upper_case_transform() {
    assert_eq!(EnumC::Foo.to_str(), "FOO");
    assert_eq!(EnumC::Bar.to_str(), "BAR");
    assert_eq!(EnumC::from_str("BAR"), Ok(EnumC::Bar));
    assert_eq!(EnumC::from_str("Bar"), Err(()));
}
