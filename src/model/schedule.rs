use crate::model::game::Game;
use crate::model::league::League;

use itertools::Itertools;

pub fn schedule(league: &mut League) -> Vec<Game> {
    let teams = league.teams().to_vec();
    teams
        .iter()
        .cartesian_product(&teams)
        .filter(|x| !((x.0).name == (x.1).name))
        .map(|x| Game::new(0, 0, *x.0, *x.1))
        .collect()
}
