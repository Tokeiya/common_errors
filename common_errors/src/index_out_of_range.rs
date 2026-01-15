use thiserror::Error as ThisError;


#[derive(Debug, ThisError)]
#[error("index out of range:{0}")]
pub struct  IndexOutOfRangeError(String);


impl From<String> for IndexOutOfRangeError {
	fn from(value: String) -> Self {
		Self(value)
	}
}

impl From<&str> for IndexOutOfRangeError {
	fn from(value: &str) -> Self {
		Self(value.to_string())
	}
}
