use game::GameState;
use search::Direction;

pub trait Snake: Send {
    fn get_move(&self, game: &GameState) -> Direction;
}
