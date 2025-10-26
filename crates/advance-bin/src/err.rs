use std::fmt::Display;

#[derive(Debug)]
pub enum Kind {
    NotFound,
    AlreadyExists,
    Database,
    Config,
}

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub kind: Kind,
}

impl AppError {
    pub fn new(message: String, cause: Option<Box<dyn std::error::Error>>, kind: Kind) -> Self {
        Self { message, cause, kind }
    }

    pub fn with_cause(cause: Box<dyn std::error::Error>, kind: Kind) -> Self {
        Self::new(cause.to_string(), Some(cause), kind)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value:sqlx::Error) -> Self {
        Self::with_cause(Box::new(value),Kind::Database)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}