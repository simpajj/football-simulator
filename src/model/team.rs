use std::cmp::PartialEq;
use uuid::Uuid;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, Clone, Copy, PartialOrd)]
pub struct Team {
  pub id: Uuid,
  pub name: &'static str,
  pub rating: f64,
}

impl Display for Team {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.name())
  }
}

impl PartialEq for Team {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

impl Eq for Team {}

impl Team {
  pub fn new(name: &'static str, rating: f64) -> Team {
    Team {
      id: Uuid::new_v4(),
      name: name,
      rating: rating,
    }
  }

  pub fn name(&self) -> &'static str {
    self.name
  }

  pub fn rating(&self) -> f64 {
    self.rating
  }

  pub fn update_rating(
    mut self,
    match_importance: f64,
    match_result: f64,
    win_probability: f64,
  ) -> Self {
    self.rating = self.rating + match_importance * (match_result - win_probability).round();
    self
  }
}
