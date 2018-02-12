# GUID

This crate provides a `guid!` macro for expressing
[`GUID`](https://docs.rs/winapi/0.3.4/x86_64-pc-windows-msvc/winapi/shared/guiddef/struct.GUID.html)
structs with a convenient literal syntax. A GUID is a Windows
[_globally unique identifier_](https://msdn.microsoft.com/en-us/library/windows/desktop/aa368767(v=vs.85).aspx),
usually expressed in the following format:

```text
{6B29FC40-CA47-1067-B31D-00DD010662DA}
```

With this crate, a GUID can be generated with the syntax:

```rust
guid!{"6B29FC40-CA47-1067-B31D-00DD010662DA"}
```

# Example

```rust
#[macro_use]
extern crate guid;

use guid::GUID;

const MY_GUID: GUID = guid!{"6B29FC40-CA47-1067-B31D-00DD010662DA"};

fn main() {
    assert_eq!(MY_GUID.Data1, 0x6B29FC40);
    assert_eq!(MY_GUID.Data2, 0xCA47);
    assert_eq!(MY_GUID.Data3, 0x1067);
    assert_eq!(MY_GUID.Data4, [ 0xB3, 0x1D, 0x00, 0xDD, 0x01, 0x06, 0x62, 0xDA ]);
}
```

The `GUID` type is re-exported from the [winapi](https://crates.io/crates/winapi) crate,
and is only available in Windows. The `guid!` macro is also only available in Windows.

This crate also provides a parser, which can be used to parse GUID strings at runtime.
The parser is only available to generate an array of bytes on non-Windows platforms.
In Windows, this crate defines a parser that produces a `GUID` struct.

# Compatibility

This crate supports all versions of Rust (stable and nightly) starting with Rust 1.15.

# License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
