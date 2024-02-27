use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Cell {
    x: i32,
    y: i32,
}
