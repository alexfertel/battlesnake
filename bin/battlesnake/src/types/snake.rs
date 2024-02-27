use serde::{Deserialize, Serialize};

use super::cell::Cell;

#[derive(Deserialize, Serialize, Debug)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: i32,
    body: Vec<Cell>,
    head: Cell,
    length: i32,
    latency: String,
    shout: Option<String>,
}
