use std::collections::HashMap;

use crate::snakes::snake::Snake;
use game::Game;

pub type GameId = String;

pub struct AppState {
    pub games: HashMap<GameId, Game>,
    pub snakes: HashMap<GameId, Box<dyn Snake>>,
}
