use crate::model::game::Game;
use crate::model::team::Team;

use colored::*;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use uuid::Uuid;

const TEAMS: [(&'static str, f64); 20] = [
    ("London City", 2875.4),
    ("Manchester Albion", 2770.9),
    ("Borussia Berlin", 2759.0),
    ("Sporting Madrid", 2805.0),
    ("AC Roma", 2759.8),
    ("Olympique de Paris", 2774.4),
    ("SV München", 2779.4),
    ("Real Barcelona", 2818.7),
    ("Istanbul SK", 2752.4),
    ("FC Moscow", 2753.0),
    ("Liverpool Wanderers", 2777.1),
    ("Atletico Sevilla", 2760.6),
    ("AS Milano", 2760.0),
    ("SSD Napoli", 2777.1),
    ("AFC Amsterdam", 2767.5),
    ("Brussels FC", 2742.2),
    ("IFK Stockholm", 2747.6),
    ("Köpenhamn FK", 2753.5),
    ("Vienna FC", 2740.7),
    ("Prague FC", 2740.0),
];

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

    fn update<'a>(&'a mut self, gf: i64, ga: i64) {
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
    teams: HashMap<Uuid, Team>,
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
    pub fn new() -> League {
        let mut standings: HashMap<&'static str, TeamStats> = HashMap::new();
        let mut teams: HashMap<Uuid, Team> = HashMap::new();
        for team in TEAMS.iter() {
            let team = Team::new(team.0, team.1);
            standings.insert(team.name, TeamStats::new(team.name));
            teams.insert(team.id, team);
        }
        League {
            name: "The League",
            num_teams: TEAMS.len(),
            teams: teams,
            standings: standings,
        }
    }

    pub fn standings(&self) -> Vec<TeamStats> {
        let mut standings: Vec<TeamStats> = self.standings.iter().map(|(_, v)| v.clone()).collect();
        standings.sort();
        standings.reverse();
        return standings.clone();
    }

    pub fn update_standings<'a>(&'a mut self, game: Game) {
        self.standings
            .entry(game.home_team.name)
            .and_modify(|x| x.update(game.home_team_score, game.away_team_score));

        self.standings
            .entry(game.away_team.name)
            .and_modify(|x| x.update(game.away_team_score, game.home_team_score));
    }

    pub fn teams(&self) -> HashMap<Uuid, Team> {
        return self.teams.clone();
    }
}
