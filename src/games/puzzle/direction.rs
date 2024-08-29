use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left, Right, Up, Down, LeftUp, LeftDown, RightUp, RightDown
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DirectionSet(u8);

impl DirectionSet {
    const LEFT: u8 = 1 << 0;
    const RIGHT: u8 = 1 << 1;
    const UP: u8 = 1 << 2;
    const DOWN: u8 = 1 << 3;
    const LEFT_UP: u8 = 1 << 4;
    const LEFT_DOWN: u8 = 1 << 5;
    const RIGHT_UP: u8 = 1 << 6;
    const RIGHT_DOWN: u8 = 1 << 7;

    fn new() -> Self {
        DirectionSet(0)
    }

    fn from_directions(directions: &[Direction]) -> Self {
        let mut set = 0;
        for &dir in directions {
            set |= match dir {
                Direction::Left => Self::LEFT,
                Direction::Right => Self::RIGHT,
                Direction::Up => Self::UP,
                Direction::Down => Self::DOWN,
                Direction::LeftUp => Self::LEFT_UP,
                Direction::LeftDown => Self::LEFT_DOWN,
                Direction::RightUp => Self::RIGHT_UP,
                Direction::RightDown => Self::RIGHT_DOWN,
            };
        }
        DirectionSet(set)
    }

    fn contains(&self, direction: Direction) -> bool {
        let mask = match direction {
            Direction::Left => Self::LEFT,
            Direction::Right => Self::RIGHT,
            Direction::Up => Self::UP,
            Direction::Down => Self::DOWN,
            Direction::LeftUp => Self::LEFT_UP,
            Direction::LeftDown => Self::LEFT_DOWN,
            Direction::RightUp => Self::RIGHT_UP,
            Direction::RightDown => Self::RIGHT_DOWN,
        };
        self.0 & mask != 0
    }
}

impl BitAnd for DirectionSet {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        DirectionSet(self.0 & rhs.0)
    }
}

impl BitOr for DirectionSet {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        DirectionSet(self.0 | rhs.0)
    }
}

impl BitXor for DirectionSet {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        DirectionSet(self.0 ^ rhs.0)
    }
}

impl Not for DirectionSet {
    type Output = Self;
    fn not(self) -> Self::Output {
        DirectionSet(!self.0)
    }
}