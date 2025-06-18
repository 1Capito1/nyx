use derive_more::{Add, AddAssign, BitOr, Deref, Display, From, Into, Mul, Sub};

use crate::bit_board::{File, FileChars};

use super::{Position, Square};

#[derive(Default, Clone, Copy, BitOr)]
pub(crate) struct BitBoard(u64);

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
        &self.0
    }
    pub fn bits_mut(&mut self) -> &mut u64 {
        &mut self.0
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
            print!("{}   ", rank + 1);
            // start from rank 8
            for file in 0..8 {
                let i = rank as usize * WIDTH + file;
                print!("{} ", rep[i]);
            }
            println!()
        }
        println!("\n    A B C D E F G H ");
    }
}
