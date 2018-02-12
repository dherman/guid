//! This crate provides a [chomp](https://github.com/m4rw3r/chomp) parser for
//! Windows GUIDs, using the most customary syntax used for example by the
//! tool `guidgen.exe` that comes with Microsoft Visual Studio. For example,
//! the GUID `6B29FC40-CA47-1067-B31D-00DD010662DA` would be represented by
//! the string `"6B29FC40-CA47-1067-B31D-00DD010662DA"`.
//!
//! See the documentation for the `chunks` function below for an example.

#[macro_use]
extern crate chomp;

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
use winapi::guiddef::GUID;

use chomp::prelude::*;

/// A 48-bit unsigned integer.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct u48 {
    /// The high-order 16 bits of the integer.
    pub hi: u16,

    /// The low-order 32 bits of the integer.
    pub lo: u32
}

impl u48 {
    /// Non-lossy cast of a 48-bit unsigned integer to a 64-bit unsigned integer.
    ///
    /// ```
    /// # extern crate guid_parser;
    /// # use guid_parser::u48;
    /// # fn main() {
    /// assert_eq!(u48 { hi: 0xf00d, lo: 0xdeadbeef }.to_u64(), 0xf00ddeadbeef);
    /// # }
    /// ```
    pub fn to_u64(self) -> u64 {
        ((self.hi as u64) << 32) +
        ((self.lo as u64)      )
    }
}

/// The result of parsing the five chunks of a customary representation of a Windows GUID.
/// 
/// For the GUID `6B29FC40-CA47-1067-B31D-00DD010662DA`, the resulting `Chunks` would be:
///
/// ```
/// # extern crate guid_parser;
/// # use guid_parser::{u48, Chunks};
/// # fn main() {
/// # let _ =
/// Chunks {
///     chunk1: 0x6B29FC40,
///     chunk2: 0xCA47,
///     chunk3: 0x1067,
///     chunk4: 0xB31D,
///     chunk5: u48 {
///         hi: 0x00DD,
///         lo: 0x010662DA
///     }
/// }
/// # ;
/// # }
/// ```
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Chunks {
    pub chunk1: u32,
    pub chunk2: u16,
    pub chunk3: u16,
    pub chunk4: u16,
    pub chunk5: u48
}

impl Chunks {
    pub fn to_bytes(self) -> [u8; 16] {
        [ ((self.chunk1    & 0xFF000000) >> 24) as u8,
          ((self.chunk1    & 0x00FF0000) >> 16) as u8,
          ((self.chunk1    & 0x0000FF00) >>  8) as u8,
          ((self.chunk1    & 0x000000FF)      ) as u8,
          ((self.chunk2    &     0xFF00) >>  8) as u8,
          ((self.chunk2    &     0x00FF)      ) as u8,
          ((self.chunk3    &     0xFF00) >>  8) as u8,
          ((self.chunk3    &     0x00FF)      ) as u8,
          ((self.chunk4    &     0xFF00) >>  8) as u8,
          ((self.chunk4    &     0x00FF)      ) as u8,
          ((self.chunk5.hi &     0xFF00) >>  8) as u8,
          ((self.chunk5.hi &     0x00FF)      ) as u8,
          ((self.chunk5.lo & 0xFF000000) >> 24) as u8,
          ((self.chunk5.lo & 0x00FF0000) >> 16) as u8,
          ((self.chunk5.lo & 0x0000FF00) >>  8) as u8,
          ((self.chunk5.lo & 0x000000FF)      ) as u8 ]
    }

    pub fn to_parts(self) -> (u32, u16, u16, [u8; 8]) {
        let b = self.to_bytes();
        (self.chunk1,
         self.chunk2,
         self.chunk3,
         [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]])
    }

    #[cfg(windows)]
    pub fn to_guid(self) -> GUID {
        GUID {
            Data1: self.chunk1,
            Data2: self.chunk2,
            Data3: self.chunk3,
            Data4: [
                ((self.chunk4    &     0xFF00) >>  8) as u8,
                ((self.chunk4    &     0x00FF)      ) as u8,
                ((self.chunk5.hi &     0xFF00) >>  8) as u8,
                ((self.chunk5.hi &     0x00FF)      ) as u8,
                ((self.chunk5.lo & 0xFF000000) >> 24) as u8,
                ((self.chunk5.lo & 0x00FF0000) >> 16) as u8,
                ((self.chunk5.lo & 0x0000FF00) >>  8) as u8,
                ((self.chunk5.lo & 0x000000FF)      ) as u8
            ]
        }
    }
}

fn hex_digit<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| {
        b'0' <= c && c <= b'9' ||
        b'A' <= c && c <= b'F' ||
        b'a' <= c && c <= b'f'
    }).map(|c| {
        if b'0' <= c && c <= b'9' {
            c - b'0'
        } else if b'A' <= c && c <= b'F' {
            (c - b'A') + 10
        } else if b'a' <= c && c <= b'f' {
            (c - b'a') + 10
        } else {
            unreachable!()
        }
    })
}

fn short_chunk<I: U8Input>(i: I) -> SimpleResult<I, u16> {
    parse!{i;
        let digit1 = hex_digit();
        let digit2 = hex_digit();
        let digit3 = hex_digit();
        let digit4 = hex_digit();
        ret ((digit1 as u16) << 12) +
            ((digit2 as u16) <<  8) +
            ((digit3 as u16) <<  4) +
            ((digit4 as u16)      )
    }
}

fn medium_chunk<I: U8Input>(i: I) -> SimpleResult<I, u32> {
    parse!{i;
        let short1 = short_chunk();
        let short2 = short_chunk();
        ret ((short1 as u32) << 16) +
            ((short2 as u32)      )
    }
}

fn long_chunk<I: U8Input>(i: I) -> SimpleResult<I, u48> {
    parse!{i;
        let hi = short_chunk();
        let lo = medium_chunk();
        ret u48 { hi, lo }
    }
}

/// A [chomp](https://github.com/m4rw3r/chomp) parser for the chunks of a Windows GUID.
///
/// ```
/// # extern crate chomp;
/// # extern crate guid_parser;
/// use chomp::prelude::*;
/// use guid_parser::{chunks, Chunks, u48};
///
/// # fn main() {
/// assert_eq!(parse_only(chunks, "6B29FC40-CA47-1067-B31D-00DD010662DA".as_bytes()),
///     Ok(Chunks {
///         chunk1: 0x6B29FC40,
///         chunk2: 0xCA47,
///         chunk3: 0x1067,
///         chunk4: 0xB31D,
///         chunk5: u48 {
///             hi: 0x00DD,
///             lo: 0x010662DA
///         }
///     }));
/// # }
/// ```
pub fn chunks<I: U8Input>(i: I) -> SimpleResult<I, Chunks> {
    parse!{i;
        let chunk1 = medium_chunk();
                     string(b"-");
        let chunk2 = short_chunk();
                     string(b"-");
        let chunk3 = short_chunk();
                     string(b"-");
        let chunk4 = short_chunk();
                     string(b"-");
        let chunk5 = long_chunk();
        ret Chunks { chunk1, chunk2, chunk3, chunk4, chunk5 }
    }
}

#[cfg(test)]
mod tests {
    use chomp::prelude::*;
    use super::{hex_digit, short_chunk, medium_chunk, long_chunk, chunks, Chunks, u48};

    #[test]
    fn test_hex_digit() {
        assert_eq!(parse_only(hex_digit, b"c"), Ok(12));
        assert_eq!(parse_only(hex_digit, b"C"), Ok(12));
        assert_eq!(parse_only(hex_digit, b"9"), Ok(9));
    }

    #[test]
    fn test_short_chunk() {
        assert_eq!(parse_only(short_chunk, b"cafe"), Ok(0xcafe));
        assert_eq!(parse_only(short_chunk, b"CAFE"), Ok(0xcafe));
    }

    #[test]
    fn test_medium_chunk() {
        assert_eq!(parse_only(medium_chunk, b"cafef00d"), Ok(0xcafef00d));
        assert_eq!(parse_only(medium_chunk, b"CAFEF00D"), Ok(0xcafef00d));
    }

    #[test]
    fn test_long_chunk() {
        assert_eq!(parse_only(long_chunk, b"1234cafef00d").map(|long| long.to_u64()), Ok(0x1234cafef00d));
        assert_eq!(parse_only(long_chunk, b"1234CAFEF00D").map(|long| long.to_u64()), Ok(0x1234cafef00d));
    }

    #[test]
    fn test_chunks() {
        assert_eq!(parse_only(chunks, b"cafef00d-CAFE-f00d-BEEF-1234abcdDADA"), Ok(Chunks {
            chunk1: 0xcafef00d,
            chunk2: 0xCAFE,
            chunk3: 0xf00d,
            chunk4: 0xBEEF,
            chunk5: u48 {
                hi: 0x1234,
                lo: 0xabcdDADA
            }
        }));
    }

    #[test]
    fn test_bytes() {
        let chunks = parse_only(chunks, b"cafef00d-CAFE-f00d-BEEF-1234abcdDADA").unwrap();
        let bytes = chunks.to_bytes();
        assert_eq!( [ 0xca, 0xfe, 0xf0, 0x0d, 0xCA, 0xFE, 0xf0, 0x0d, 0xBE, 0xEF, 0x12, 0x34, 0xab, 0xcd, 0xDA, 0xDA ],
            bytes);
    }

    #[cfg(windows)]
    #[test]
    fn test_guid() {
        let chunks = parse_only(chunks, b"cafef00d-CAFE-f00d-BEEF-1234abcdDADA").unwrap();
        let guid = chunks.to_guid();
        assert_eq!(GUID {
            Data1: 0xcafef00d,
            Data2: 0xCAFE,
            Data3: 0cf00d,
            Data4: [ 0xBE, 0xEF, 0x12, 0x34, 0xab, 0xcd, 0xDA, 0xDA ]
        }, guid);
    }
}
