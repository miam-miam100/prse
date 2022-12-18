#[cfg(feature = "alloc")]
pub extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;
use core::num::*;
use core::str::FromStr;
#[cfg(feature = "std")]
use std::ffi::OsString;
#[cfg(feature = "std")]
use std::net::*;
#[cfg(feature = "std")]
use std::path::PathBuf;

use crate::parse_error::ParseError;

/// Parse a string into the implemented type, unlike [`FromStr`] this trait allows
/// you to borrow the string.
pub trait LendingFromStr<'a> {
    /// Parses a string `s` to return value of this type.
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return a [`ParseError`].
    fn from_str(s: &'a str) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl<'a> LendingFromStr<'a> for &'a str {
    fn from_str(s: &'a str) -> Result<Self, ParseError> {
        Ok(s)
    }
}

macro_rules! impl_lending_from_str {
    ( $( $Ty: ty )+) => {
        $(
            impl<'a> LendingFromStr<'a> for $Ty {
                fn from_str(s: &'a str) -> Result<Self, ParseError> {
                    <Self as FromStr>::from_str(&s).map_err(|e| e.into())
                }
            }
        )+
    };
}

macro_rules! impl_lending_from_str_infallible {
    ( $( $Ty: ty )+) => {
        $(
            impl<'a> LendingFromStr<'a> for $Ty {
                fn from_str(s: &'a str) -> Result<Self, ParseError> {
                    Ok(<Self as FromStr>::from_str(&s).unwrap())
                }
            }
        )+
    };
}

impl_lending_from_str!(isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128);
impl_lending_from_str!(bool char f32 f64);
impl_lending_from_str!(NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize);
impl_lending_from_str!(NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize);

#[cfg(feature = "alloc")]
impl_lending_from_str_infallible!(String);

#[cfg(feature = "std")]
impl_lending_from_str_infallible!(OsString PathBuf);
#[cfg(feature = "std")]
impl_lending_from_str!(IpAddr SocketAddr Ipv4Addr Ipv6Addr SocketAddrV4 SocketAddrV6);

#[doc(hidden)]
mod __private {
    pub trait Sealed {}

    impl Sealed for str {}
}

/// An str extension trait to allow you to call the `from_str` from [`LendingFromStr`]
/// without specifying the type.
///
/// The trait is sealed and cannot be implemented on any other type.
pub trait ExtParseStr: __private::Sealed {
    /// Parses the string slice into another type.
    ///
    /// lending_parse can parse into any type that implements the [`LendingFromStr`] trait.
    fn lending_parse<'a, F: LendingFromStr<'a>>(&'a self) -> Result<F, ParseError>;
}

impl ExtParseStr for str {
    fn lending_parse<'a, F: LendingFromStr<'a>>(&'a self) -> Result<F, ParseError> {
        LendingFromStr::from_str(self)
    }
}
