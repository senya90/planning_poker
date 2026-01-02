use std::collections::HashMap;

use crate::domain::{participant::Participant, vote::Vote};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Round {
  pub votes: HashMap<String, Vote>,
  final_vote: Option<Vote>,
  is_finish: bool,
}

impl Round {
  pub fn new() -> Self {
    Self {
      votes: HashMap::new(),
      final_vote: None,
      is_finish: false,
    }
  }

  pub fn finish(&mut self, vote: Vote) {
    self.is_finish = true;
    self.final_vote = Some(vote);
  }

  pub fn fill_votes(&mut self, participants: &[Participant]) {
    participants.iter().for_each(|p| {
      let vote = Vote::read_vote(Some(p));
      self.votes.insert(p.id.to_string(), vote);
    });
  }
}
