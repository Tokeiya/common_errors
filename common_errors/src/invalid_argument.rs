use std::fmt::{Debug, Display, Formatter};
use thiserror::Error as ThisError;

pub struct Information{
	name:Option<String>,
	cause:Option<String>,
}

impl Information{
	pub fn new(name:Option<String>,cause:Option<String>) -> Self{
		Self{name,cause}
	}
	
	pub fn name(&self)->Option<&String>{
		self.name.as_ref()
	}
	pub fn cause(&self)->Option<&String>{
		self.cause.as_ref()
	}
}

impl Debug for Information{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Information {{ name: {:?}, cause: {:?} }}", self.name, self.cause)
	}
}

impl Display for Information{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match (&self.name,&self.cause) { (None, None) => {std::fmt::Result::Ok(())}
			(Some(name),None) => write!(f, "name:{}",name),
			(None,Some(cause)) => write!(f, "cause:{}",cause),
			(Some(name),Some(cause)) => write!(f, "name:{},cause:{}",name,cause),
		}
	}
}

#[derive(Debug, ThisError)]
pub enum Error{
	InvalidArgument(String),
	ArgumentOutOfRange(String),
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		let e= Error::InvalidArgument("test".to_string());
		println!("{}",&e)
	}
}