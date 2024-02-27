use serde::{Deserialize, Serialize};

use super::cell::Cell;
use super::snake::Battlesnake;

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    height: usize,
    width: usize,
    food: Vec<Cell>,
    snakes: Vec<Battlesnake>,
    hazards: Vec<Cell>,
}
