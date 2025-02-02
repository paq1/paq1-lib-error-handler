use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum Error {
    ErrorWithCode(ErrorWithCode),
}


#[derive(Debug, Clone)]
pub struct ErrorWithCode {
    pub code: String,
    pub status: u32,
    pub title: String,
    pub description: Option<String>,
    pub problems: Vec<Problem>,
}

pub struct ErrorWithCodeBuilder {
    pub code: String,
    pub status: u32,
    pub title: String,
    pub description: Option<String>,
    pub problems: Vec<Problem>,
}

impl ErrorWithCodeBuilder {
    pub fn new(code: &str, status: u32, title: &str) -> Self {
        ErrorWithCodeBuilder {
            code: code.to_string(),
            status: status,
            title: title.to_string(),
            description: None,
            problems: vec![],
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_problems(mut self, problems: Vec<Problem>) -> Self {
        self.problems = problems;
        self
    }

    pub fn build(self) -> Error {
        Error::ErrorWithCode(ErrorWithCode {
            code: self.code,
            status: self.status,
            title: self.title,
            description: self.description,
            problems: self.problems,
        })
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ErrorWithCode(error) => write!(f, "{:?}", error),
        }
    }
}

impl std::error::Error for Error {}


#[derive(Debug, Clone)]
pub struct Problem {
    pub title: String,
    pub description: Option<String>,
    pub warn_message: Option<String>,
}
