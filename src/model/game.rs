use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Copy, Clone, Debug)]
pub enum Outcome {
    HomeWin,
    AwayWin,
    Draw,
}

#[derive(Debug, Clone)]
pub struct Game {
    round: usize,
    season: u64,
    pub home_team_name: &'static str,
    pub away_team_name: &'static str,
    pub home_team_score: i64,
    pub away_team_score: i64,
    pub outcome: Outcome,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        return self.home_team_name == other.home_team_name
            || self.away_team_name == other.away_team_name;
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
            self.home_team_name,
            self.away_team_name,
            self.home_team_score,
            self.away_team_score,
        )
    }
}

impl Game {
    pub fn new(
        round: usize,
        season: u64,
        home_team_name: &'static str,
        away_team_name: &'static str,
        home_team_score: i64,
        away_team_score: i64,
    ) -> Game {
        let outcome = if home_team_score > away_team_score {
            Outcome::HomeWin
        } else if away_team_score > home_team_score {
            Outcome::AwayWin
        } else {
            Outcome::Draw
        };
        Game {
            round: round,
            season: season,
            home_team_name: home_team_name,
            away_team_name: away_team_name,
            home_team_score: home_team_score,
            away_team_score: away_team_score,
            outcome: outcome,
        }
    }
}
