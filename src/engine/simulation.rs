use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::engine::game;
use crate::model::team::Team;

const MATCH_IMPORTANCE: f64 = 5.0;

// http://www.worldcup-simulator.de/static/data/Dormagen_2014_World_Cup_Simulator_2014-05-29.pdf
pub fn simulate_game<'a>(home_team: &'a Team, away_team: &'a Team) -> Box<game::Result> {
    let p_home: f64 = win_probability(home_team.rating() - away_team.rating());
    let p_away: f64 = win_probability(away_team.rating() - home_team.rating());
    let p_draw: f64 = draw_probability(p_home, p_away);

    let outcome = choose_outcome(p_home, p_away, p_draw);

    match outcome {
        game::Outcome::HomeWin => {
            let ht = Team::update_rating(home_team, MATCH_IMPORTANCE, 1.0, p_home);
            let at = Team::update_rating(away_team, MATCH_IMPORTANCE, 0.0, p_away);
            return Box::new(game::WinLossResult::new(ht, at, 1, 0, ht, at));
        }
        game::Outcome::AwayWin => {
            let ht = Team::update_rating(home_team, MATCH_IMPORTANCE, 0.0, p_home);
            let at = Team::update_rating(away_team, MATCH_IMPORTANCE, 1.0, p_away);
            return Box::new(game::WinLossResult::new(ht, at, 0, 1, at, ht));
        }
        game::Outcome::Draw => {
            let ht = Team::update_rating(home_team, MATCH_IMPORTANCE, 0.0, p_home); // FIXME: score when updating elo
            let at = Team::update_rating(away_team, MATCH_IMPORTANCE, 0.0, p_away); // FIXME
            return Box::new(game::DrawResult::new(ht, at, 0, 0));
        }
    };
    // TODO: simulate match score
}

fn win_probability(diff: f64) -> f64 {
    let exp: f64 = 10.0;
    let c: f64 = 291.5;
    return 1.0 / (1.0 + exp.powf(-diff / c));
}

fn draw_probability(p_home: f64, p_away: f64) -> f64 {
    let normalized_p: f64 = p_home / (p_home + p_away);
    return (1.0 / 3.0)
        * std::f64::consts::E
            .powf(-f64::powf(normalized_p - 0.5, 2.0) / (2.0 * f64::powf(0.28, 2.0)));
}

fn choose_outcome(p_home: f64, p_away: f64, p_draw: f64) -> game::Outcome {
    let mut rng = thread_rng();
    let probabilities = [
        (game::Outcome::HomeWin, p_home * 10.0),
        (game::Outcome::AwayWin, p_away * 10.0),
        (game::Outcome::Draw, p_draw * 10.0),
    ];
    let index = WeightedIndex::new(probabilities.iter().map(|item| item.1)).unwrap();
    return probabilities[index.sample(&mut rng)].0;
}
