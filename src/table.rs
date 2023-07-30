use std::fmt::Display;

pub trait Iden: Display + Clone + Copy {}
impl<T> Iden for T where T: Display + Clone + Copy {}

pub trait Table: Iden + Unquote {
    type Unquoted: Table;
    const UNQUOTED: Self::Unquoted;
    const QUOTE: &'static str;
}

pub trait TableColumn: Iden + Unquote {
    type Table: Table;
    type Unquoted: TableColumn;
    const TABLE: Self::Table;
    const QUOTE: &'static str;
}

pub trait Unquote: Iden {
    type Target: Iden;
    fn unquoted(self) -> Self::Target;
}

#[macro_export]
macro_rules! table {
    (
        $name:ident: $table_name:literal {
            $($col:ident: $col_name:literal),+ $(,)?
        } $(,)?
    ) => {
        table!($name: $table_name {
            $($col: $col_name),+
        }, quote: "");
    };
    (
        $name:ident: $table_name:literal {
            $($col:ident: $col_name:literal),+ $(,)?
        }, quote: $q:literal $(,)?
    ) => {

        sql_table::paste::paste! {
            #[derive(Debug, Clone, Copy)]
            pub struct $name;

            #[allow(non_upper_case_globals)]
            impl $name {
                $(
                    const [<$col>]: [<$name Column>] = [<$name Column>]::$col;
                )+
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(&format!("{}{}{}", $q, $table_name, $q))
                }
            }

            #[derive(Debug, Clone, Copy)]
            pub enum [<$name Column>] {
                $($col),+
            }

            impl std::fmt::Display for [<$name Column>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $(
                            Self::$col => f.write_str(&format!("{}{}{}", $q, $col_name, $q)),
                        )+
                    }
                }
            }

            impl sql_table::table::TableColumn for [<$name Column>] {
                type Table = $name;
                type Unquoted = [<$name UnquotedColumn>];
                const TABLE: $name = $name;
                const QUOTE: &'static str = $q;
            }

            #[derive(Debug, Clone, Copy)]
            pub struct [<Unquoted $name>];

            #[allow(non_upper_case_globals)]
            impl [<Unquoted $name>] {
                $(
                    const [<$col>]: [<Unquoted $name Column>] 
                        = [<Unquoted $name Column>]::$col;
                )+
            }

            impl std::fmt::Display for [<Unquoted $name>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str($table_name)
                }
            }

            impl sql_table::table::Unquote for $name {
                type Target = <Self as sql_table::table::Table>::Unquoted;
                fn unquoted(self) -> Self::Target {
                    [<Unquoted $name>]
                }
            }

            impl sql_table::table::Table for $name {
                type Unquoted = [<Unquoted $name>];
                const UNQUOTED: Self::Unquoted = [<Unquoted $name>];
                const QUOTE: &'static str = $q;
            }

            impl sql_table::table::Unquote for [<Unquoted $name>] {
                type Target = <Self as sql_table::table::Table>::Unquoted;
                fn unquoted(self) -> Self::Target {
                    self
                }
            }

            impl sql_table::table::Table for [<Unquoted $name>] {
                type Unquoted = Self;
                const UNQUOTED: Self::Unquoted = [<Unquoted $name>];
                const QUOTE: &'static str = "";
            }

            #[derive(Debug, Clone, Copy)]
            pub enum [<$name UnquotedColumn>] {
                $($col),+
            }

            impl std::fmt::Display for [<$name UnquotedColumn>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $(
                            Self::$col => f.write_str($col_name),
                        )+
                    }
                }
            }

            impl sql_table::table::TableColumn for [<$name UnquotedColumn>] {
                type Table = [<$name>];
                type Unquoted = Self;
                const TABLE: [<$name>] = [<$name>];
                const QUOTE: &'static str = "";
            }

            #[derive(Debug, Clone, Copy)]
            pub enum [<Unquoted $name Column>] {
                $($col),+
            }

            impl std::fmt::Display for [<Unquoted $name Column>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $(
                            Self::$col => f.write_str($col_name),
                        )+
                    }
                }
            }

            impl sql_table::table::TableColumn for [<Unquoted $name Column>] {
                type Table = [<Unquoted $name>];
                type Unquoted = Self;
                const TABLE: [<Unquoted $name>] = [<Unquoted $name>];
                const QUOTE: &'static str = $q;
            }

            #[derive(Debug, Clone, Copy)]
            pub enum [<Unquoted $name UnquotedColumn>] {
                $($col),+
            }

            impl std::fmt::Display for [<Unquoted $name UnquotedColumn>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $(
                            Self::$col => f.write_str($col_name),
                        )+
                    }
                }
            }

            impl sql_table::table::TableColumn for [<Unquoted $name UnquotedColumn>] {
                type Table = [<Unquoted $name>];
                type Unquoted = Self;
                const TABLE: [<Unquoted $name>] = [<Unquoted $name>];
                const QUOTE: &'static str = "";
            }

            impl sql_table::table::Unquote for [<$name Column>] {
                type Target = [<$name UnquotedColumn>];
                fn unquoted(self) -> [<$name UnquotedColumn>] {
                    match self {
                        $(
                            Self::$col => Self::Target::$col,
                        )+
                    }
                }
            }

            impl sql_table::table::Unquote for [<$name UnquotedColumn>] {
                type Target = Self;
                fn unquoted(self) -> Self {
                    match self {
                        $(
                            Self::$col => Self::Target::$col,
                        )+
                    }
                }
            }

            impl sql_table::table::Unquote for [<Unquoted $name Column>] {
                type Target = [<Unquoted $name UnquotedColumn>];
                fn unquoted(self) -> Self::Target {
                    match self {
                        $(
                            Self::$col => Self::Target::$col,
                        )+
                    }
                }
            }

            impl sql_table::table::Unquote for [<Unquoted $name UnquotedColumn>] {
                type Target = Self;
                fn unquoted(self) -> Self {
                    match self {
                        $(
                            Self::$col => Self::Target::$col,
                        )+
                    }
                }
            }
        }
    };
}
