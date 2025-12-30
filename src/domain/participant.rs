use serde::Serialize;
use uuid::Uuid;

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
}
