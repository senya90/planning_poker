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

  if participants.len() < 2 {
    eprintln!("Not enough participants. GAME OVER");
    process::exit(1);
  }

  println!("Participant: {} \r\n", participants.get_titles_as_string());
  println!("Let's choose agendas");

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
    let raw = read_line(&format!("Vote for {} (1/2/3/5/8/13/21) ", p.name));

    let raw = match raw {
      Ok(value) => value,
      Err(error) => {
        eprintln!("Error reading input, {}", error);
        process::exit(1);
      }
    };

    let vote = Vote::parse(&raw).ok_or_else(|| AppError::InvalidInput("Unknown vote".into()));

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
