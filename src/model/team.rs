#[derive(Debug)]
pub struct Team {
  name: &'static str,
  rating: f64
}

impl Team {
  pub fn new(name: &'static str, rating: f64) -> Team {
    Team {
      name: name,
      rating: rating
    }
  }

  pub fn name(&self) -> &'static str {
    self.name
  }

  pub fn rating(&self) -> f64 {
    self.rating
  }
}

