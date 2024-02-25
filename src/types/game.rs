use serde_json::Value;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{board::Board, snake::Battlesnake};

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    game: Game,
    turn: i32,
    board: Board,
    you: Battlesnake,
}
