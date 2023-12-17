pub mod event;
pub mod season;
pub mod team;

pub use event::*;
pub use season::*;
pub use team::*;

/// Implements [`std::fmt::Display`] for a filter struct.
/// 
/// This will convert the query parameters provided to the struct into a single query string.
#[macro_export]
macro_rules! impl_filter_display {
    ( $name:ident ) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut iter = self.query.iter();

                if let Some(first) = iter.next() {
                    write!(f, "?{}={}", first.0, first.1);
                } else {
                    return Ok(());
                }

                for (key, value) in iter {
                    write!(f, "&{key}={value}");
                }

                Ok(())
            }
        }
    };
}

pub use impl_filter_display;