use crate::domain::{participant::Participant, round::Round, task::Task};
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
    self.tasks.iter_mut().for_each(|_task| {
      let mut round = Round::new();
      round.collect_votes(&self.participants);
    });
  }
}
