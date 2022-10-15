use crate::table::{Iden, TableColumn, Unquote};

pub trait IndexName: Unquote + Iden {
    fn index_name<C>(cols: &[C]) -> String
    where
        C: TableColumn + Unquote,
    {
        let q = C::QUOTE;
        format!(
            "{q}ix_{}{}{q}",
            C::TABLE.unquoted(),
            cols.iter()
                .fold(String::new(), |acc, x| format!("{}_{}", acc, x.unquoted()))
        )
    }
}

impl<T> IndexName for T where T: Unquote + Iden {}
