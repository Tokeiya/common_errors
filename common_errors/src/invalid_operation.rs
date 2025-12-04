use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
#[error("invalid operation:{0}")]
pub struct Error(String);


impl Error {
	pub fn description(&self)->&str{
		&self.0
	}
}

impl From<String> for Error {
	fn from(value: String) -> Self {
		Self(value)
	}
}