mod domain;
mod error;
mod io;

use domain::participant::Participant;
use io::input::read_line;
use std::process;

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

  let names: Vec<&str> = participants.iter().map(|p| p.name.as_str()).collect();

  print!("Let's start. Participant: {}", names.join(", "));
}
