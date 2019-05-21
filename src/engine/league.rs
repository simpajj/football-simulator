use crate::model::team::Team;
use colored::*;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

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

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Debug)]
pub struct TeamStats {
    team: Team,
    wins: u16,
    draws: u16,
    losses: u16,
    goals_for: u64,
    goals_against: u64,
    goal_difference: u64,
    points: u64,
}

impl Ord for TeamStats {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.points).cmp(&(other.points))
    }
}

impl Display for TeamStats {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            "{0: <20} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10}",
            self.team.name,
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
    fn new(team: Team) -> TeamStats {
        TeamStats {
            team: team,
            wins: 0,
            draws: 0,
            losses: 0,
            goals_for: 0,
            goals_against: 0,
            goal_difference: 0,
            points: 0,
        }
    }
}

#[derive(Clone)]
pub struct League {
    name: &'static str,
    num_teams: usize,
    pub standings: Vec<TeamStats>,
}

impl Display for League {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let league = self.clone();
        let mut res: Result = Ok(());
        res = writeln!(
            f,
            "{0: <20} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10}",
            "Team".bold(),
            "Wins".bold(),
            "Draws".bold(),
            "Losses".bold(),
            "GF".bold(),
            "GA".bold(),
            "GD".bold(),
            "Points".bold()
        );
        for team in league.standings {
            res = write!(f, "{}", team)
        }
        return res;
    }
}

impl League {
    pub fn new() -> League {
        let mut standings = Vec::new();
        for team in TEAMS.iter() {
            standings.push(TeamStats::new(Team::new(team.0, team.1)))
        }
        League {
            name: "The League",
            num_teams: TEAMS.len(),
            standings: standings,
        }
    }

    pub fn standings(&mut self) -> &Vec<TeamStats> {
        self.standings.sort();
        return &self.standings;
    }
}
