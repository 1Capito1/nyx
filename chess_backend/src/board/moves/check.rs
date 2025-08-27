use crate::{bit_board::{Offset, Position}, board::{Board, FenNotation, Piece, KNIGHT_OFFSETS}, ray::Ray};
use crate::{ray::Direction::*};

const ADJACENT_OFFSETS: i8 = [
    (-1, -1), (0, -1), (1, -1),
    (0, -1)           ,(0, 1),
    (1, -1), (1, 0),   (1, 1)
];

impl Board {
    fn get_attack_squares(&self, square: Square, enemy_color: FenNotation) -> Vec<Square> {
        let mut out = Vec::with_capacity(8*7 + 8 + 8 + 2);
        // rays
        for dir in [North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest] {
            for s in Ray::new(square, dir) {
                out.push(s);
            }
        }

        for off in KNIGHT_OFFSETS {
            let s = square.offset(off);
            if let Some(sout) = s {
                out.push(sout);
            }
        }
    }
    // with regards to enemy_color, piece type does not matter
    pub(crate) fn is_check(&self, square: Square, enemy_color: Piece) -> Option<Position> {
        let attack_squares = self.get_attack_squares(square, enemy_color);
        let pawn_attack_squares = match enemy_color {
            Piece::White(_) => [(1, -1), (1, 1)],
            Piece::Black(_) => [(-1, 1), (-1, -1)],
        }
    }
}
