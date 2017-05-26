use std::ascii::AsciiExt;
use std::ops::Deref;

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

#[derive(Debug)]
pub struct IdError(&'static str);

impl Id {
    pub fn new(id: &str) -> Result<Id, IdError> {
        if id.len() != 64 {
            return Err(IdError("Wrong length for Id"));
        }
        if !id.chars().all(|c| char::is_ascii_alphanumeric(&c)) {
            return Err(IdError("Characters in Id must be ASCII alphanumeric"));
        }
        Ok(Id(id.to_owned()))
    }
}
