use crate::model::team::Team;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, Clone)]
pub struct Game {
    round: usize,
    season: u64,
    home_team: Team,
    away_team: Team,
    home_team_score: u64,
    away_team_score: u64,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        return self.home_team == other.home_team || self.away_team == other.away_team;
    }
}

impl Eq for Game {}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> Result {
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
        }
    }
}
