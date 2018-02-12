//! This crate provides a `guid!` macro for expressing
//! [`GUID`](https://docs.rs/winapi/0.3.4/x86_64-pc-windows-msvc/winapi/shared/guiddef/struct.GUID.html)
//! structs with a convenient literal syntax. A GUID is a Windows
//! [_globally unique identifier_](https://msdn.microsoft.com/en-us/library/windows/desktop/aa368767(v=vs.85).aspx),
//! usually expressed in the following format:
//!
//! ```text
//! {6B29FC40-CA47-1067-B31D-00DD010662DA}
//! ```
//!
//! With this crate, a GUID can be generated with the syntax:
//!
//! ```
//! # #[macro_use]
//! # extern crate guid;
//! # use guid::GUID;
//! # fn main() {
//! # let _ : GUID =
//! guid!{"6B29FC40-CA47-1067-B31D-00DD010662DA"}
//! # ;
//! # }
//! ```
//!
//! # Example
//!
//! ```
//! #[macro_use]
//! extern crate guid;
//!
//! use guid::GUID;
//!
//! const MY_GUID: GUID = guid!{"6B29FC40-CA47-1067-B31D-00DD010662DA"};
//!
//! fn main() {
//!     assert_eq!(MY_GUID.Data1, 0x6B29FC40);
//!     assert_eq!(MY_GUID.Data2, 0xCA47);
//!     assert_eq!(MY_GUID.Data3, 0x1067);
//!     assert_eq!(MY_GUID.Data4, [ 0xB3, 0x1D, 0x00, 0xDD, 0x01, 0x06, 0x62, 0xDA ]);
//! }
//! ```
//!
//! The `GUID` type is re-exported from the [winapi](https://crates.io/crates/winapi) crate,
//! and is only available in Windows. The `guid!` macro is also only available in Windows.
//!
//! This crate also provides a parser, which can be used to parse GUID strings at runtime.
//! The parser is only available to generate an array of bytes on non-Windows platforms.
//! In Windows, this crate defines a parser that produces a `GUID` struct.
//!
//! # Compatibility
//!
//! This crate supports all versions of Rust (stable and nightly) starting with Rust 1.15.

extern crate chomp;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate guid_parser;

use chomp::prelude::*;

use guid_parser::Chunks;

use std::string::ToString;

/// Error returned whenever a string fails to parse as a GUID.
#[derive(Fail, Debug)]
#[fail(display = "{}", msg)]
pub struct ParseGuidError {
    /// The error message.
    msg: String
}

fn parse_chunks(src: &str) -> Result<Chunks, ParseGuidError> {
    parse_only(guid_parser::chunks, src.as_bytes())
        .map_err(|(_, e)| ParseGuidError {
                    msg: e.to_string()
                })
}

/// Parse a source string as a GUID, and return the GUID as a sequence of bytes.
pub fn parse_bytes(src: &str) -> Result<[u8; 16], ParseGuidError> {
    parse_chunks(src).map(|chunks| chunks.to_bytes())
}

#[cfg(windows)]
/// Parse a source string as a GUID.
pub fn parse(src: &str) -> Result<GUID, ParseGuidError> {
    parse_chunks(src).map(|chunks| chunks.to_guid())
}

#[cfg(windows)]
#[macro_use]
extern crate proc_macro_hack;

#[cfg(windows)]
#[allow(unused_imports)]
#[macro_use]
extern crate guid_macro_impl;

#[cfg(windows)]
#[doc(hidden)]
pub use guid_macro_impl::*;

#[cfg(windows)]
proc_macro_expr_decl! {
    #[doc(hidden)]
    guid_parts! => guid_parts_impl
}

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
/// The struct representing a Windows GUID in memory.
pub use winapi::guiddef::GUID;

#[cfg(windows)]
#[macro_export]
/// A literal syntax for generating GUID values.
///
/// ```
/// # #[macro_use]
/// # extern crate guid;
/// # use guid::GUID;
/// # fn main() {
/// # let _ : GUID =
/// guid!{"6B29FC40-CA47-1067-B31D-00DD010662DA"}
/// # ;
/// # }
/// ```
macro_rules! guid {
    {$literal:expr} => {
        {
            const PARTS: (u32, u16, u16, [u8; 8]) = guid_parts! {$literal};
            $crate::GUID {
                Data1: PARTS.0,
                Data2: PARTS.1,
                Data3: PARTS.2,
                Data4: PARTS.3
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parser() {
        use parse_bytes;
        let bytes = parse_bytes("6B29FC40-CA47-1067-B31D-00DD010662DA").unwrap();
        assert_eq!(bytes, [ 0x6B, 0x29, 0xFC, 0x40, 0xCA, 0x47, 0x10, 0x67, 0xB3, 0x1D, 0x00, 0xDD, 0x01, 0x06, 0x62, 0xDA ]);
    }

    #[cfg(windows)]
    #[test]
    fn test_macro() {
        assert_eq!(guid!{"6B29FC40-CA47-1067-B31D-00DD010662DA"}.Data1, 0x6B29FC40);
        assert_eq!(guid!{"6B29FC40-CA47-1067-B31D-00DD010662DA"}.Data2, 0xCA47);
        assert_eq!(guid!{"6B29FC40-CA47-1067-B31D-00DD010662DA"}.Data3, 0x1067);
        assert_eq!(guid!{"6B29FC40-CA47-1067-B31D-00DD010662DA"}.Data4, [ 0xB3, 0x1D, 0x00, 0xDD, 0x01, 0x06, 0x62, 0xDA ]);
    }
}
