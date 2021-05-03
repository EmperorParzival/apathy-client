use hyper::{
	header::ToStrError,
	http::{uri::InvalidUri, Error},
};
use std::{
	fmt::{self, Display, Formatter},
	string::FromUtf8Error,
    result
};

pub type Result<T> = result::Result<T, ApathyError>;

#[derive(Debug)]
pub enum ApathyError {
	PopcornError(String),
	HyperHTTPError(Error),
	InvalidUri(InvalidUri),
	RequestError(hyper::Error),
	HeaderConversionError(ToStrError),
	DeserializeError(serde_json::Error),
	StringConversionError(FromUtf8Error),
}

impl Display for ApathyError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.write_str(&format!("{:?}", &self))
	}
}

impl From<String> for ApathyError {
	fn from(err: String) -> Self {
		ApathyError::PopcornError(err)
	}
}

impl From<Error> for ApathyError {
	fn from(err: Error) -> Self {
		ApathyError::HyperHTTPError(err)
	}
}

impl From<InvalidUri> for ApathyError {
	fn from(err: InvalidUri) -> Self {
		ApathyError::InvalidUri(err)
	}
}

impl From<hyper::Error> for ApathyError {
	fn from(err: hyper::Error) -> Self {
		ApathyError::RequestError(err)
	}
}

impl From<ToStrError> for ApathyError {
	fn from(err: ToStrError) -> Self {
		ApathyError::HeaderConversionError(err)
	}
}

impl From<serde_json::Error> for ApathyError {
	fn from(err: serde_json::Error) -> Self {
		ApathyError::DeserializeError(err)
	}
}

impl From<FromUtf8Error> for ApathyError {
	fn from(err: FromUtf8Error) -> Self {
		ApathyError::StringConversionError(err)
	}
}
