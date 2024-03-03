use game::{Cell, GameState};
use search::Direction;

const YOU: u8 = 0b0001;
const FOE: u8 = 0b0010;
const FOOD: u8 = 0b0100;
const HEAD: u8 = 0b1000;

#[derive(Debug, Clone)]
pub struct Bits {
    bs: Vec<u8>,
    size: usize,
}

impl Bits {
    pub fn new(size: usize, len: usize) -> Self {
        Self {
            bs: vec![0; len],
            size,
        }
    }

    pub fn get(&self, what: u8) -> Option<usize> {
        for i in 0..self.bs.len() {
            if self.bs[i] & !what == 0 {
                return Some(i);
            }
        }

        None
    }

    pub fn set(&mut self, cells: &[Cell], what: u8) {
        let n = self.bs.len();
        for c in cells {
            let cx = c.x as usize;
            let cy = c.y as usize;

            let i = cx * n + cy;

            self.bs[i] &= what;
        }
    }
}

pub struct Node {
    pub board: Bits,
    pub wins: usize,
    pub simulations: usize,
    pub children: Option<Vec<Node>>,
}

impl Node {
    pub fn new(board: Bits) -> Self {
        Self {
            board,
            wins: 0,
            simulations: 0,
            children: None,
        }
    }

    fn populate_children(&mut self) {
        let yh = self
            .board
            .get(YOU | HEAD)
            .expect("the head of the you snake should exist");
        let fh = self
            .board
            .get(FOE | HEAD)
            .expect("the head of the foe snake should exist");

        let yhx = (yh / self.board.size) as isize;
        let yhy = (yh % self.board.size) as isize;
        let fhx = (fh / self.board.size) as isize;
        let fhy = (fh % self.board.size) as isize;
        for (dy, df) in Direction::all().iter().zip(Direction::all()) {
            let dyv = dy.vector();
            // TODO: Check for invalid children. (head moving in direction will eat itself)
        }
    }

    pub fn get_successor(&mut self) -> Node {
        if self.children.is_none() {
            self.populate_children();
        }

        Node::new(self.board.clone())
    }
}

pub struct MonteCarlo {
    pub root: Node,
}

impl MonteCarlo {
    pub fn new(game: &GameState) -> Self {
        // We assume the board is a square.
        let n = game.board.height * game.board.height;
        let snakes = game.board.snakes;
        let mut board = Bits::new(game.board.height, n);
        board.set(&game.board.food, FOOD);
        board.set(&snakes[0].body, YOU);
        board.set(&snakes[1].body, FOE);
        board.set(&[snakes[0].body[0]], HEAD);
        board.set(&[snakes[1].body[0]], HEAD);

        let root = Node::new(board);
        Self { root }
    }
}
