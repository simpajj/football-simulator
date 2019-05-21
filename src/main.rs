extern crate rand;
extern crate simmy_dimmy;

pub use simmy_dimmy::engine::league;
pub use simmy_dimmy::engine::simulation;
pub use simmy_dimmy::model::team::Team;

use std::io;

fn main() {
    loop {
        let l = league::League::new();
        //let gr = simulation::simulate_game(team1, team2);
        println!("{}", l);
        println!("Play another match? (Yes/no)");

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
