use crate::error::AppError;
use std::io::{self, Write};

pub fn read_line(prompt: &str) -> Result<String, AppError> {
  print!("{}", prompt);
  io::stdout().flush()?;

  let mut input = String::new();
  io::stdin().read_line(&mut input)?;

  Ok(input.trim().to_owned())
}
