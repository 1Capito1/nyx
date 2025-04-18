use std::ops::{BitOr, Deref};

#[derive(Default, Clone, Copy)]
pub(crate) struct BitBoard(u64);

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

pub(crate) struct Position {
    file: u8,
    rank: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square(u8);

impl Square {
    pub fn new(position: u8) -> Self {
        if position >= 64 {
            panic!("position should not be > 64: {position}");
        }
        return Self(position);
    }

    pub(crate) fn from_position(position: &Position) -> Self {
        let value = position.square_num();
        Square::new(value)
    }
    pub(crate) fn to_position(&self) -> Position {
        Position::new(**self % 8, **self / 8)
    }
}

impl Square {}

impl Deref for Square {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Position {
    pub(crate) fn new(file: u8, rank: u8) -> Self {
        if file >= 8 || rank >= 8 {
            panic!("Invalid position: File {file}, Rank: {rank}");
        }
        Self { file, rank }
    }

    pub(crate) fn file(&self) -> u8 {
        self.file
    }

    fn file_mut(&mut self) -> &mut u8 {
        &mut self.file
    }

    pub(crate) fn rank(&self) -> u8 {
        self.rank
    }

    fn rank_mut(&mut self) -> &mut u8 {
        &mut self.rank
    }


    pub(crate) fn set_file(&mut self, file: u8) {
        if file >= 8 {
            panic!("Invalid file: {file}");
        }
        *self.file_mut() = file;
    }
    pub(crate) fn set_rank(&mut self, rank: u8) {
        if rank >= 8 {
            panic!("Invalid rank: {rank}");
        }
        *self.rank_mut() = rank;
    }

    pub(crate) fn to_square(&self) -> Square {
        Square::new(self.square_num().try_into().unwrap())
    }

    pub(crate) fn bitboard_mask(&self) -> u64 {
        let shift = self.square_num();
        println!("SHIFT AMOUNT: {shift}");
        let mask = 1u64 << shift;
        return mask;
    }

    pub(crate) fn square_num(&self) -> u8 {
        if self.file >= 8 || self.rank >= 8 {
            panic!("file: {}, rank: {}", self.file, self.rank)
        }
        println!("file: {}, rank: {}", self.file, self.rank);
        self.rank * 8 + self.file
    }
}

impl BitBoard {
    pub fn iter_set_bits(self) -> impl Iterator<Item = u8> {
        let mut bits = self.0;
        std::iter::from_fn(move || {
            if bits == 0 {
                return None;
            }
            let idx = bits.trailing_zeros() as u8;
            bits &= bits - 1;
            Some(idx)
        })
    }
    pub fn bits(&self) -> &u64 {
        return &self.0;
    }
    pub fn bits_mut(&mut self) -> &mut u64 {
        return &mut self.0;
    }
    pub fn place(&mut self, position: &Position) {
        let square = position.to_square();
        self.set_bit(square);
    }
    pub fn set_bit(&mut self, square: Square) {
        debug_assert!(*square < 64);
        self.0 |= 1u64 << *square;
    }
    pub fn clear_bit(&mut self, square: Square) {
        self.0 &= !(1 << *square);
    }
    pub fn is_set(&self, square: u8) -> bool {
        (self.bits() >> square) & 1 != 0
    }

    pub fn print_bitboard_rep(rep: &[char; 64]) {
        const WIDTH: usize = 8;
        for rank in (0..8).rev() {
            // start from rank 8
            for file in 0..8 {
                let i = rank * WIDTH + file;
                print!("{} ", rep[i]);
            }
            println!();
        }
    }
}
