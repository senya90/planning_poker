use serde::Serialize;
use uuid::Uuid;

use crate::{io::input::read_line, traits::EntityCollection};

#[derive(Debug, Clone, Serialize)]
pub struct Participant {
  pub id: String,
  pub name: String,
}

impl Participant {
  pub fn new(name: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      name,
    }
  }

  pub fn read_participants_by_input() -> Vec<Participant> {
    let mut participants: Vec<Participant> = Vec::new();

    loop {
      let name = read_line("Participant name (empty to finish): ");

      match name {
        Ok(name) => {
          if name.is_empty() {
            if participants.len() >= 2 {
              break;
            }

            println!("We need more participants! Add more");
            continue;
          }

          participants.push(Participant::new(name))
        }
        Err(error) => {
          eprintln!("{}", error);
          break;
        }
      }
    }

    participants
  }
}

impl EntityCollection for Vec<Participant> {
  fn get_titles(&self) -> Vec<&str> {
    self.iter().map(|a| a.name.as_str()).collect()
  }
}
