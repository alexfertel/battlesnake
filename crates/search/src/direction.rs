use crate::point::Point;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Up, Self::Left, Self::Down, Self::Right]
    }

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn vector(&self) -> Point {
        match self {
            Self::Up => Point { x: 0, y: -1 },
            Self::Left => Point { x: -1, y: 0 },
            Self::Down => Point { x: 0, y: 1 },
            Self::Right => Point { x: 1, y: 0 },
        }
    }
}
