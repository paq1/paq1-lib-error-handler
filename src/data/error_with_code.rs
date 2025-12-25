use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum Error {
    Failure(ErrorWithCode),
}


#[derive(Debug, Clone)]
pub struct ErrorWithCode {
    pub code: String,
    pub status: u32,
    pub title: String,
    pub description: Option<String>,
    pub problems: Vec<Problem>,
}

impl ErrorWithCode {
    pub fn new(code: &str, status: u32, title: &str) -> Self {
        Self {
            code: code.to_string(),
            status: status,
            title: title.to_string(),
            description: None,
            problems: vec![],
        }
    }

    pub fn with_description(&self, description: &str) -> Self {
        Self {
            description: Some(description.to_string()),
            ..self.clone()
        }
    }

    pub fn with_problems(&self, problems: Vec<Problem>) -> Self {
        Self {
            problems,
            ..self.clone()
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Failure(error) => write!(f, "{:?}", error),
        }
    }
}

impl std::error::Error for Error {}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Problem {
    pub title: String,
    pub description: Option<String>,
    pub warn_message: Option<String>,
}
