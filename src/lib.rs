#![forbid(unsafe_code, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2018_compatibility)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! # use failure::Error;
//! use http::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};
//!
//! # fn main () -> Result<(), failure::Error> {
//! let mut headers = HeaderMap::new();
//! headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("gzip, deflate, br")?);
//!
//! let encoding = accept_encoding::parse(&headers)?;
//! assert!(encoding.is_brotli());
//! # Ok(())}
//! ```

mod error;

use derive_is_enum_variant::is_enum_variant;
use http::header::{ACCEPT_ENCODING, HeaderMap};
use failure::{ResultExt};
pub use crate::error::{Error, Result, ErrorKind};

/// Encoding levels.
#[derive(Debug, Clone, is_enum_variant)]
pub enum Encoding {
  /// Gzip is the best encoding present.
  Gzip,
  /// Deflate is the best encoding present.
  Deflate,
  /// Brotli is the best encoding is present.
  Brotli,
  /// No encoding is present.
  None,
}

/// Parse a set of HTTP headers into an `Encoding`.
pub fn parse(headers: &HeaderMap) -> Result<Encoding> {
  let header = match headers.get(ACCEPT_ENCODING) {
    Some(header) => header,
    None => return Ok(Encoding::None),
  };

  let string = header.to_str().context(ErrorKind::InvalidEncoding)?;

  if string.contains("br") {
    Ok(Encoding::Brotli)
  } else if string.contains("deflate") {
    Ok(Encoding::Deflate)
  } else if string.contains("gzip") {
    Ok(Encoding::Gzip)
  } else {
    Err(ErrorKind::UnknownEncoding)?
  }
}
