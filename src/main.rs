extern crate rand;
extern crate simmy_dimmy;

pub use simmy_dimmy::engine::simulation;
pub use simmy_dimmy::model::league;
pub use simmy_dimmy::model::league::TeamStats;
pub use simmy_dimmy::model::team::Team;

use itertools::Itertools;
use std::io;
use uuid::Uuid;

fn main() {
    loop {
        let mut l = league::League::new();

        let s1: Vec<(Uuid, &'static str)> = l.teams().iter().map(|x| (x.id, x.name)).collect();
        let a: Vec<(&(uuid::Uuid, &str), &(uuid::Uuid, &str))> = s1
            .iter()
            .cartesian_product(&s1)
            .filter(|x| !((x.0).1 == (x.1).1))
            .collect();

        for b in a.iter() {
            println!("{:?}", b);
        }
        //let gr = simulation::simulate_game(team1, team2);
        println!("Play another match? (Yes/no)");

        // todo: simulate all games between all teams (1 home and 1 away vs each team)
        // todo: save season history (previous winner)
        // todo: make current ranking dependent on squad (implement team squad, players and player ratings)
        // todo: player development (ranking increase/decrease based on player age, training and match performance)

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
