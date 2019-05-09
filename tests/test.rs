extern crate accept_encoding;
extern crate failure;

use failure::Error;
use http::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};

#[test]
fn single_encoding() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("gzip")?);

    let encoding = accept_encoding::parse(&headers)?;
    assert!(encoding.is_gzip());

    Ok(())
}

#[test]
fn multiple_encodings() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("gzip, deflate, br")?);

    let encoding = accept_encoding::parse(&headers)?;
    assert!(encoding.is_gzip());

    Ok(())
}

#[test]
fn single_encoding_with_qval() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("deflate;q=1.0")?);

    let encoding = accept_encoding::parse(&headers)?;
    assert!(encoding.is_deflate());

    Ok(())
}

#[test]
fn multiple_encodings_with_qval_1() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_str("deflate, gzip;q=1.0, *;q=0.5")?,
    );

    let encoding = accept_encoding::parse(&headers)?;
    assert!(encoding.is_deflate());

    Ok(())
}

#[test]
fn multiple_encodings_with_qval_2() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_str("gzip;q=0.5, deflate;q=1.0, *;q=0.5")?,
    );

    let encoding = accept_encoding::parse(&headers)?;
    assert!(encoding.is_deflate());

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
