use game::GameState;
use search::Direction;

use super::snake::Snake;

pub struct Dumb;

impl Snake for Dumb {
    fn get_move(&self, _game: &GameState) -> Direction {
        Direction::Up
    }
}
