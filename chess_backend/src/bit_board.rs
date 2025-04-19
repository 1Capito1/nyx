use derive_more::{Add, AddAssign, BitOr, Deref, Display, From, Into, Mul, Sub};

pub trait CheckedSub {
    type Output;

    fn checked_sub(&self, rhs: Self) -> Option<Self::Output>;
}

pub enum FileChars {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl FileChars {
    pub fn to_string(&self) -> String {
        match self {
            FileChars::A => "A".to_string(),
            FileChars::B => "B".to_string(),
            FileChars::C => "C".to_string(),
            FileChars::D => "D".to_string(),
            FileChars::E => "E".to_string(),
            FileChars::F => "F".to_string(),
            FileChars::G => "G".to_string(),
            FileChars::H => "H".to_string(),
        }
    }

    pub fn from_file(f: File) -> Self {
        match f {
            File(0) => Self::A,
            File(1) => Self::B,
            File(2) => Self::C,
            File(3) => Self::D,
            File(4) => Self::E,
            File(5) => Self::F,
            File(6) => Self::G,
            File(7) => Self::H,
            _ => panic!("Invalid file: {f}"),
        }
    }
}

#[derive(Default, Clone, Copy, BitOr)]
pub(crate) struct BitBoard(u64);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Add, Mul, Display, From, Into, Sub, AddAssign,)]
pub(crate) struct File(pub u8);

pub trait Offset {
    type Output;

    fn offset(&self, delta: i8) -> Option<Self::Output>;
}

impl Offset for File {
    type Output = File;
    fn offset(&self, delta: i8) -> Option<Self::Output> {
        let base = self.0 as i8 + delta;
        if (0..=7).contains(&base) {
            Some(File(base as u8))
        } else {
            None
        }
    }
}

impl CheckedSub for File {
    type Output = Self;
    fn checked_sub(&self, rhs: Self) -> Option<Self::Output> {
        self.0.checked_sub(rhs.0).map(|t| File(t))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Add, Mul, Display, From, Into, Sub)]
pub(crate) struct Rank(pub u8);

impl CheckedSub for Rank {
    type Output = Self;
    fn checked_sub(&self, rhs: Self) -> Option<Self::Output> {
        self.0.checked_sub(rhs.0).map(|t| Rank(t))
    }
}

impl Offset for Rank {
    type Output = Rank;
    fn offset(&self, delta: i8) -> Option<Self::Output> {
        let base = self.0 as i8 + delta;
        if (0..=7).contains(&base) {
            Some(Rank(base as u8))
        } else {
            None
        }
    }
}

impl Rank {
    pub(crate) fn diff(&self, rhs: Rank) -> Rank {
        Rank(self.0.abs_diff(rhs.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Position {
    file: File,
    rank: Rank,
}

#[derive(Clone, Copy, Debug, PartialEq, Deref)]
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
        Position::new(File(**self % 8), Rank(**self / 8))
    }
}

impl Square {}


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
        if self.file >= File(8) || self.rank >= Rank(8) {
            panic!("file: {}, rank: {}", self.file, self.rank)
        }
        self.rank.0 * 8 + self.file.0
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
            print!("{}  ", FileChars::from_file(File(rank)).to_string());
            // start from rank 8
            for file in 0..8 {
                let i = rank as usize * WIDTH + file;
                print!("{} ", rep[i]);
            }
            println!()
        }
        println!("\n   1 2 3 4 5 6 7 8 ");
    }
}
