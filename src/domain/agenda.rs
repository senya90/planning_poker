use serde::Serialize;
use uuid::Uuid;

use crate::traits::EntityCollection;

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
}

impl EntityCollection for Vec<Agenda> {
  fn get_titles(&self) -> Vec<&str> {
    self.iter().map(|a| a.title.as_str()).collect()
  }
}
