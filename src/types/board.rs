use serde::{Deserialize, Serialize};

use super::cell::Cell;
use super::snake::Battlesnake;

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    height: u32,
    width: i32,
    food: Vec<Cell>,
    snakes: Vec<Battlesnake>,
    hazards: Vec<Cell>,
}
