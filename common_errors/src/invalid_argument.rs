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
	
	pub fn new_both(name:String,cause:String)->Self{
		Self{name:Some(name),cause:Some(cause)}
	}
	pub fn new_name(name:String)->Self{
		Self{name:Some(name),cause:None}
	}
	
	pub fn new_cause(cause:String)->Self{
		Self{name:None,cause:Some(cause)}
	}
	
	pub fn name(&self)->Option<&str>{
		self.name.as_deref()
	}
	pub fn cause(&self)->Option<&str>{
		self.cause.as_deref()
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
	#[error("invalid argument:{0}")]
	InvalidArgument(Information),
	#[error("argument out of range:{0}")]
	ArgumentOutOfRange(Information),
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		let e= Error::InvalidArgument(Information::new(Some("name".to_string()),Some("cause".to_string())));
		println!("{}",&e)
	}
}