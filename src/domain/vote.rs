use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Vote {
  One,
  Two,
  Three,
  Five,
  Eight,
  Thirteen,
  TwentyOne,
}

impl Vote {
  pub fn parse(value: &str) -> Option<Self> {
    match value {
      "1" => Some(Vote::One),
      "2" => Some(Vote::Two),
      "3" => Some(Vote::Three),
      "5" => Some(Vote::Five),
      "8" => Some(Vote::Eight),
      "13" => Some(Vote::Thirteen),
      "21" => Some(Vote::TwentyOne),
      _ => None,
    }
  }
}
