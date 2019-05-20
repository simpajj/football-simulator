use crate::model::team::Team;
extern crate rand;

#[derive(Copy, Clone)]
pub enum Outcome {
  HomeWin,
  AwayWin,
  Draw,
}

pub trait Result {
  fn home_team(&self) -> Team;
  fn away_team(&self) -> Team;
  fn home_team_score(&self) -> u16;
  fn away_team_score(&self) -> u16;
}

#[derive(Debug)]
pub struct WinLossResult {
  home_team: Team,
  away_team: Team,
  home_team_score: u16,
  away_team_score: u16,
  winner: Team,
  loser: Team,
}

impl WinLossResult {
  pub fn winner(&self) -> Team {
    if self.home_team_score > self.away_team_score {
      return self.home_team();
    } else {
      return self.away_team();
    }
  }
  pub fn loser(&self) -> Team {
    if self.home_team_score < self.away_team_score {
      return self.home_team();
    } else {
      return self.away_team();
    }
  }
  pub fn new(
    home_team: Team,
    away_team: Team,
    home_team_score: u16,
    away_team_score: u16,
    winner: Team,
    loser: Team,
  ) -> WinLossResult {
    WinLossResult {
      home_team: home_team,
      away_team: away_team,
      home_team_score: home_team_score,
      away_team_score: away_team_score,
      winner: winner,
      loser: loser,
    }
  }
}

impl Result for WinLossResult {
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

#[derive(Debug)]
pub struct DrawResult {
  home_team: Team,
  away_team: Team,
  home_team_score: u16,
  away_team_score: u16,
}

impl DrawResult {
  pub fn new(
    home_team: Team,
    away_team: Team,
    home_team_score: u16,
    away_team_score: u16,
  ) -> DrawResult {
    DrawResult {
      home_team: home_team,
      away_team: away_team,
      home_team_score: home_team_score,
      away_team_score: away_team_score,
    }
  }
}

impl Result for DrawResult {
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
