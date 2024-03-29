use core::char::ParseCharError;
use core::num::{ParseFloatError, ParseIntError};
use core::str::ParseBoolError;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::string::ToString;

#[cfg(feature = "std")]
use std::error;
#[cfg(feature = "std")]
use std::net::AddrParseError;

/// The error returned when trying to parse a type using [`try_parse`](crate::try_parse) or [`Parse`](crate::Parse).
#[derive(Debug)]
pub enum ParseError {
    /// The variant returned when an integer cannot be parsed.
    Int(ParseIntError),
    /// The variant returned when a bool cannot be parsed.
    Bool(ParseBoolError),
    /// The variant returned when a char cannot be parsed.
    Char(ParseCharError),
    /// The variant returned when a float cannot be parsed.
    Float(ParseFloatError),
    #[cfg(feature = "std")]
    /// The variant returned when an ip address cannot be parsed.
    /// This variant is only enabled with the `std` feature.
    Addr(AddrParseError),
    #[cfg(feature = "std")]
    /// The variant returned when you want to return an error that is not defined here.
    ///
    /// This variant is only enabled with the `std` feature as the
    /// [`Error`](error::Error) trait is a part of std.
    Dyn(Box<dyn error::Error + Send + Sync>),
    /// The variant returned when [`parse!`](crate::parse) found an unexpected literal.
    /// When not using the `alloc` feature, `Literal` is a unit variant.
    #[cfg(feature = "alloc")]
    Literal {
        /// What it expected.
        expected: String,
        /// What it actually found.
        found: String,
    },
    /// The variant returned when [`parse!`](crate::parse) found an unexpected literal.
    /// When not using the `alloc` feature, `Literal` is a unit variant.
    #[cfg(not(feature = "alloc"))]
    Literal,
    /// The variant returned when parsing an array and finding more or less elements than what was expected.
    Array {
        /// The size of the array it was expecting.
        expected: u8,
        /// The size of the array it found.
        found: u8,
    },
    /// A variant that can be used when you need to return a simple error.
    /// When not using the `alloc` feature, `Other` is a unit variant.
    #[cfg(feature = "alloc")]
    Other(String),
    /// A variant that can be used when you need to return a simple error.
    /// When not using the `alloc` feature, `Other` is a unit variant.
    #[cfg(not(feature = "alloc"))]
    Other,
    #[cfg(feature = "alloc")]
    /// A variant that wraps a [`ParseError`] to add more context about the error
    /// when parsing repetition sequences.
    /// This variant is only enabled with the `alloc` feature.
    MultiContext {
        /// The string part of the repetition sequence that was trying to be parsed
        multi_string: String,
        /// The string that cause the parsing to fail
        failed_string: String,
        /// The wrapped error
        error: Box<ParseError>,
    },
    /// A variant that wraps a [`ParseError`] to add more context about the error.
    /// This variant is only enabled with the `alloc` feature.
    #[cfg(feature = "alloc")]
    Context {
        /// The full string that was trying to be parsed
        full_string: String,
        /// The string that cause the parsing to fail
        failed_item: String,
        /// The wrapped error
        error: Box<ParseError>,
    },
}

#[cfg(feature = "alloc")]
impl ParseError {
    /// Create a new ParseError from a printable error message.
    ///
    /// This function stores the passed message into the `Other` variant.
    /// And as such can only be used when using the `alloc` feature.
    ///
    /// This function can be especially useful when trying to implement [`Parse`](crate::Parse).
    /// ```
    /// # use prse::{Parse, ParseError, ExtParseStr, parse};
    /// # #[derive(PartialEq, Debug)]
    /// struct Bool(bool);
    ///
    /// impl<'a> Parse<'a> for Bool {
    ///     fn from_str(s: &'a str) -> Result<Self, ParseError> {
    ///         match s {
    ///             "false" | "False" => Ok(Bool(false)),
    ///             "true" | "True" => Ok(Bool(true)),
    ///             _ => Err(ParseError::new(format!("expected to find true or false but found {s}.")))
    ///         }   
    ///     }
    /// }
    ///
    /// # fn main() -> Result<(), ParseError> {
    /// let b: Bool = parse!("True", "{}");
    /// assert_eq!(b, Bool(true));
    /// # Ok(())}
    /// ```
    pub fn new<T: core::fmt::Display>(message: T) -> Self {
        Self::Other(message.to_string())
    }
}

#[cfg(feature = "std")]
impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParseError::Int(source) => Some(source),
            ParseError::Bool(source) => Some(source),
            ParseError::Char(source) => Some(source),
            ParseError::Float(source) => Some(source),
            ParseError::Addr(source) => Some(source),
            ParseError::Dyn(source) => Some(&**source),
            ParseError::MultiContext { error, .. } => Some(error),
            ParseError::Context { error, .. } => Some(error),
            ParseError::Literal { .. } | ParseError::Array { .. } | ParseError::Other(_) => None,
        }
    }
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            ParseError::Int(_) => write!(fmt, "unable to parse as an integer"),
            ParseError::Bool(_) => write!(fmt, "unable to parse as a boolean"),
            ParseError::Char(_) => write!(fmt, "unable to parse as a character"),
            ParseError::Float(_) => write!(fmt, "unable to parse as a float"),
            #[cfg(feature = "std")]
            ParseError::Addr(_) => write!(fmt, "unable to parse as an address"),
            #[cfg(feature = "std")]
            ParseError::Dyn(_) => write!(fmt, "unable to parse into type"),
            #[cfg(feature = "alloc")]
            ParseError::Literal { expected, found } => write!(
                fmt,
                "invalid literal match (expected to find {expected:?}, found {found:?})"
            ),
            #[cfg(not(feature = "alloc"))]
            ParseError::Literal => write!(fmt, "invalid literal match"),
            ParseError::Array { expected, found } => write!(
                fmt,
                "invalid number of items (expected to find {expected:?}, found {found:?})"
            ),
            #[cfg(feature = "alloc")]
            ParseError::Other(message) => write!(fmt, "{message}"),
            #[cfg(not(feature = "alloc"))]
            ParseError::Other => write!(fmt, "unable to parse into type"),
            #[cfg(feature = "alloc")]
            ParseError::MultiContext {
                multi_string,
                failed_string: failed_item,
                error,
            } => {
                write!(
                    fmt,
                    "unable to parse multi-item \"{failed_item}\" when parsing \"{multi_string}\":\n\t{error}"
                )
            }
            #[cfg(feature = "alloc")]
            ParseError::Context {
                full_string,
                failed_item,
                error,
            } => {
                write!(
                    fmt,
                    "unable to parse \"{failed_item}\" when parsing \"{full_string}\":\n\t{error}"
                )
            }
        }
    }
}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        use ParseError as E;

        match (self, other) {
            (E::Int(x), E::Int(y)) => x == y,
            (E::Bool(x), E::Bool(y)) => x == y,
            (E::Char(x), E::Char(y)) => x == y,
            (E::Float(x), E::Float(y)) => x == y,
            #[cfg(feature = "std")]
            (E::Addr(x), E::Addr(y)) => x == y,
            #[cfg(feature = "alloc")]
            (
                E::Literal {
                    expected: lx,
                    found: ly,
                },
                E::Literal {
                    expected: rx,
                    found: ry,
                },
            ) => lx == rx && ly == ry,
            #[cfg(not(feature = "alloc"))]
            (E::Literal, E::Literal) => true,
            (
                E::Array {
                    expected: lx,
                    found: ly,
                },
                E::Array {
                    expected: rx,
                    found: ry,
                },
            ) => lx == rx && ly == ry,
            #[cfg(feature = "alloc")]
            (E::Other(x), E::Other(y)) => x == y,
            #[cfg(not(feature = "alloc"))]
            (E::Other, E::Other) => true,
            #[cfg(feature = "alloc")]
            (
                E::MultiContext {
                    multi_string: lm,
                    failed_string: lf,
                    error: le,
                },
                E::MultiContext {
                    multi_string: rm,
                    failed_string: rf,
                    error: re,
                },
            ) => lm == rm && lf == rf && le == re,
            #[cfg(feature = "alloc")]
            (
                E::Context {
                    full_string: ls,
                    failed_item: lf,
                    error: le,
                },
                E::Context {
                    full_string: rs,
                    failed_item: rf,
                    error: re,
                },
            ) => ls == rs && lf == rf && le == re,
            _ => false,
        }
    }
}

macro_rules! impl_from_parse_error {
    ($Ty: ty, $Id: ident) => {
        impl From<$Ty> for ParseError {
            fn from(source: $Ty) -> Self {
                ParseError::$Id(source)
            }
        }
    };
}

impl_from_parse_error!(ParseIntError, Int);
impl_from_parse_error!(ParseBoolError, Bool);
impl_from_parse_error!(ParseCharError, Char);
impl_from_parse_error!(ParseFloatError, Float);
#[cfg(feature = "std")]
impl_from_parse_error!(AddrParseError, Addr);
#[cfg(feature = "std")]
impl_from_parse_error!(Box<dyn error::Error + Send + Sync>, Dyn);

#[cfg(feature = "alloc")]
impl From<()> for ParseError {
    fn from(_: ()) -> Self {
        ParseError::Other(String::from("Error: ()"))
    }
}

#[cfg(not(feature = "alloc"))]
impl From<()> for ParseError {
    fn from(_: ()) -> Self {
        ParseError::Other
    }
}

#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "alloc")]
    use super::{Box, ToString};
    use crate::{ExtParseStr, Parse, ParseError};

    #[doc(hidden)]
    /// Not part of public api used to unwrap the result when parsing.
    pub fn unwrap_parse<T>(result: Result<T, ParseError>) -> T {
        match result {
            Ok(x) => x,
            Err(e) => panic!("Unable to parse input:\n\t{e}"),
        }
    }

    #[doc(hidden)]
    #[cfg(not(feature = "alloc"))]
    pub fn try_parse_context<'a, T: Parse<'a>>(
        item: &'a str,
        _full_string: &'a str,
    ) -> Result<T, ParseError> {
        item.lending_parse()
    }

    #[doc(hidden)]
    #[cfg(not(feature = "alloc"))]
    pub fn add_err_multi_context<T>(
        result: Result<T, ParseError>,
        _input: &str,
        _failed_item: &str,
    ) -> Result<T, ParseError> {
        result
    }

    #[doc(hidden)]
    #[cfg(feature = "alloc")]
    pub fn try_parse_context<'a, T: Parse<'a>>(
        item: &'a str,
        full_string: &'a str,
    ) -> Result<T, ParseError> {
        item.lending_parse().map_err(|e| ParseError::Context {
            full_string: full_string.to_string(),
            failed_item: item.to_string(),
            error: Box::new(e),
        })
    }

    #[doc(hidden)]
    #[cfg(feature = "alloc")]
    pub fn add_err_multi_context<T>(
        result: Result<T, ParseError>,
        input: &str,
        failed_item: &str,
    ) -> Result<T, ParseError> {
        result.map_err(|e| ParseError::MultiContext {
            multi_string: input.to_string(),
            failed_string: failed_item.to_string(),
            error: Box::new(e),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::ParseError;

    #[test]
    fn check_impl_traits() {
        fn is_send<T: Send>() {}
        fn is_sync<T: Sync>() {}

        is_send::<ParseError>();
        is_sync::<ParseError>();
    }
}
