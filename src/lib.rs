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
    /// Gzip is the most preferred encoding.
    Gzip,
    /// Deflate is the most preferred encoding.
    Deflate,
    /// Brotli is the most preferred encoding.
    Brotli,
    /// Zstd is the most preferred encoding.
    Zstd,
    /// No encoding is preferred.
    Identity,
    /// No preference is expressed on which encoding to use. Either the `Accept-Encoding` header is not present, or `*` is set as the most preferred encoding.
    Any,
}

impl Encoding {
    /// Parses a given string into its corresponding encoding.
    fn parse(s: &str) -> Result<Encoding> {
        match s {
            "gzip" => Ok(Encoding::Gzip),
            "deflate" => Ok(Encoding::Deflate),
            "br" => Ok(Encoding::Brotli),
            "zstd" => Ok(Encoding::Zstd),
            "identity" => Ok(Encoding::Identity),
            "*" => Ok(Encoding::Any),
            _ => Err(ErrorKind::UnknownEncoding)?,
        }
    }

    /// Converts the encoding into its' corresponding header value.
    ///
    /// Note that [`Encoding::Any`] will return a HeaderValue with the content `*`.
    /// This is likely not what you want if you are using this to generate the `Content-Encoding` header to be included in an encoded response.
    pub fn to_header_value(self) -> HeaderValue {
        match self {
            Encoding::Gzip => HeaderValue::from_str("gzip").unwrap(),
            Encoding::Deflate => HeaderValue::from_str("deflate").unwrap(),
            Encoding::Brotli => HeaderValue::from_str("br").unwrap(),
            Encoding::Zstd => HeaderValue::from_str("zstd").unwrap(),
            Encoding::Identity => HeaderValue::from_str("identity").unwrap(),
            Encoding::Any => HeaderValue::from_str("*").unwrap(),
        }
    }
}

/// Parse a set of HTTP headers into a single `Encoding` that the client prefers.
///
/// If you're looking for an easy way to determine the best encoding for the client and support every [`Encoding`] listed, this is likely what you want.
pub fn parse(headers: &HeaderMap) -> Result<Encoding> {
    let mut preferred_encoding = Encoding::Any;
    let mut max_qval = 0.0;

    for (encoding, qval) in encodings(headers)? {
        if (qval - 1.0f32).abs() < 0.01 {
            preferred_encoding = encoding;
            break;
        } else if qval > max_qval {
            preferred_encoding = encoding;
            max_qval = qval;
        }
    }

    Ok(preferred_encoding)
}

/// Parse a set of HTTP headers into a vector containing tuples of encodings and their corresponding q-values.
///
/// If you're looking for more fine-grained control over what encoding to choose for the client, or if you don't support every [`Encoding`] listed, this is likely what you want.
/// ## Example
/// ```rust
/// # use failure::Error;
/// use accept_encoding::Encoding;
/// use http::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};
///
/// # fn main () -> Result<(), failure::Error> {
/// let mut headers = HeaderMap::new();
/// headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("zstd;q=1.0, deflate;q=0.8, br;q=0.9")?);
///
/// let encodings = accept_encoding::encodings(&headers)?;
/// for (encoding, qval) in encodings {
///     println!("{:?} {}", encoding, qval);
/// }
/// # Ok(())}
/// ```
pub fn encodings(headers: &HeaderMap) -> Result<Vec<(Encoding, f32)>> {
    headers
        .get_all(ACCEPT_ENCODING)
        .iter()
        .map(|hval| {
            hval.to_str()
                .context(ErrorKind::InvalidEncoding)
                .map_err(std::convert::Into::into)
        })
        .collect::<Result<Vec<&str>>>()?
        .iter()
        .flat_map(|s| s.split(',').map(str::trim))
        .filter_map(|v| {
            let mut v = v.splitn(2, ";q=");
            let encoding = match Encoding::parse(v.next().unwrap()) {
                Ok(encoding) => encoding,
                Err(_) => return None, // ignore unknown encodings
            };
            let qval = if let Some(qval) = v.next() {
                let qval = match qval.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Some(Err(ErrorKind::InvalidEncoding)),
                };
                if qval > 1.0 {
                    return Some(Err(ErrorKind::InvalidEncoding)); // q-values over 1 are unacceptable
                }
                qval
            } else {
                1.0f32
            };
            Some(Ok((encoding, qval)))
        })
        .map(|v| v.map_err(std::convert::Into::into))
        .collect::<Result<Vec<(Encoding, f32)>>>()
}
