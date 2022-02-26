use crate::{Error, Result};

#[derive(Debug, Clone)]
pub enum Auth {
    Token(String),
    None,
}

impl Auth {
    pub fn headers(&self) -> Result<(String, String)> {
        match &self {
            &Auth::Token(token) => Ok(("PRIVATE-TOKEN".to_string(), token.to_owned())),
            &Auth::None => Err(Error::Unauthorized("access token not set".to_string())),
        }
    }
}
