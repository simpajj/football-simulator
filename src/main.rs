extern crate simmy_dimmy;

pub use simmy_dimmy::team::team::Team;

fn main() {
    println!("Hello, world!");
    let team1 = Team::new("a", 1);
    println!("{:?}", team1);
}
