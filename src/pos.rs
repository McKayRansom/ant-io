use std::ops::{Add, Sub};



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

pub mod dirs {
    use macroquad::prelude::rand;

    use super::Pos;

    pub const UP: Pos = Pos::new(0, -1);
    pub const DOWN: Pos = Pos::new(0, 1);
    pub const RIGHT: Pos = Pos::new(1, 0);
    pub const LEFT: Pos = Pos::new(-1, 0);
    pub const NONE: Pos = Pos::new(0, 0);

    pub const ALL: &[Pos] = &[UP, DOWN, LEFT, RIGHT];
    pub const _ALL_REV: &[Pos] = &[RIGHT, LEFT, DOWN, UP];

    pub fn rand() -> Pos {
        ALL[rand::rand() as usize % ALL.len()]
    }

    pub fn rotate_right(pos: Pos) -> Pos {
        match pos {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => panic!("Invalid dir: {:?}", pos),
        }
    }

    pub fn rotate_left(pos: Pos) -> Pos {
        match pos {
            UP => LEFT,
            RIGHT => UP,
            DOWN => RIGHT,
            LEFT => DOWN,
            _ => panic!("Invalid dir: {:?}", pos),
        }
    }
}

impl Pos {
    pub const fn new(x: i16, y: i16) -> Self {
        Self {x, y}
    }
}

impl From<(i16, i16)> for Pos {
    fn from(value: (i16, i16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub<Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
