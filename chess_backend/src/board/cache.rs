use crate::bit_board::Square;
use crate::board::Board;
use crate::board::Piece;
use crate::Position;

impl Board {
    pub fn get_cached_piece_at(&self, position: impl Into<Position>) -> Option<Piece> {
        let pos = position.into();
        let square = Square::from_position(&pos);
        self.board_rep()[*square as usize]
    }

    pub fn update_cache(&mut self) {
        let pieces = self.get_bitboard_pieces();
        let board_rep = self.board_rep_mut();
        *board_rep = [None; 64];

        for (bitboard, piece) in pieces {
            for square in bitboard.iter_set_bits() {
                board_rep[square as usize] = Some(piece);
            }
        }
    }

    pub fn get_cache(&self) -> &[Option<Piece>] {
        self.board_rep()
    }
}
