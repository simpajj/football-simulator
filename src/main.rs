extern crate simmy_dimmy;
extern crate rand;

pub use simmy_dimmy::model::team::Team;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

fn main() {
    println!("Hello, world!");
    let team1 = &mut Team::new("a", 2128.0);
    let team2 = &mut Team::new("b", 2094.0);
    let gr = pick_winner(team1, team2);
    println!("{:?}", gr);
}

// http://www.worldcup-simulator.de/static/data/Dormagen_2014_World_Cup_Simulator_2014-05-29.pdf
fn pick_winner<'a>(home_team: &'a Team, away_team: &'a Team) -> GameResult {    
    let diff_home: f64 = home_team.rating()-away_team.rating();
    let diff_away: f64 = away_team.rating()-home_team.rating();
    let p_home: f64 = win_probability(diff_home);
    let p_away: f64 = win_probability(diff_away);

    let mut rng = thread_rng();
    let probabilities = [(home_team, p_home*10.0), (away_team, p_away*10.0)];
    let index = WeightedIndex::new(probabilities.iter().map(|item|item.1)).unwrap();
    let winning_team = &probabilities[index.sample(&mut rng)].0;
    
    let match_importance = 5.0;
    return match winning_team {
        ref t if t.name() == home_team.name() => {
            let ht = Team::update_rating(home_team, match_importance, 1.0, p_home);
            let at = Team::update_rating(away_team, match_importance, 0.0, p_away);
            println!("{}", home_team.rating());
            GameResult {
                home_team: ht,
                away_team: at,
                home_team_score: 1,
                away_team_score: 0,
                winner: Some(ht),
                loser: Some(at)
            }
        }
        _ => {
            let ht = Team::update_rating(home_team, match_importance, 0.0, p_home);
            let at = Team::update_rating(away_team, match_importance, 1.0, p_away);
            GameResult {
                home_team: ht,
                away_team: at,
                home_team_score: 0,
                away_team_score: 1,
                winner: Some(at),
                loser: Some(ht)
            }
        }
    }

    // TODO: match score
    // TODO: draws
}

#[derive(Debug)]
struct GameResult<> {
    home_team: Team,
    away_team: Team,
    home_team_score: i32,
    away_team_score: i32,
    winner: Option<Team>,
    loser: Option<Team>
}

fn win_probability(diff: f64) -> f64 {
    let exp: f64 = 10.0;
    let c: f64 = 291.5;
    return 1.0/(1.0+exp.powf(-diff/c));
}