extern crate rand;
extern crate simmy_dimmy;

pub use simmy_dimmy::engine::simulation;
pub use simmy_dimmy::model::game::Game;
pub use simmy_dimmy::model::league;
pub use simmy_dimmy::model::league::TeamStats;
pub use simmy_dimmy::model::team::Team;

use std::collections::HashMap;
use std::io;

use itertools::Itertools;
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

fn main() {
    // todo: save season history (previous winner)
    let mut teams: HashMap<Uuid, Team> = TEAMS
        .iter()
        .map(|t| Team::new(t.0, t.1))
        .map(|team| (team.id, team))
        .collect();

    loop {
        let mut l = league::League::new(teams.iter().map(|t| t.1).collect_vec());
        let schedule: Vec<(Uuid, Uuid)> = teams
            .iter()
            .map(|(_, v)| v.id)
            .cartesian_product(teams.iter().map(|(_, v)| v.id))
            .filter(|x| !(x.0 == x.1))
            .collect();

        for t in schedule.iter() {
            let (result, ht, at) =
                simulation::simulate_game(*teams.get(&t.0).unwrap(), *teams.get(&t.1).unwrap());
            teams.insert(ht.id, ht);
            teams.insert(at.id, at);
            l.update_standings(result);
        }
        println!("{}", l);
        for team in teams.iter() {
            println!("{}, {}", team.1.name, team.1.rating)
        }

        // todo: make current ranking dependent on squad (implement team squad, players and player ratings)
        // todo: player development (ranking increase/decrease based on player age, training and match performance)

        println!("Play another season? (Yes/no)");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().to_lowercase().eq("yes") {
                    continue;
                } else {
                    break;
                }
            }
            Err(error) => {
                println!("error: {}", error);
                break;
            }
        }
    }
}
