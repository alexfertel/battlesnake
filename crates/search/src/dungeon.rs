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
        let in_range = p.x >= 0
            && p.x < self.board.len() as i64
            && p.y >= 0
            && p.y < self.board[0].len() as i64;

        if !in_range {
            return false;
        }

        let px = p.x as usize;
        let py = p.y as usize;

        self.board[px][py] != '*'
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

    #[allow(non_camel_case_types)]
    type dfs = Dfs<Point>;
    #[allow(non_camel_case_types)]
    type bfs = Bfs<Point>;

    fn position(needle: char, dungeon: &Vec<Vec<char>>) -> Option<Point> {
        for i in 0..dungeon.len() {
            for j in 0..dungeon[0].len() {
                if dungeon[i][j] == needle {
                    return Some(Point::new(i as i64, j as i64));
                }
            }
        }

        None
    }

    macro_rules! assert_strategies {
        ($dungeon:ident, $needle:literal, $($strategy:ty),* $(,)?) => {
            $(
                let p = Dungeon::new($dungeon.clone(), 'S', $needle);
                let s = <$strategy>::default();
                let expected_end = position($needle, &p.board).unwrap();
                let actual_end = search(p, s);
                assert_eq!(expected_end, actual_end);
            )*
        }
    }

    macro_rules! dungeons {
        ($($name:ident => $dungeon:literal),* $(,)?) => {
            $(paste::paste! {
                #[test]
                fn [<$name _dungeon>]() {
                    let d = $dungeon;
                    let board: Vec<Vec<char>> = d
                        .trim()
                        .lines()
                        .map(str::trim)
                        .map(|l| l.chars().skip_while(|c| c.is_whitespace()).collect())
                        .collect();

                    assert_strategies!(board, 'E', dfs, bfs);
                }
            })*
        };
    }

    dungeons!(
    simple => r#" ..........
                  ........E.
                  ..........
                  .S........
                  .........."#,
    walled => r#" ..........
                  **.*****.E
                  .*.....***
                  .*****....
                  ..*..****.
                  .S........"#,
    );
}
