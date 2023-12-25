pub mod event;
pub mod season;
pub mod team;

pub use event::*;
pub use season::*;
pub use team::*;

pub trait PaginatedQuery {
    fn page(self, page: i32) -> Self;
    fn per_page(self, per_page: i32) -> Self;
}

/// Implements [`std::fmt::Display`] for a query struct.
///
/// This will convert the query parameters provided to the struct into a single query string.
#[macro_export]
macro_rules! impl_query_display {
    ( $name:ident ) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut iter = self.query.iter();

                if let Some(first) = iter.next() {
                    write!(f, "?{}={}", first.0, first.1)?;
                } else {
                    return Ok(());
                }

                for (key, value) in iter {
                    write!(f, "&{key}={value}")?;
                }

                Ok(())
            }
        }
    };
}

/// Implements [`PaginatedQuery`] for a query struct.
#[macro_export]
macro_rules! impl_paginated_query {
    ( $name:ident ) => {
        impl crate::query::PaginatedQuery for $name {
            fn page(mut self, page: i32) -> Self {
                self.query.insert("page", page.to_string());
                self
            }

            fn per_page(mut self, per_page: i32) -> Self {
                self.query.insert("per_page", per_page.to_string());
                self
            }
        }
    };
}

pub use impl_query_display;
pub use impl_paginated_query;