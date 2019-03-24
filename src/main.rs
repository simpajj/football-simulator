extern crate simmy_dimmy;
extern crate rand;

pub use simmy_dimmy::model::team::Team;
use rand::distributions::WeightedIndex;

fn main() {
    println!("Hello, world!");
    let team1 = Team::new("a", 2128.0);
    let team2 = Team::new("b", 2094.0);
    println!("{:?}", team1);
    println!("{:?}", team2);
    pick_winner(team1, team2)
}

// http://www.worldcup-simulator.de/static/data/Dormagen_2014_World_Cup_Simulator_2014-05-29.pdf
fn pick_winner(home_team: Team, away_team: Team) -> () {    
    let diff_home: f64 = home_team.rating()-away_team.rating();
    let diff_away: f64 = away_team.rating()-home_team.rating();
    let p_home: f64 = win_probability(diff_home);
    let p_away: f64 = win_probability(diff_away);
    println!("{}", p_home);
    println!("{}", p_away);
}

fn win_probability(diff: f64) -> f64 {
    let exp: f64 = 10.0;
    let c: f64 = 291.5;
    return 1.0/(1.0+exp.powf(-(diff/c)));
}
