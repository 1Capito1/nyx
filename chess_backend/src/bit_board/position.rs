use super::rank::Rank;
use super::{File, FileChars, Square};
use derive_more::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("{file}, {rank}")]
pub(crate) struct Position {
    file: File,
    rank: Rank,
}

impl Position {
    pub(crate) fn new(file: File, rank: Rank) -> Self {
        if file >= File(8) || rank >= Rank(8) {
            panic!("Invalid position: File {file}, Rank: {rank}");
        }
        Self { file, rank }
    }

    pub(crate) fn from_notation(file_letter: FileChars, rank_num: u8) -> Self {
        Position::new(File(file_letter as u8), Rank(rank_num - 1))
    }

    pub(crate) fn file(&self) -> File {
        self.file
    }

    fn file_mut(&mut self) -> &mut File {
        &mut self.file
    }

    pub(crate) fn rank(&self) -> Rank {
        self.rank
    }

    fn rank_mut(&mut self) -> &mut Rank {
        &mut self.rank
    }

    pub(crate) fn set_file(&mut self, file: u8) {
        if file >= 8 {
            panic!("Invalid file: {file}");
        }
        *self.file_mut() = File(file);
    }
    pub(crate) fn set_rank(&mut self, rank: u8) {
        if rank >= 8 {
            panic!("Invalid rank: {rank}");
        }
        *self.rank_mut() = Rank(rank);
    }

    pub(crate) fn to_square(self) -> Square {
        Square::new(self.square_num())
    }

    pub(crate) fn bitboard_mask(&self) -> u64 {
        let shift = self.square_num();
        println!("SHIFT AMOUNT: {shift}");
        1u64 << shift
    }

    pub(crate) fn square_num(&self) -> u8 {
        if self.file >= File(8) || self.rank >= Rank(8) {
            panic!("file: {}, rank: {}", self.file, self.rank)
        }
        self.rank.0 * 8 + self.file.0
    }
}
