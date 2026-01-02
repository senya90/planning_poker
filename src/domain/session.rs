use std::fmt::Display;

use crate::{
  domain::{participant::Participant, round::Round, task::Task, vote::Vote},
  io::input::{YesNo, read_line, read_yes_no},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PlanningSession {
  pub participants: Vec<Participant>,
  pub tasks: Vec<Task>,
}

impl PlanningSession {
  pub fn new(participants: Vec<Participant>, tasks: Vec<Task>) -> Self {
    Self {
      participants,
      tasks,
    }
  }

  pub fn play(&mut self) {
    self.tasks.iter_mut().for_each(|task| {
      loop {
        let mut round = Round::new();
        round.fill_votes(&self.participants);

        let is_success = read_yes_no("Is the round successful? (y/N): ");

        match is_success {
          YesNo::Yes => {
            let final_res = Vote::read_vote(None);
            round.finish(final_res);
          }
          YesNo::No => {
            continue;
          }
        }
      }
    });
  }
}
