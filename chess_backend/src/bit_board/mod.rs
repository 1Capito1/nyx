mod bitboard;
mod square;
mod file_chars;
mod file;
mod rank;
mod position;

pub use bitboard::*;
pub use square::*;
pub use file_chars::*;
pub use file::*;
pub use position::*;
pub use rank::*;

pub trait CheckedSub {
    type Output;

    fn checked_sub(&self, rhs: Self) -> Option<Self::Output>;
}

pub trait Offset {
    type Output;

    fn offset(&self, delta: i8) -> Option<Self::Output>;
}
