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
    ("Malmö FF", 2875.4),
    ("AIK", 2818.7),
    ("IFK Norrköping", 2805.0),
    ("Hammarby IF", 2779.4),
    ("BK Häcken", 2777.1),
    ("Djurgårdens IF", 2777.1),
    ("IFK Göteborg", 2774.4),
    ("Östersund IK", 2770.9),
    ("IF Elfsborg", 2767.5),
    ("Kalmar FF", 2760.6),
    ("Örebro SK", 2760.0),
    ("IK Sirius", 2759.8),
    ("Helsingsborgs IF", 2759.0),
    ("GIF Sundsvall", 2753.5),
    ("AFC Eskilstuna", 2753.0),
    ("Örgryte IS", 2752.4),
    ("Falkenbergs FF", 2747.6),
    ("GAIS", 2742.2),
    ("IF Brommapojkarna", 2740.7),
    ("Halmstads BK", 2740.0),
];

fn main() {
    let mut winners: HashMap<&'static str, i64> = HashMap::new();

    let mut seasons: u64 = 0;
    while seasons <= 0 {
        println!("Enter the number of seasons that you want to simulate (min. 1): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the number of seasons");
        let trimmed = input.trim();
        match trimmed.parse::<u64>() {
            Ok(i) => {
                if i <= 0 {
                    continue;
                } else {
                    seasons = i;
                }
            }
            Err(..) => {
                println!("this was not an integer: {}", trimmed);
                std::process::exit(1)
            }
        }
    }

    let mut teams: HashMap<Uuid, Team> = TEAMS
        .iter()
        .map(|t| Team::new(t.0, t.1))
        .map(|team| (team.id, team))
        .collect();

    for _ in 0..seasons {
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
        /*println!("{}", l);
        for team in teams.iter() {
            println!("{}, {}", team.1.name, team.1.rating)
        }*/

        winners
            .entry(l.leader().team)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        // todo: player development (ranking increase/decrease based on player age, training and match performance)
        // todo: make team ranking dependent on squad (implement team squad, players and player ratings)
        // parallelize certain operations using Rayon (https://github.com/rayon-rs/rayon)?
    }

    let mut w: Vec<(&'static str, &i64)> = winners.iter().map(|(k, v)| (*k, v)).collect();
    w.sort_by(|a, b| b.1.cmp(a.1));
    for winner in w {
        println!("{:?}", winner);
    }
}
