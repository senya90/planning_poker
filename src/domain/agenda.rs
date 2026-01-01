use serde::Serialize;
use uuid::Uuid;

use crate::{io::input::read_line, traits::EntityCollection};

#[derive(Debug, Clone, Serialize)]
pub struct Agenda {
  pub id: String,
  pub title: String,
}

impl Agenda {
  pub fn new(title: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      title,
    }
  }

  pub fn read_agendas_by_input() -> Vec<Agenda> {
    let mut agendas: Vec<Agenda> = Vec::new();

    loop {
      match read_line("Agenda (empty to finish): ") {
        Ok(title) => {
          if title.is_empty() {
            if !agendas.is_empty() {
              break;
            }

            println!("At least one agenda for discussion is needed");
            continue;
          }
          agendas.push(Agenda::new(title));
        }
        Err(error) => {
          eprintln!("{}", error);
          break;
        }
      }
    }

    agendas
  }
}

impl EntityCollection for Vec<Agenda> {
  fn get_titles(&self) -> Vec<&str> {
    self.iter().map(|a| a.title.as_str()).collect()
  }
}
