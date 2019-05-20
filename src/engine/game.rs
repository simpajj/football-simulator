use crate::model::team::Team;
use std::fmt::Debug;
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
  fn home_team_score(&self) -> u64;
  fn away_team_score(&self) -> u64;
  fn debug_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Data {{ ... }}")
  }
}

impl Debug for Result {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    self.debug_fmt(f)
  }
}

#[derive(Debug)]
pub struct WinLossResult {
  home_team: Team,
  away_team: Team,
  home_team_score: u64,
  away_team_score: u64,
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
    home_team_score: u64,
    away_team_score: u64,
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
  fn home_team_score(&self) -> u64 {
    self.home_team_score
  }
  fn away_team_score(&self) -> u64 {
    self.away_team_score
  }
  fn debug_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    self.fmt(f)
  }
}

#[derive(Debug)]
pub struct DrawResult {
  home_team: Team,
  away_team: Team,
  home_team_score: u64,
  away_team_score: u64,
}

impl DrawResult {
  pub fn new(
    home_team: Team,
    away_team: Team,
    home_team_score: u64,
    away_team_score: u64,
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
  fn home_team_score(&self) -> u64 {
    self.home_team_score
  }
  fn away_team_score(&self) -> u64 {
    self.away_team_score
  }
  fn debug_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    self.fmt(f)
  }
}
