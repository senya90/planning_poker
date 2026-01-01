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
  pub fn parse(value: &str) -> Option<Self> {
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

  pub fn read_vote(participant: &Participant) -> Result<Vote, AppError> {
    let raw_vote = read_line(&format!("Vote for {} (1/2/3/5/8/13/21) ", participant.name))?;

    let vote =
      Vote::parse(&raw_vote).ok_or_else(|| AppError::InvalidInput("Unknown vote".into()))?;

    Ok(vote)
  }
}
