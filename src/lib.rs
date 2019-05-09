#![forbid(unsafe_code, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2018_compatibility)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Examples
//! ```rust
//! # use failure::Error;
//! use http::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};
//!
//! # fn main () -> Result<(), failure::Error> {
//! let mut headers = HeaderMap::new();
//! headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("gzip, deflate, br")?);
//!
//! let encoding = accept_encoding::parse(&headers)?;
//! assert!(encoding.is_gzip());
//! # Ok(())}
//! ```
//!
//! ```rust
//! # use failure::Error;
//! use http::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};
//!
//! # fn main () -> Result<(), failure::Error> {
//! let mut headers = HeaderMap::new();
//! headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("gzip;q=0.5, deflate;q=0.9, br;q=1.0")?);
//!
//! let encoding = accept_encoding::parse(&headers)?;
//! assert!(encoding.is_brotli());
//! # Ok(())}
//! ```

mod error;

pub use crate::error::{Error, ErrorKind, Result};
use derive_is_enum_variant::is_enum_variant;
use failure::ResultExt;
use http::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};

/// Encoding levels.
#[derive(Debug, Clone, Copy, Eq, PartialEq, is_enum_variant)]
pub enum Encoding {
    /// Gzip is the most preferred encoding present.
    Gzip,
    /// Deflate is the most preferred encoding present.
    Deflate,
    /// Brotli is the most preferred encoding present.
    Brotli,
    /// No encoding is preferred.
    Identity,
    /// No preference is expressed on which encoding to use. Either the `Accept-Encoding` header is not present, or `*` is set as the most preferred encoding.
    None,
}

impl Encoding {
    /// Parses a given string into its corresponding encoding.
    fn parse(s: &str) -> Result<Encoding> {
        match s {
            "gzip" => Ok(Encoding::Gzip),
            "deflate" => Ok(Encoding::Deflate),
            "br" => Ok(Encoding::Brotli),
            "identity" => Ok(Encoding::Identity),
            "*" => Ok(Encoding::None),
            _ => Err(ErrorKind::UnknownEncoding)?,
        }
    }

    /// Converts the encoding into its' corresponding header value.
    ///
    /// Note that [`Encoding::None`] will return a HeaderValue with the content `*`.
    /// This is likely not what you want if you are using this to generate the `Content-Encoding` header to be included in an encoded response.
    pub fn to_header_value(&self) -> HeaderValue {
        match *self {
            Encoding::Gzip => HeaderValue::from_str("gzip").unwrap(),
            Encoding::Deflate => HeaderValue::from_str("deflate").unwrap(),
            Encoding::Brotli => HeaderValue::from_str("br").unwrap(),
            Encoding::Identity => HeaderValue::from_str("identity").unwrap(),
            Encoding::None => HeaderValue::from_str("*").unwrap(),
        }
    }
}

/// Parse a set of HTTP headers into an `Encoding`.
pub fn parse(headers: &HeaderMap) -> Result<Encoding> {
    let mut preferred_encoding = Encoding::None;
    let mut max_qval = 0.0;

    for header_value in headers.get_all(ACCEPT_ENCODING).iter() {
        let header_value = header_value.to_str().context(ErrorKind::InvalidEncoding)?;
        for v in header_value.split(',').map(str::trim) {
            let v: Vec<&str> = v.splitn(2, ";q=").collect();
            let encoding = v[0];

            match Encoding::parse(encoding) {
                Ok(encoding) => {
                    if v.len() > 1 {
                        let qval = match v[1].parse::<f32>() {
                            Ok(f) => f,
                            Err(_) => return Err(ErrorKind::InvalidEncoding)?,
                        };
                        if (qval - 1.0f32).abs() < 0.01 {
                            preferred_encoding = encoding;
                            break;
                        } else if qval > 1.0 {
                            return Err(ErrorKind::InvalidEncoding)?; // q-values over 1 are unacceptable
                        } else if qval > max_qval {
                            preferred_encoding = encoding;
                            max_qval = qval;
                        }
                    } else {
                        preferred_encoding = encoding;
                        break;
                    }
                }
                Err(_) => continue, // ignore unknown encodings for now
            }
        }
    }

    Ok(preferred_encoding)
}
