#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod foreign_key_name;
pub mod index_name;
pub mod qualified;
pub mod table;

pub use foreign_key_name::ForeignKeyName;
pub use index_name::IndexName;
pub use qualified::Qualified;
pub use table::{Table, TableColumn, Unquote};

pub use paste;
pub use sql_table_inject::inject;
