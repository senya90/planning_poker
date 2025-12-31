use std::collections::HashMap;

use crate::domain::vote::Vote;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Round {
  pub votes: HashMap<String, Vote>,
  pub revealed: bool,
}

impl Round {
  pub fn new() -> Self {
    Self {
      votes: HashMap::new(),
      revealed: false,
    }
  }

  pub fn cast_vote(&mut self, participant_id: &str, vote: Vote) {
    self.votes.insert(participant_id.to_string(), vote);
  }

  pub fn reveal(&mut self) {
    self.revealed = true;
  }
}
