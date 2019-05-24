extern crate football_simulator;
extern crate rand;

#[macro_use]
extern crate lazy_static;

pub use football_simulator::engine::simulation;
pub use football_simulator::model::game::Game;
pub use football_simulator::model::league;
pub use football_simulator::model::league::League;
pub use football_simulator::model::team::Team;

use std::collections::HashMap;
use std::io;

use indicatif::ProgressBar;
use itertools::Itertools;
use uuid::Uuid;

const TEAMS: [(&'static str, f64); 20] = [
    ("Malmö FF", 2875.4),
    ("AIK", 2818.7),
    ("IFK Norrköping", 2805.0),
    ("Hammarby IF", 2709.4),
    ("BK Häcken", 2677.1),
    ("Djurgårdens IF", 2598.1),
    ("IFK Göteborg", 2556.4),
    ("Östersund IK", 2511.9),
    ("IF Elfsborg", 2567.5),
    ("Kalmar FF", 2547.6),
    ("Örebro SK", 2540.0),
    ("Helsingsborgs IF", 2520.0),
    ("GIF Sundsvall", 2501.5),
    ("IK Sirius", 2475.8),
    ("AFC Eskilstuna", 2459.0),
    ("Örgryte IS", 2452.4),
    ("Falkenbergs FF", 2447.6),
    ("GAIS", 2442.2),
    ("IF Brommapojkarna", 2430.7),
    ("Halmstads BK", 2400.0),
];

lazy_static! {
    static ref HELP_FMT: String = format!(
        "{0: <20}{1}\n{2: <20}{3}\n{4: <20}{5}\n{6: <20}{7}",
        "help:",
        "show this message",
        "stats <season>:",
        "show the season stats for the selected season",
        "winners:",
        "show the count of all league winners",
        "exit:",
        ":'("
    );
    static ref HELP: &'static str = HELP_FMT.as_str();
}

// todo: refactor fugliness
fn main() {
    let mut winners: HashMap<&'static str, i64> = HashMap::new();
    let mut standings: HashMap<i64, League> = HashMap::new();

    let mut seasons: u64 = 0;
    println!("Enter the number of seasons that you want to simulate (min. 1):");

    while seasons <= 0 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the number of seasons");
        match input.trim().parse::<u64>() {
            Ok(i) => {
                if i <= 0 {
                    println!("Please enter a number greater than 0:");
                    continue;
                } else {
                    seasons = i;
                }
            }
            Err(..) => {
                println!("Please enter a number greater than 0:");
                continue;
            }
        }
    }

    let mut teams: HashMap<Uuid, Team> = TEAMS
        .iter()
        .map(|t| Team::new(t.0, t.1))
        .map(|team| (team.id, team))
        .collect();

    // todo: use https://docs.rs/ndarray/0.12.1/ndarray/ instead? may be able to parallelize
    let schedule: Vec<(Uuid, Uuid)> = teams
        .iter()
        .map(|(_, v)| v.id)
        .cartesian_product(teams.iter().map(|(_, v)| v.id))
        .filter(|x| !(x.0 == x.1))
        .collect();
    
    let pb = ProgressBar::new(seasons);
    for season in 1..seasons + 1 {
        let mut l = league::League::new(teams.values().collect_vec(), season as i64);

        for t in schedule.iter() {
            let (result, ht, at) =
                simulation::simulate_game(*teams.get(&t.0).unwrap(), *teams.get(&t.1).unwrap());
            teams.insert(ht.id, ht);
            teams.insert(at.id, at);
            l.update_standings(result);
        }

        winners
            .entry(l.leader().team)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        standings.insert(season as i64, l);
        pb.inc(1);

        // todo: player development (ranking increase/decrease based on player age, training and match performance)
        // todo: make team ranking dependent on squad (implement team squad, players and player ratings)
        // parallelize certain operations using Rayon (https://github.com/rayon-rs/rayon)?
    }

    pb.finish_with_message("All seasons simulated!");
    println!("{}", *HELP);
    let mut exit = false;
    while !exit {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read command");
        match input.trim() {
            "help" => println!("{}", *HELP),
            "winners" => {
                let mut w: Vec<(&'static str, &i64)> =
                    winners.iter().map(|(k, v)| (*k, v)).collect();
                w.sort_by(|a, b| b.1.cmp(a.1));
                for winner in w {
                    println!("{:?}", winner);
                }
            }
            x if x.starts_with("stats") => {
                let split: Vec<&str> = x.split_whitespace().collect();
                match split.get(1) {
                    Some(arg) => match arg.parse::<i64>() {
                        Ok(season) => match standings.get(&season) {
                            Some(season_standings) => println!("{}", season_standings),
                            None => println!("No standings found for season"),
                        },
                        Err(..) => println!("Invalid season for command 'stats'"),
                    },
                    None => println!("No season provided for command 'stats'"),
                }
            } // take the second index and check that it's a valid number
            "exit" => exit = true,
            _ => {
                println!("Unrecognized command!");
                continue;
            }
        }
    }
    std::process::exit(0);
}
