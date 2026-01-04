use crate::bit_board::Square;
use crate::board::Board;

use super::Direction;

pub struct Ray {
    current: i8,
    dir: Direction,
    start_file: u8,
}

impl Ray {
    pub fn new(from: Square, dir: Direction) -> Self {
        Self {
            current: *from as i8,
            dir,
            start_file: *from % 8,
        }
    }
}

impl Iterator for Ray {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        // 1) decode current index into file/rank
        let idx = self.current as u8;
        let file = (idx % 8) as i8; // 0..7 = files a..h
        let rank = (idx / 8) as i8; // 0..7 = ranks 8..1 depending on your convention

        // 2) get the small (df,dr) step for this direction
        let (df, dr) = self.dir.delta();

        // 3) compute the target file/rank
        let new_file = file + df;
        let new_rank = rank + dr;

        // 4) if it’s off‐board, stop
        if !(0..8).contains(&new_file) || !(0..8).contains(&new_rank) {
            return None;
        }

        // 5) convert back to 0..63 index, advance, and return
        let new_idx = (new_rank * 8 + new_file) as u8;
        self.current = new_idx as i8;
        Some(Square(new_idx))
    }
}
