use std::{fmt, io};

#[derive(Debug)]
pub enum AppError {
  Io(io::Error),
  InvalidInput(String),
  Serialization(serde_json::Error),
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AppError::Io(e) => write!(f, "IO error: {}", e),
      AppError::InvalidInput(message) => write!(f, "Invalid input: {}", message),
      AppError::Serialization(e) => write!(f, "Serialization error: {}", e),
    }
  }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
  fn from(error: io::Error) -> Self {
    AppError::Io(error)
  }
}

impl From<serde_json::Error> for AppError {
  fn from(e: serde_json::Error) -> Self {
    AppError::Serialization(e)
  }
}
