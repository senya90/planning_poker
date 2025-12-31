use crate::domain::{agenda::Agenda, participant::Participant, round::Round};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PlanningSession {
  pub participants: Vec<Participant>,
  pub agendas: Vec<Agenda>,
  pub current_round: Option<Round>,
}
