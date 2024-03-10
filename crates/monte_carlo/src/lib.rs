use std::ops::Index;

use game::GameState;
use search::{Direction, Point};

const YOU: u8 = 0b00001;
const FOE: u8 = 0b00010;
const FOOD: u8 = 0b00100;
const HEAD: u8 = 0b01000;
const TAIL: u8 = 0b10000;
const BRANCHING_FACTOR: usize = 0;

#[derive(Debug, Clone)]
pub struct Buffer {
    bs: Vec<u8>,
    size: usize,
}

impl Buffer {
    pub fn new(size: usize, len: usize) -> Self {
        Self {
            bs: vec![0; len],
            size,
        }
    }

    pub fn get(&self, what: u8) -> Option<usize> {
        for i in 0..self.bs.len() {
            if self.bs[i] == what {
                return Some(i);
            }
        }

        None
    }

    #[inline]
    pub fn or(&mut self, idx: usize, what: u8) {
        self.bs[idx] |= what;
    }

    #[inline]
    pub fn and(&mut self, idx: usize, what: u8) {
        self.bs[idx] &= what;
    }
}

impl Index<usize> for Buffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bs[index]
    }
}

pub struct Node {
    pub board: Buffer,
    pub wins: usize,
    pub simulations: usize,
    pub children: Option<Vec<Node>>,
}

impl Node {
    pub fn new(board: Buffer) -> Self {
        Self {
            board,
            wins: 0,
            simulations: 0,
            children: None,
        }
    }

    fn populate_children(&mut self) {
        let mut children = Vec::with_capacity(BRANCHING_FACTOR);

        let yh = self.board.get(YOU | HEAD);
        let yh = yh.expect("the head of the you snake should exist");
        let fh = self.board.get(FOE | HEAD);
        let fh = fh.expect("the head of the foe snake should exist");

        let n = self.board.size;
        let yhx = (yh / n) as isize;
        let yhy = (yh % n) as isize;
        let fhx = (fh / n) as isize;
        let fhy = (fh % n) as isize;

        let yt = self.board.get(YOU | TAIL);
        let yt = yt.expect("the tail of the you snake should exist");
        let ft = self.board.get(YOU | TAIL);
        let ft = ft.expect("the tail of the foe snake should exist");
        for (dy, df) in Direction::all().iter().zip(Direction::all()) {
            let dyv = dy.vector();
            let nyv = dyv + Point::new(yhx, yhy);
            let yi = nyv.index(n);
            if self.board[yi] == YOU {
                continue;
            }

            let dfv = df.vector();
            let nfv = dfv + Point::new(fhx, fhy);
            let fi = nfv.index(n);
            if self.board[fi] == FOE {
                continue;
            }

            let mut nb = self.board.clone();
            nb.or(yi, YOU | HEAD);
            nb.and(yh, YOU);
            nb.and(yt, 0);
            nb.or(fi, FOE | HEAD);
            nb.and(fh, FOE);
            nb.and(ft, 0);

            children.push(Node::new(nb));
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
        let n = game.board.height;
        let snakes = &game.board.snakes;
        let mut board = Buffer::new(n, n * n);
        for c in game.board.food.iter().map(|p| p.index(n)) {
            board.or(c, FOOD);
        }

        let you = &snakes[0];
        let head = you.body[0];
        let tail = you.body[you.body.len() - 1];
        for c in you.body.iter().map(|p| p.index(n)) {
            board.or(c, YOU);
        }
        board.or(head.index(n), HEAD);
        board.or(tail.index(n), TAIL);

        let foe = &snakes[1];
        let head = foe.body[0];
        let tail = foe.body[foe.body.len() - 1];
        for c in foe.body.iter().map(|p| p.index(n)) {
            board.or(c, FOE);
        }
        board.or(head.index(n), HEAD);
        board.or(tail.index(n), TAIL);

        let root = Node::new(board);
        Self { root }
    }
}
