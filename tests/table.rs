use sql_table::{
    foreign_key_name::ForeignKeyName, index_name::IndexName, qualified::Qualified, table,
    table::TableColumn, table::Unquote,
};

table!(Person: "person" {
    Forename: "forename",
    Surname: "surname",
});

#[test]
fn no_quotes() {
    assert_eq!(Person.to_string(), "person");
    assert_eq!(Person.to_string(), Person.unquoted().to_string());
    assert_eq!(Person::Forename.to_string(), "forename");
    assert_eq!(
        Person::Forename.to_string(),
        Person::Forename.unquoted().to_string()
    );
    assert_eq!(Person::Surname.to_string(), "surname");
    assert_eq!(
        Person::Surname.to_string(),
        Person::Surname.unquoted().to_string()
    );
    assert_eq!(PersonColumn::TABLE.to_string(), Person.to_string());
    assert_eq!(Person::QUOTE, "");
}

table!(PhoneNumber: "phone number" {
    DialingCode: "dialing code",
    Rest: "rest",
}, quote: r#"`"#);

#[test]
fn with_quotes() {
    assert_eq!(PhoneNumber.to_string(), "`phone number`");
    assert_eq!(PhoneNumber.unquoted().to_string(), "phone number");
    assert_eq!(PhoneNumber::DialingCode.to_string(), "`dialing code`");
    assert_eq!(
        PhoneNumber::DialingCode.unquoted().to_string(),
        "dialing code"
    );
    assert_eq!(PhoneNumber::Rest.to_string(), "`rest`");
    assert_eq!(PhoneNumber::Rest.unquoted().to_string(), "rest");
    assert_eq!(
        PhoneNumberColumn::TABLE.to_string(),
        PhoneNumber.to_string()
    );
    assert_eq!(PhoneNumber::QUOTE, "`");
}

#[test]
fn foreign_key_name() {
    assert_eq!(
        Person::Forename.foreign_key_name(Person::Surname),
        "fk_person_forename_person_surname"
    );
    assert_eq!(
        Person::Forename.foreign_key_name(PhoneNumber::Rest),
        "`fk_person_forename_phone number_rest`"
    );
    assert_eq!(
        PhoneNumber::DialingCode.foreign_key_name(Person::Forename),
        "`fk_phone number_dialing code_person_forename`"
    );
}

#[test]
fn index_name() {
    assert_eq!(
        Person::index_name(&[Person::Forename]),
        "ix_person_forename"
    );
    assert_eq!(
        PhoneNumber::index_name(&[PhoneNumber::DialingCode, PhoneNumber::Rest]),
        "`ix_phone number_dialing code_rest`"
    );
}

#[test]
fn qualified_name() {
    assert_eq!(Person::Forename.qualified(), "person.forename");
    assert_eq!(
        PhoneNumber::DialingCode.qualified(),
        "`phone number`.`dialing code`"
    );
}
