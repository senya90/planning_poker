mod domain;
mod error;
mod io;
mod traits;

use domain::participant::Participant;
use domain::task::Task;
use std::process;
use traits::EntityCollection;

use crate::domain::round::Round;
use crate::domain::session::PlanningSession;
use crate::io::output::print_round;

fn main() {
  println!("Planning Poker");

  let participants: Vec<Participant> = Participant::read_participants_by_input();

  if participants.len() < 2 {
    eprintln!("Not enough participants. GAME OVER");
    process::exit(1);
  }

  println!("Participant: {} \r\n", participants.get_titles_as_string());
  println!("Let's choose tasks");

  let tasks: Vec<Task> = Task::read_tasks_by_input();

  if tasks.is_empty() {
    eprintln!("There are no tasks. GAME OVER");
    process::exit(1);
  }

  println!("Tasks: {} \r\n", tasks.get_titles_as_string());

  let mut session = PlanningSession::new(participants, tasks);

  session.play();

  let mut round = Round::new();
  round.finish();
  print_round(&round);
}
