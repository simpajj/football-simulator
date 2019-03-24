extern crate simmy_dimmy;

pub use simmy_dimmy::model::team::Team;

fn main() {
    println!("Hello, world!");
    let team1 = Team::new("a", 1);
    let team2 = Team::new("b", 1);
    println!("{:?}", team1);
    println!("{:?}", team2);
}
