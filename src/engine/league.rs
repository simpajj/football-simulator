use crate::model::team::Team;

const DEFAULT_NUM_TEAMS: u8 = 20;

#[derive(Copy, Clone)]
pub struct TeamStats {
    team: Team,
    wins: u16,
    draws: u16,
    losses: u16,
    goals_for: u64,
    goals_against: u64,
    points: u64,
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
            points: 0,
        }
    }
}

pub struct League {
    name: &'static str,
    num_teams: u16,
    standings: Vec<TeamStats>,
}

impl League {
    fn new() -> League {
        let mut standings = Vec::new();
        for _ in 0..DEFAULT_NUM_TEAMS {
            standings.push(TeamStats::new(Team::new("allan", 1.0)))
        }
        League {
            name: "The League",
            num_teams: 20,
            standings: standings,
        }
    }
}

// todo: team names
