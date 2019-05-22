use crate::model::team::Team;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Copy, Clone, Debug)]
pub enum Outcome {
    Scheduled,
    HomeWin,
    AwayWin,
    Draw,
}

#[derive(Debug, Clone)]
pub struct Game {
    round: usize,
    season: u64,
    pub home_team: Team,
    pub away_team: Team,
    pub home_team_score: i64,
    pub away_team_score: i64,
    pub winner: Option<Team>,
    pub loser: Option<Team>,
    pub outcome: Outcome,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        return self.home_team == other.home_team || self.away_team == other.away_team;
    }
}

impl Eq for Game {}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "{0} | {1} | {2} vs. {3: <5} {4}:{5}",
            self.round,
            self.season,
            self.home_team,
            self.away_team,
            self.home_team_score,
            self.away_team_score,
        )
    }
}

impl Game {
    pub fn new(round: usize, season: u64, home_team: Team, away_team: Team) -> Game {
        Game {
            round: round,
            season: season,
            home_team: home_team,
            away_team: away_team,
            home_team_score: 0,
            away_team_score: 0,
            winner: None,
            loser: None,
            outcome: self::Outcome::Scheduled,
        }
    }

    pub fn copy(&self, home_team_score: i64, away_team_score: i64) -> Game {
        let (outcome, winner, loser) = if home_team_score > away_team_score {
            (Outcome::HomeWin, Some(self.home_team), Some(self.away_team))
        } else if away_team_score > home_team_score {
            (Outcome::AwayWin, Some(self.away_team), Some(self.home_team))
        } else {
            (Outcome::Draw, None, None)
        };

        Game {
            round: self.round,
            season: self.season,
            home_team: self.home_team,
            away_team: self.away_team,
            home_team_score: home_team_score,
            away_team_score: away_team_score,
            winner: winner,
            loser: loser,
            outcome: outcome,
        }
    }
}
