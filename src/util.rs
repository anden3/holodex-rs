use std::io::Read;

use serde::Deserialize;

use crate::errors::{ParseError, ServerError, ValidationError};

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

fn into_bytes(response: ureq::Response) -> Result<Vec<u8>, ParseError> {
    let len = response
        .header("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .ok_or(ParseError::MissingHeader("Content-Length"))?;

    let mut bytes: Vec<u8> = Vec::with_capacity(len);

    match response.into_reader().read_to_end(&mut bytes) {
        Ok(_) => Ok(bytes),
        Err(e) => Err(ParseError::ResponseDecodeError(e)),
    }
}

pub fn validate_response<T>(response: ureq::Response) -> Result<T, ValidationError>
where
    T: for<'de> Deserialize<'de> + std::fmt::Debug,
{
    if let status @ (400..=499 | 500..=599) = response.status() {
        let bytes = into_bytes(response).map_err(|e| {
            ValidationError::ServerError(ServerError::ErrorCodeWithValueParseError(status, e))
        })?;

        Err(match validate_json_bytes::<T>(&bytes) {
            Ok(val) => ServerError::ErrorCodeWithValue(status, format!("{:?}", val)).into(),
            Err(error) => ServerError::ErrorCodeWithValueParseError(status, error).into(),
        })
    } else {
        let bytes = into_bytes(response).map_err(ValidationError::ParseError)?;
        validate_json_bytes(&bytes).map_err(std::convert::Into::into)
    }
}

pub fn validate_json_bytes<T>(bytes: &[u8]) -> Result<T, ParseError>
where
    T: for<'de> Deserialize<'de> + std::fmt::Debug,
{
    let data: Result<T, _> = serde_json::from_slice(bytes);

    match data {
        Ok(data) => Ok(data),
        Err(e) => Err(match serde_json::from_slice::<serde_json::Value>(bytes) {
            Ok(v) => ParseError::ResponseParseError(e, v),
            Err(e) => match std::str::from_utf8(bytes) {
                Ok(s) => ParseError::ResponseJsonParseError(e, s.to_owned()),
                Err(e) => ParseError::ResponseUtf8Error(e),
            },
        }),
    }
}
