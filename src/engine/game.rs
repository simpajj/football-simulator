use crate::model::team::Team;
extern crate rand;
use rand::Rng;

pub struct GameResult {
  home_team: Team,
  away_team: Team,
  home_team_score: u16,
  away_team_score: u16
}

impl GameResult {
  fn home_team(&self) -> &Team {
    &self.home_team
  }
  fn away_team(&self) -> &Team {
    &self.away_team
  }
  fn home_team_score(&self) -> &u16 {
    &self.home_team_score
  }
  fn away_team_score(&self) -> &u16 {
    &self.away_team_score
  }
  fn is_draw(&self) -> bool {
    &self.home_team_score == &self.away_team_score
  }
}

pub fn game(home_team: Team, away_team: Team) -> GameResult {
  let num = rand::thread_rng().gen_range(0, 10);
  if num < 5 {
    GameResult{
      home_team: home_team,
      away_team: away_team,
      home_team_score: 1,
      away_team_score: 0,
    }
  } else {
    GameResult{
      home_team: home_team,
      away_team: away_team,
      home_team_score: 0,
      away_team_score: 1,
    }
  }
}