use crate::bit_board::FileChars;

use super::rank::Rank;
use super::{File, Offset, Position};
use derive_more::Deref;

#[derive(Clone, Copy, Debug, PartialEq, Deref)]
pub struct Square(pub u8);

impl Square {
    pub fn new(position: u8) -> Self {
        if position >= 64 {
            panic!("position should not be > 64: {position}");
        }
        Self(position)
    }

    pub(crate) fn from_position(position: &Position) -> Self {
        let value = position.square_num();
        Square::new(value)
    }

    pub(crate) fn from_notation(file_letter: FileChars, rank_num: u8) -> Self {
        assert!(&(1..=8).contains(&rank_num), "from_notation OOB");
        Position::new(File(file_letter as u8), Rank(rank_num - 1)).to_square()
    }

    pub(crate) fn to_position(self) -> Position {
        Position::new(File(*self % 8), Rank(*self / 8))
    }
}

impl Offset for Square {
    type Output = Self;

    fn offset(&self, delta: i8) -> Option<Self::Output> {
        let idx = **self as i8 + delta;
        if (0..64).contains(&idx) {
            return Some(Square::new(idx as u8));
        }
        None
    }
}
