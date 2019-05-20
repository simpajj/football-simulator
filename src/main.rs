extern crate rand;
extern crate simmy_dimmy;

pub use simmy_dimmy::engine::simulation;
pub use simmy_dimmy::model::team::Team;

fn main() {
    let team1 = &mut Team::new("a", 2128.0);
    let team2 = &mut Team::new("b", 2094.0);
    let gr = simulation::simulate_game(team1, team2);
}
