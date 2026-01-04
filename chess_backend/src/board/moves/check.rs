use crate::bit_board::{File, Rank};
use crate::board::PieceType;
use crate::ray::Direction::*;
use crate::{
    bit_board::{Offset, Position, Square},
    board::{Board, FenNotation, Piece, KNIGHT_OFFSETS},
    ray::Ray,
};

const ADJACENT_OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Board {
    fn get_attack_squares(&self, square: Square, enemy_color: Piece) -> Vec<Square> {
        let mut out = Vec::with_capacity(8 * 7 + 8 + 8 + 2);
        // rays
        for dir in [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ] {
            for s in Ray::new(square, dir) {
                if let Some(p) = self.get_cached_piece_at(s) {
                    if p.is_same_color(&enemy_color) {
                        out.push(s);
                    }
                    break;
                }
            }
        }

        for off in KNIGHT_OFFSETS {
            let s = square.offset(off);
            if let Some(sout) = s {
                let (df, dr) = square.to_position().diff(&sout.to_position());
                if !((df.abs() == 1 && dr.abs() == 2) || (df.abs() == 2 && dr.abs() == 1)) {
                    continue;
                }
                if let Some(p) = self.get_cached_piece_at(sout) {
                    if p.is_same_color(&enemy_color) && p.is_type(PieceType::Knight) {
                        out.push(sout);
                    }
                }
            }
        }

        out
    }
    // with regards to enemy_color, piece type does not matter
    pub(crate) fn is_check(&self, pos: &Position, enemy_color: Piece) -> bool {
        let enemy_pieces_in_attack_squares =
            self.get_attack_squares(pos.to_square(), enemy_color);

        dbg!(&enemy_pieces_in_attack_squares);

        for square in enemy_pieces_in_attack_squares {
            let piece = self
                .get_cached_piece_at(square)
                .expect("Already determined to be there");

            if piece.attacks(&square.to_position(), pos) {
                return true;
            }
        }

        false
    }
}
