use std::collections::HashMap;

use crate::domain::vote::Vote;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Round {
  pub votes: HashMap<String, Vote>,
  is_finish: bool,
}

impl Round {
  pub fn new() -> Self {
    Self {
      votes: HashMap::new(),
      is_finish: false,
    }
  }

  pub fn cast_vote(&mut self, participant_id: &str, vote: Vote) {
    self.votes.insert(participant_id.to_string(), vote);
  }

  pub fn finish(&mut self) {
    self.is_finish = true;
  }
}
