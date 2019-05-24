extern crate football_simulator;
extern crate rand;

#[macro_use]
extern crate lazy_static;

pub use football_simulator::engine::simulation;
pub use football_simulator::model::game::Game;
pub use football_simulator::model::league;
pub use football_simulator::model::league::League;
pub use football_simulator::model::team::Team;

use std::io::{self, Write};

use colored::*;
use hashbrown::HashMap;
use indicatif::{ProgressBar, ProgressStyle};
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
        "{0: <20}{1}\n{2: <20}{3}\n{4: <20}{5}\n{6: <20}{7}\n{8: <20}{9}\n{10: <20}{11}",
        "command".bold(),
        "description".bold(),
        "help",
        ": show this message",
        "simulate <seasons>",
        ": start a new simulation with the given number of seasons",
        "stats <season>",
        ": show the season stats for the given season from the previous simulation",
        "winners",
        ": show the count of all league winners from the previous simulation",
        "exit:",
        ":'("
    );
    static ref HELP: &'static str = HELP_FMT.as_str();
}

fn main() {
    let mut winners: HashMap<&'static str, i64> = HashMap::new();
    let mut standings: HashMap<i64, League> = HashMap::new();

    println!("{}", *HELP);
    let mut exit = false;
    while !exit {
        print!("football-simulator> ");
        let _ = io::stdout().flush();
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
            x if x.starts_with("simulate") => {
                let split: Vec<&str> = x.split_whitespace().collect();
                match split.get(1) {
                    Some(arg) => match arg.parse::<u64>() {
                        Ok(seasons) => {
                            let pb = ProgressBar::new(seasons);
                            pb.set_style(ProgressStyle::default_bar().template(
                                "[{elapsed_precise}] {bar:40.black} {pos:>7}/{len:7} {msg}",
                            ));
                            let (w, s) = simulate(seasons, &pb);
                            winners = w;
                            standings = s;
                            pb.finish_with_message(
                                format!("Finished simulating {} seasons", seasons).as_str(),
                            );
                        }
                        Err(..) => println!("Invalid number of seasons for command 'simulate'"),
                    },
                    None => println!("No seasons provided for command 'simulate'"),
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

fn simulate(seasons: u64, pb: &ProgressBar) -> (HashMap<&'static str, i64>, HashMap<i64, League>) {
    let mut winners: HashMap<&'static str, i64> = HashMap::new();
    let mut standings: HashMap<i64, League> = HashMap::new();

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

    return (winners, standings);
}