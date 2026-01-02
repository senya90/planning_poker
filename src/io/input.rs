use crate::error::AppError;
use std::{
  fmt,
  io::{self, Write},
};

#[derive(Debug)]
pub enum YesNo {
  Yes,
  No,
}

impl fmt::Display for YesNo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    match self {
      YesNo::Yes => write!(f, "Yes"),
      YesNo::No => write!(f, "No"),
    }
  }
}

pub fn read_line(prompt: &str) -> Result<String, AppError> {
  print!("{}", prompt);
  io::stdout().flush()?;

  let mut input = String::new();
  io::stdin().read_line(&mut input)?;

  Ok(input.trim().to_owned())
}

pub fn read_yes_no(prompt: &str) -> YesNo {
  loop {
    let is_success = read_line(prompt);

    let is_success = match is_success {
      Ok(value) => match value.as_str() {
        "y" | "Y" => Some(YesNo::Yes),
        "N" => Some(YesNo::No),
        _ => None,
      },
      Err(_) => None,
    };

    match is_success {
      Some(value) => break value,
      None => {
        eprintln!("Invalid value. Please repeat.");
        continue;
      }
    }
  }
}
