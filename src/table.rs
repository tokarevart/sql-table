use std::fmt::Display;

pub trait Iden: Display + Clone + Copy {}
impl<T> Iden for T where T: Display + Clone + Copy {}

pub trait TableColumn: Iden + Unquote {
    type Table: Iden + Unquote;
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

        paste::paste! {
            #[derive(Debug, Clone, Copy)]
            pub struct $name;

            impl $name {
                const QUOTE: &'static str = $q;
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
                const TABLE: $name = $name;
                const QUOTE: &'static str = $q;
            }

            #[derive(Debug, Clone, Copy)]
            pub struct [<Unquoted $name>];

            impl [<Unquoted $name>] {
                const QUOTE: &'static str = "";
                $(
                    const [<$col>]: [<Unquoted $name Column>] = [<Unquoted $name Column>]::$col;
                )+
            }

            impl std::fmt::Display for [<Unquoted $name>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str($table_name)
                }
            }

            impl sql_table::table::Unquote for $name {
                type Target = [<Unquoted $name>];
                fn unquoted(self) -> Self::Target {
                    [<Unquoted $name>]
                }
            }

            impl sql_table::table::Unquote for [<Unquoted $name>] {
                type Target = [<Unquoted $name>];
                fn unquoted(self) -> Self::Target {
                    [<Unquoted $name>]
                }
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
                const TABLE: [<Unquoted $name>] = [<Unquoted $name>];
                const QUOTE: &'static str = "";
            }

            impl sql_table::table::Unquote for [<$name Column>] {
                type Target = [<Unquoted $name Column>];
                fn unquoted(self) -> [<Unquoted $name Column>] {
                    match self {
                        $(
                            Self::$col => Self::Target::$col,
                        )+
                    }
                }
            }

            impl sql_table::table::Unquote for [<Unquoted $name Column>] {
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
