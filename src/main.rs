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
