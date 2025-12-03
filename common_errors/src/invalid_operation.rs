use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
#[error("invalid operation:{0}")]
pub struct Error(String);

