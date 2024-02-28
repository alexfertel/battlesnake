use crate::{direction::Direction, point::Point, traits::Problem};

pub struct Dungeon {
    pub start: char,
    pub board: Vec<Vec<char>>,
    pub end: char,

    visited: Vec<Vec<bool>>,
}

impl Dungeon {
    pub fn new(board: Vec<Vec<char>>, start: char, end: char) -> Self {
        let visited = vec![vec![false; board[0].len()]; board.len()];

        Self {
            start,
            board,
            end,
            visited,
        }
    }

    pub fn valid_successor(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.board.len() as i64 && p.y >= 0 && p.y < self.board[0].len() as i64
    }
}

impl Problem for Dungeon {
    type State = Point;

    fn get_start(&self) -> Self::State {
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                if self.board[i][j] == self.start {
                    return Point::new(i as i64, j as i64);
                }
            }
        }

        unreachable!("the board should have a starting position")
    }

    fn get_successors(&mut self, state: &Self::State) -> Vec<Self::State> {
        let mut successors = Vec::new();
        for d in Direction::all().iter().map(Direction::vector) {
            let nx = state.x + d.x;
            let ny = state.y + d.y;

            let s = Point::new(nx, ny);
            if !self.valid_successor(&s) {
                continue;
            }

            let nx = s.x as usize;
            let ny = s.y as usize;
            if self.visited[nx][ny] {
                continue;
            }

            self.visited[nx][ny] = true;
            successors.push(s);
        }

        successors
    }

    fn is_end(&self, state: &Self::State) -> bool {
        let x = state.x as usize;
        let y = state.y as usize;

        self.board[x][y] == self.end
    }
}

#[cfg(test)]
mod tests {
    use crate::{bfs::Bfs, dfs::Dfs, point::Point, search};

    use super::Dungeon;

    macro_rules! simple_dungeon_with_strategies {
        ($($strategy:ty),* $(,)?) => {
            paste::paste! {$(#[test]
            #[allow(non_snake_case)]
            fn [< simple_dungeon_ $strategy >]() {
                let dungeon = r#"
                    ..........
                    ........E.
                    ..........
                    .S........
                    ..........
                    "#;

                let board: Vec<Vec<char>> = dungeon
                    .trim()
                    .lines()
                    .map(str::trim)
                    .map(|l| l.chars().skip_while(|c| c.is_whitespace()).collect())
                    .collect();
                let p = Dungeon::new(board, 'S', 'E');
                let s = <$strategy>::default();

                let expected_end = Point::new(1, 8);
                let actual_end = search(p, s);
                assert_eq!(expected_end, actual_end);
            })*
            }
        };
    }

    type DungeonDfs = Dfs<Point>;
    type DungeonBfs = Bfs<Point>;

    simple_dungeon_with_strategies!(DungeonDfs, DungeonBfs,);
}
