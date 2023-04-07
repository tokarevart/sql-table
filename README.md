# sql-table

Makes no-ORM querying of SQL databases more concise.

[![Cargo](https://img.shields.io/crates/v/sql-table.svg)](https://crates.io/crates/sql-table)
[![Documentation](https://docs.rs/sql-table/badge.svg)](https://docs.rs/sql-table)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/tokarevart/sql-table)

## Examples

Basic usage:

```rust ignore
table!(Person: "person" {
    Name: "name",
    Country: "country",
});

assert_eq!(
    inject!("
        SELECT #{Person::Name}#
        FROM #{Person}#
        WHERE #{Person::Country}# = 'United States'
    "),
    format!("
        SELECT name
        FROM person
        WHERE country = 'United States'
    ")
);
```

If you need more sophisticated behaviour:

```rust ignore
// If you want a specific table identifiers to be displayed by default as quoted
// just add `quote` parameter after table definition.
table!(SimCard: "sim card" {
    Owner: "owner",
    PhoneNumber: "phone number",
}, quote: "`");

table!(Person: "person" {
    Name: "name",
    Country: "country",
});

// Identifiers can also be unquoted using `.unquoted()` method,
// or, in case of field names, they can be converted into qualified form 
// using `.qualified()` method of trait `Qualified` provided by this library
// and implemented for all fields of each generated table.
// Said methods can also be chained `.unquoted().qualified()`.
assert_eq!(
    inject!("
        SELECT #{SimCard::PhoneNumber.qualified()}#
        FROM #{SimCard}#
        JOIN #{Person}# ON #{Person::Name.qualified()}# = #{SimCard::Owner.unquoted().qualified()}#
        WHERE #{Person::Country.qualified()}# = 'United States'
    "),
    format!("
        SELECT `sim card`.`phone number`
        FROM `sim card`
        JOIN person ON person.name = `sim card`.owner
        WHERE #{Person::Country.qualified()}# = 'United States'
    ")
);

// Format: fk_<target table>_<target field>_<source table>_<source field>
// The whole name will get quoted if any of it's components (table/field names) are quoted.
// In this case quotation will be done using the first found quote
// of corresponding components listed in the key name.
assert_eq!(
    SimCard::Owner.foreign_key_name(Person::Name),
    "`fk_sim card_owner_person_name`"
);
assert_eq!(
    <SimCard as Table>::Unquoted::Owner.unquoted().foreign_key_name(Person::Name),
    "fk_sim card_owner_person_name"
);

// Format: ix_<table>_<field 1>_<field 2>_..._<field n>
// All fields must be either quoted or unquoted.
// The whole name will get quoted if either table or fields are quoted.
assert_eq!(
    Person::index_name(&[Person::Name]),
    "ix_person_name"
);
assert_eq!(
    SimCard::index_name(&[SimCard::Owner, SimCard::PhoneNumber]),
    "`ix_sim card_owner_phone number`"
);
assert_eq!(
    <SimCard as Table>::Unquoted::index_name(&[
        <SimCard as Table>::Unquoted::Owner.unquoted(), 
        <SimCard as Table>::Unquoted::PhoneNumber.unquoted()
    ]),
    "ix_sim card_owner_phone number"
);

// If you need a different naming format for foreign keys, indices or anything else
// you can implement custom `IndexName`, `ForeignKeyName`, etc. traits
// using provided by this library default implementations as a reference.
```
