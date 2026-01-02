use serde::Serialize;
use uuid::Uuid;

use crate::{domain::round::Round, io::input::read_line, traits::EntityCollection};

#[derive(Debug, Clone, Serialize)]
pub struct Task {
  pub title: String,
  id: String,
  rounds: Vec<Round>,
}

impl Task {
  pub fn new(title: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      title,
      rounds: Vec::new(),
    }
  }

  pub fn read_tasks_by_input() -> Vec<Task> {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
      match read_line("Task (empty to finish): ") {
        Ok(title) => {
          if title.is_empty() {
            if !tasks.is_empty() {
              break;
            }

            println!("At least one task for discussion is needed");
            continue;
          }
          tasks.push(Task::new(title));
        }
        Err(error) => {
          eprintln!("{}", error);
          break;
        }
      }
    }

    tasks
  }

  pub fn add_round(&mut self, round: Round) {
    self.rounds.push(round);
  }

  pub fn get_ended_rounds_count(&self) -> usize {
    self.rounds.len()
  }
}

impl EntityCollection for Vec<Task> {
  fn get_titles(&self) -> Vec<&str> {
    self.iter().map(|a| a.title.as_str()).collect()
  }
}
