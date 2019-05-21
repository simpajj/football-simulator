use crate::model::game;
use std::collections::HashMap;

pub struct Schedule {
    rounds: HashMap<u8, Vec<game::Game>>,
}
