use crate::table::TableColumn;

pub trait Qualified: TableColumn {
    fn qualified(&self) -> String {
        format!("{}.{}", Self::TABLE, self)
    }
}

impl<C> Qualified for C where C: TableColumn {}
