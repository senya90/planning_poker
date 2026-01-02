use core::fmt;

use serde::Serialize;

use crate::{domain::participant::Participant, error::AppError, io::input::read_line};

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum Vote {
  One,
  Two,
  Three,
  Five,
  Eight,
  Thirteen,
  TwentyOne,
}

impl Vote {
  pub fn read_vote(participant: Option<&Participant>) -> Vote {
    loop {
      let prompt = match participant {
        Some(p) => &format!("Vote for {} (1/2/3/5/8/13/21) ", p.name),
        None => &"Vote (1/2/3/5/8/13/21) ".to_string(),
      };

      let raw_vote = read_line(prompt);

      let raw_vote = match raw_vote {
        Ok(value) => value,
        Err(_) => {
          eprintln!("⚠︎ Error reading. Please try again.");
          continue;
        }
      };

      let vote = Vote::parse(&raw_vote);
      match vote {
        Some(value) => break value,
        None => {
          let error = AppError::InvalidInput("⚠︎ Invalid value. Please try again.".to_string());
          eprintln!("{}", error);
          continue;
        }
      };
    }
  }

  fn parse(value: &str) -> Option<Self> {
    match value {
      "1" => Some(Vote::One),
      "2" => Some(Vote::Two),
      "3" => Some(Vote::Three),
      "5" => Some(Vote::Five),
      "8" => Some(Vote::Eight),
      "13" => Some(Vote::Thirteen),
      "21" => Some(Vote::TwentyOne),
      _ => None,
    }
  }
}

impl fmt::Display for Vote {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Vote::One => write!(f, "1"),
      Vote::Two => write!(f, "2"),
      Vote::Three => write!(f, "3"),
      Vote::Five => write!(f, "5"),
      Vote::Eight => write!(f, "8"),
      Vote::Thirteen => write!(f, "13"),
      Vote::TwentyOne => write!(f, "21"),
    }
  }
}
