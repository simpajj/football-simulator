#[derive(Debug, Clone, Copy)]
pub struct Team {
  name: &'static str,
  pub rating: f64
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

  pub fn update_rating<'a>(team: &'a Team, match_importance: f64, match_result: f64, win_probability: f64) -> Team {
    let rating = team.rating + match_importance * (match_result - win_probability).round();
    return Team::new(team.name(), rating);
  }
}