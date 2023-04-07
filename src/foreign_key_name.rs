use crate::table::{Table, TableColumn, Unquote};

pub trait ForeignKeyName: TableColumn {
    fn foreign_key_name<F: TableColumn>(&self, fcol: F) -> String {
        let q = if !Self::QUOTE.is_empty() {
            Self::QUOTE
        } else if !Self::Table::QUOTE.is_empty() {
            Self::Table::QUOTE
        } else if !F::QUOTE.is_empty() {
            F::QUOTE
        } else {
            F::Table::QUOTE
        };
        format!(
            "{q}fk_{}_{}_{}_{}{q}",
            Self::TABLE.unquoted(),
            self.unquoted(),
            F::TABLE.unquoted(),
            fcol.unquoted()
        )
    }
}

impl<C> ForeignKeyName for C where C: TableColumn {}
