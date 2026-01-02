use serde::Serialize;

use crate::{domain::participant::Participant, error::AppError, io::input::read_line};

#[derive(Debug, Clone, Copy, Serialize)]
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
  pub fn read_vote(participant: &Participant) -> Vote {
    loop {
      let raw_vote = read_line(&format!("Vote for {} (1/2/3/5/8/13/21) ", participant.name));

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
