use crate::model::team::Team;
extern crate rand;
use rand::Rng;

pub trait GameResult {
  fn home_team(&self) -> Team;
  fn away_team(&self) -> Team;
  fn home_team_score(&self) -> u16;
  fn away_team_score(&self) -> u16;
}

// TODO: implement DrawResult
pub struct WinLossResult {
  home_team: Team,
  away_team: Team,
  home_team_score: u16,
  away_team_score: u16,
}

impl WinLossResult {
  fn winner(&self) -> Team {
    if self.home_team_score > self.away_team_score {
      return self.home_team();
    } else {
      return self.away_team();
    }
  }
}

impl GameResult for WinLossResult {
  fn home_team(&self) -> Team {
    self.home_team
  }
  fn away_team(&self) -> Team {
    self.away_team
  }
  fn home_team_score(&self) -> u16 {
    self.home_team_score
  }
  fn away_team_score(&self) -> u16 {
    self.away_team_score
  }
}

pub fn game(home_team: Team, away_team: Team) -> impl GameResult {
  let num = rand::thread_rng().gen_range(0, 10);
  if num < 5 {
    WinLossResult {
      home_team: home_team,
      away_team: away_team,
      home_team_score: 1,
      away_team_score: 0,
    }
  } else {
    WinLossResult {
      home_team: home_team,
      away_team: away_team,
      home_team_score: 0,
      away_team_score: 1,
    }
  }
}
