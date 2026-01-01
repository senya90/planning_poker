mod domain;
mod error;
mod io;
mod traits;

use domain::agenda::Agenda;
use domain::participant::Participant;
use io::input::read_line;
use std::process;
use traits::EntityCollection;

use crate::domain::round::Round;
use crate::domain::session::PlanningSession;
use crate::domain::vote::Vote;
use crate::error::AppError;
use crate::io::output::print_round;

fn main() {
  println!("Planning Poker");

  let participants: Vec<Participant> = Participant::read_participants_by_input();

  if participants.len() < 2 {
    eprintln!("Not enough participants. GAME OVER");
    process::exit(1);
  }

  println!("Participant: {} \r\n", participants.get_titles_as_string());
  println!("Let's choose agendas");

  let agendas: Vec<Agenda> = Agenda::read_agendas_by_input();

  if agendas.is_empty() {
    eprintln!("There are no agendas. GAME OVER");
    process::exit(1);
  }

  println!("Agendas: {} \r\n", agendas.get_titles_as_string());

  let session = PlanningSession {
    participants,
    agendas,
    current_round: None,
  };

  let mut round = Round::new();

  for p in &session.participants {
    let raw_vote = read_line(&format!("Vote for {} (1/2/3/5/8/13/21) ", p.name));

    let raw_vote = match raw_vote {
      Ok(value) => value,
      Err(error) => {
        eprintln!("Error reading input, {}", error);
        process::exit(1);
      }
    };

    let vote = Vote::parse(&raw_vote).ok_or_else(|| AppError::InvalidInput("Unknown vote".into()));

    let vote = match vote {
      Ok(vote) => vote,
      Err(error) => {
        eprintln!("Parsing error, {}", error);
        process::exit(1);
      }
    };

    round.cast_vote(&p.id, vote);
  }

  round.reveal();
  print_round(&round);
}
