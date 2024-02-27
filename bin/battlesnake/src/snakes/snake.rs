use crate::types::{direction::Direction, game::GameState};

pub trait Snake: Send {
    fn get_move(&self, game: &GameState) -> Direction;
}
