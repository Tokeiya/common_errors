use thiserror::Error as ThisError;


#[derive(Debug, ThisError)]
#[error("index out of range:{0}")]
pub struct  Error(String);


impl From<String> for Error {
	fn from(value: String) -> Self {
		Self(value)
	}
}

impl From<&str> for Error {
	fn from(value: &str) -> Self {
		Self(value.to_string())
	}
}
