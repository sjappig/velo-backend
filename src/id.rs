use std::ascii::AsciiExt;
use std::cmp::PartialEq;
use std::ops::Deref;
use error::{Error, ErrorKind};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Id(String);
impl Into<String> for Id {
    fn into(self) -> String {
        self.0
    }
}
impl Deref for Id {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> PartialEq<&'a str> for Id {
    fn eq(&self, rhs: &&'a str) -> bool {
        &self.0[..] == *rhs
    }
}

impl Id {
    pub fn new(id: &str) -> Result<Id, Error> {
        if id.len() != 64 {
            return Err(Error::from_kind(
                ErrorKind::IdError("Wrong length for Id".into()),
            ));
        }
        if !id.chars().all(|c| char::is_ascii_alphanumeric(&c)) {
            return Err(Error::from_kind(ErrorKind::IdError(
                "Characters in Id must be ASCII alphanumeric".into(),
            )));
        }
        Ok(Id(id.to_owned()))
    }
}
