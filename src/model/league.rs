use crate::model::game::Game;
use crate::model::team::Team;

use colored::*;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TeamStats {
    pub team: &'static str,
    games_played: i16,
    wins: u16,
    draws: u16,
    losses: u16,
    goals_for: i64,
    goals_against: i64,
    goal_difference: i64,
    points: u64,
}

impl Ord for TeamStats {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.points).cmp(&(other.points))
    }
}

impl PartialOrd for TeamStats {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for TeamStats {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            "{0: <20} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10} | {8: <10}",
            self.team,
            self.games_played,
            self.wins,
            self.draws,
            self.losses,
            self.goals_for,
            self.goals_against,
            self.goal_difference,
            self.points
        )
    }
}

impl TeamStats {
    fn new(team: &'static str) -> TeamStats {
        TeamStats {
            team: team,
            games_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            goals_for: 0,
            goals_against: 0,
            goal_difference: 0,
            points: 0,
        }
    }

    fn update(&mut self, gf: i64, ga: i64) {
        self.games_played = self.games_played + 1;
        self.goals_for = self.goals_for + gf;
        self.goals_against = self.goals_against + ga;
        self.goal_difference = self.goal_difference + (gf - ga);
        if gf > ga {
            self.points = self.points + 3;
            self.wins = self.wins + 1;
        } else if ga == gf {
            self.points = self.points + 1;
            self.draws = self.draws + 1;
        } else {
            self.losses = self.losses + 1;
        }
    }
}

#[derive(Clone)]
pub struct League {
    name: &'static str,
    num_teams: usize,
    pub standings: HashMap<&'static str, TeamStats>,
}

impl Display for League {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let league = self.clone();
        let mut res: Result = Ok(());
        res = writeln!(
            f,
            "{0: <20} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10} | {8: <10}",
            "Team".bold(),
            "GP".bold(),
            "Wins".bold(),
            "Draws".bold(),
            "Losses".bold(),
            "GF".bold(),
            "GA".bold(),
            "GD".bold(),
            "Points".bold()
        );
        let standings = league.standings();
        for team in standings {
            res = write!(f, "{}", team)
        }
        return res;
    }
}

impl League {
    pub fn new(teams: Vec<&Team>) -> League {
        let standings: HashMap<&'static str, TeamStats> = teams
            .iter()
            .map(|t| (t.name, TeamStats::new(t.name)))
            .collect();

        League {
            name: "The League",
            num_teams: teams.len(),
            standings: standings,
        }
    }

    pub fn standings(&self) -> Vec<TeamStats> {
        let mut standings: Vec<TeamStats> = self.standings.iter().map(|(_, v)| v.clone()).collect();
        standings.sort();
        standings.reverse();
        return standings.clone();
    }

    pub fn leader(&self) -> TeamStats {
        *self.standings().first().unwrap()
    }

    pub fn update_standings(&mut self, game: Game) {
        self.standings
            .entry(game.home_team_name)
            .and_modify(|x| x.update(game.home_team_score, game.away_team_score));

        self.standings
            .entry(game.away_team_name)
            .and_modify(|x| x.update(game.away_team_score, game.home_team_score));
    }
}
