use crate::domain::round::Round;

pub fn print_round(round: &Round) {
  println!("--- Voting results ---");
  for (participant_id, vote) in &round.votes {
    println!("Participant {} -> {:?}", participant_id, vote);
  }
}
