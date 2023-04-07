use crate::table::{Iden, Table, TableColumn, Unquote};

pub trait IndexName: Unquote + Iden {
    fn index_name<C>(cols: &[C]) -> String
    where
        C: TableColumn,
    {
        let q = if !C::QUOTE.is_empty() {
            C::QUOTE
        } else {
            C::Table::QUOTE
        };
        format!(
            "{q}ix_{}{}{q}",
            C::TABLE.unquoted(),
            cols.iter()
                .fold(String::new(), |acc, x| format!("{}_{}", acc, x.unquoted()))
        )
    }
}

impl<T> IndexName for T where T: Unquote + Iden {}
