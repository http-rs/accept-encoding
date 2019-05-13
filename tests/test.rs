extern crate accept_encoding;
extern crate failure;

use accept_encoding::Encoding;
use failure::Error;
use http::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};

#[test]
fn single_encoding() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("gzip")?);

    let encoding = accept_encoding::parse(&headers)?.unwrap();
    assert_eq!(encoding, Encoding::Gzip);

    Ok(())
}

#[test]
fn multiple_encodings() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("gzip, deflate, br")?);

    let encoding = accept_encoding::parse(&headers)?.unwrap();
    assert_eq!(encoding, Encoding::Gzip);

    Ok(())
}

#[test]
fn single_encoding_with_qval() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("deflate;q=1.0")?);

    let encoding = accept_encoding::parse(&headers)?.unwrap();
    assert_eq!(encoding, Encoding::Deflate);

    Ok(())
}

#[test]
fn multiple_encodings_with_qval_1() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_str("deflate, gzip;q=1.0, *;q=0.5")?,
    );

    let encoding = accept_encoding::parse(&headers)?.unwrap();
    assert_eq!(encoding, Encoding::Deflate);

    Ok(())
}

#[test]
fn multiple_encodings_with_qval_2() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_str("gzip;q=0.5, deflate;q=1.0, *;q=0.5")?,
    );

    let encoding = accept_encoding::parse(&headers)?.unwrap();
    assert_eq!(encoding, Encoding::Deflate);

    Ok(())
}

#[test]
fn multiple_encodings_with_qval_3() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_str("gzip;q=0.5, deflate;q=0.75, *;q=1.0")?,
    );

    let encoding = accept_encoding::parse(&headers)?;
    assert!(encoding.is_none());

    Ok(())
}

#[test]
fn list_encodings() -> Result<(), Error> {
    use accept_encoding::Encoding;

    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_str("zstd;q=1.0, deflate;q=0.8, br;q=0.9")?,
    );

    let encodings = accept_encoding::encodings(&headers)?;
    assert_eq!(encodings[0], (Some(Encoding::Zstd), 1.0));
    assert_eq!(encodings[1], (Some(Encoding::Deflate), 0.8));
    assert_eq!(encodings[2], (Some(Encoding::Brotli), 0.9));
    Ok(())
}

#[test]
fn list_encodings_ignore_unknown() -> Result<(), Error> {
    use accept_encoding::Encoding;

    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_str("zstd;q=1.0, unknown;q=0.8, br;q=0.9")?,
    );

    let encodings = accept_encoding::encodings(&headers)?;
    assert_eq!(encodings[0], (Some(Encoding::Zstd), 1.0));
    assert_eq!(encodings[1], (Some(Encoding::Brotli), 0.9));
    Ok(())
}
