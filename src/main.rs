use inject::inject;
use table::{foreign_key_name::ForeignKeyName, index_name::IndexName, table, Unquote};

table!(MyTable: "my table" {
    Foo: "foo",
    Bar: "bar",
}, quote: r#"`"#);

fn main() {
    println!(
        "{}, {}, {}, {}",
        MyTable::Foo.foreign_key_name(MyTable::Bar),
        MyTable::index_name(&[MyTable::Foo, MyTable::Bar]),
        MyTable::Foo.unquoted(),
        UnquotedMyTable::QUOTE
    );

    println!("{}", inject!("select #{MyTable::Foo.unquoted()}"));
}
