use rand::distributions::Poisson;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::engine::game;
use crate::model::team::Team;

const MATCH_IMPORTANCE: f64 = 5.0;

enum ResultPoints {
    Win,
    Draw,
    Defeat,
}

impl ResultPoints {
    fn value(&self) -> f64 {
        match *self {
            ResultPoints::Win => 1.0,
            ResultPoints::Draw => 0.5,
            ResultPoints::Defeat => 0.0,
        }
    }
}

// http://www.worldcup-simulator.de/static/data/Dormagen_2014_World_Cup_Simulator_2014-05-29.pdf
pub fn simulate_game<'a>(home_team: &'a Team, away_team: &'a Team) -> Box<game::Result> {
    let p_home: f64 = win_probability(home_team.rating() - away_team.rating());
    let p_away: f64 = win_probability(away_team.rating() - home_team.rating());
    let p_draw: f64 = draw_probability(p_home, p_away);

    let outcome = choose_outcome(p_home, p_away, p_draw);
    let normalized_probability = p_home / (p_home + p_away);

    match outcome {
        game::Outcome::HomeWin => {
            let ht = Team::update_rating(
                home_team,
                MATCH_IMPORTANCE,
                ResultPoints::Win.value(),
                p_home,
            );
            let at = Team::update_rating(
                away_team,
                MATCH_IMPORTANCE,
                ResultPoints::Defeat.value(),
                p_away,
            );
            let score = score(normalized_probability, outcome);
            return Box::new(game::WinLossResult::new(ht, at, score.0, score.1, ht, at));
        }
        game::Outcome::AwayWin => {
            let ht = Team::update_rating(
                home_team,
                MATCH_IMPORTANCE,
                ResultPoints::Defeat.value(),
                p_home,
            );
            let at = Team::update_rating(
                away_team,
                MATCH_IMPORTANCE,
                ResultPoints::Win.value(),
                p_away,
            );
            let score = score(normalized_probability, outcome);
            return Box::new(game::WinLossResult::new(ht, at, score.0, score.1, at, ht));
        }
        game::Outcome::Draw => {
            let ht = Team::update_rating(
                home_team,
                MATCH_IMPORTANCE,
                ResultPoints::Draw.value(),
                p_home,
            );
            let at = Team::update_rating(
                away_team,
                MATCH_IMPORTANCE,
                ResultPoints::Draw.value(),
                p_away,
            );
            let score = score(normalized_probability, outcome);
            return Box::new(game::DrawResult::new(ht, at, score.0, score.1));
        }
    };
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
    let probabilities = [
        (game::Outcome::HomeWin, p_home * 10.0),
        (game::Outcome::AwayWin, p_away * 10.0),
        (game::Outcome::Draw, p_draw * 10.0),
    ];
    let index = WeightedIndex::new(probabilities.iter().map(|item| item.1)).unwrap();
    return probabilities[index.sample(&mut thread_rng())].0;
}

fn score(normalized_probability: f64, outcome: game::Outcome) -> (u64, u64) {
    let dist = 1.8 * normalized_probability + 0.27;
    let home_goals = Poisson::new(dist);
    let away_goals = Poisson::new(dist);

    let mut h_score = 0;
    let mut a_score = 0;
    let mut rng = thread_rng();

    match outcome {
        game::Outcome::HomeWin => {
            while h_score <= a_score {
                h_score = home_goals.sample(&mut rng);
                a_score = away_goals.sample(&mut rng);
            }
            return (h_score, a_score);
        }
        game::Outcome::AwayWin => {
            while h_score >= a_score {
                h_score = home_goals.sample(&mut rng);
                a_score = away_goals.sample(&mut rng);
            }
            return (h_score, a_score);
        }
        game::Outcome::Draw => {
            h_score = home_goals.sample(&mut rng);
            a_score = h_score;
            return (h_score, a_score);
        }
    }
}
