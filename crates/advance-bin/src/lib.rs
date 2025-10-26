pub mod err;
pub mod config;

use err::AppError;

pub type Result<T> = std::result::Result<T,AppError>;