# accept-encoding
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Determine the best encoding possible from an Accept-Encoding HTTP header.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
__Basic usage__
```rust
use failure::Error;
use http::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};

fn main () -> Result<(), failure::Error> {
  let mut headers = HeaderMap::new();
  headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("gzip, deflate, br")?);

  let encoding = accept_encoding::parse(&headers)?;
  assert!(encoding.is_brotli());
  Ok(())
}
```

## Installation
```sh
$ cargo add accept-encoding
```

## Safety
This crate uses `#![deny(unsafe_code)]` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
None.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/accept-encoding.svg?style=flat-square
[2]: https://crates.io/crates/accept-encoding
[3]: https://img.shields.io/travis/rust-net-web/accept-encoding/master.svg?style=flat-square
[4]: https://travis-ci.org/rust-net-web/accept-encoding
[5]: https://img.shields.io/crates/d/accept-encoding.svg?style=flat-square
[6]: https://crates.io/crates/accept-encoding
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/accept-encoding

[releases]: https://github.com/rust-net-web/accept-encoding/releases
[contributing]: https://github.com/rust-net-web/accept-encoding/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/rust-net-web/accept-encoding/labels/good%20first%20issue
[help-wanted]: https://github.com/rust-net-web/accept-encoding/labels/help%20wanted
