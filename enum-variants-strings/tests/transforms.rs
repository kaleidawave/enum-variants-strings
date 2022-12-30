use enum_variants_strings::EnumVariantsStrings;

#[test]
fn none_transform() {
    #[derive(Debug, PartialEq, EnumVariantsStrings)]
    #[enum_variants_strings_transform(transform = "none")]
    enum EnumA {
        Foo,
        Bar,
    }

    assert_eq!(EnumA::Foo.to_str(), "Foo");
    assert_eq!(EnumA::Bar.to_str(), "Bar");
    assert_eq!(EnumA::from_str("Bar"), Ok(EnumA::Bar));
    assert_eq!(EnumA::from_str("bar"), Err(&["Foo", "Bar"][..]));
}

#[test]
fn lower_case_transform() {
    #[derive(Debug, PartialEq, EnumVariantsStrings)]
    #[enum_variants_strings_transform(transform = "lower_case")]
    enum EnumB {
        Foo,
        Bar,
    }

    assert_eq!(EnumB::Foo.to_str(), "foo");
    assert_eq!(EnumB::Bar.to_str(), "bar");
    assert_eq!(EnumB::from_str("bar"), Ok(EnumB::Bar));
    assert_eq!(EnumB::from_str("Bar"), Err(&["foo", "bar"][..]));
}

#[test]
fn upper_case_transform() {
    #[derive(Debug, PartialEq, EnumVariantsStrings)]
    #[enum_variants_strings_transform(transform = "upper_case")]
    enum EnumC {
        Foo,
        Bar,
    }

    assert_eq!(EnumC::Foo.to_str(), "FOO");
    assert_eq!(EnumC::Bar.to_str(), "BAR");
    assert_eq!(EnumC::from_str("BAR"), Ok(EnumC::Bar));
    assert_eq!(EnumC::from_str("Bar"), Err(&["FOO", "BAR"][..]));
}

#[test]
fn snake_case_transform() {
    #[derive(Debug, PartialEq, EnumVariantsStrings)]
    #[enum_variants_strings_transform(transform = "snake_case")]
    enum EnumD {
        SomeMember1,
        SomeMemberFoo,
    }

    assert_eq!(EnumD::SomeMember1.to_str(), "some_member_1");
    assert_eq!(EnumD::SomeMemberFoo.to_str(), "some_member_foo");
}
