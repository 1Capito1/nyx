use crate::bit_board::Square;
use crate::board::Board;
use crate::board::Piece;
use crate::board::UndoChange;
use crate::board::UndoMove;
use crate::Position;

impl Board {
    pub fn get_cached_piece_at(&self, position: impl Into<Position>) -> Option<Piece> {
        let pos = position.into();
        let square = Square::from_position(&pos);
        self.board_rep()[*square as usize]
    }

    pub fn build_cache(&mut self) {
        let pieces = self.get_bitboard_pieces();
        let board_rep = self.board_rep_mut();
        *board_rep = [None; 64];

        for (bitboard, piece) in pieces {
            for square in bitboard.iter_set_bits() {
                board_rep[square as usize] = Some(piece);
            }
        }
    }

    pub fn cache_set(&mut self, sq: Square, val: Option<Piece>) {
        self.board_rep_mut()[sq.0 as usize] = val;
    }

    #[deprecated = "only try_move and undo should update cache, use cache_set now"]
    pub fn update_cache(&mut self, um: &UndoMove) {
        for change in um.get_changes() {
            self.update_cache_atomic(change);
        }
    }

    pub fn update_cache_atomic(&mut self, change: &UndoChange) {
        self.board_rep_mut()[change.at().0 as usize] = change.piece_before();
    }

    pub fn get_cache(&self) -> &[Option<Piece>] {
        self.board_rep()
    }
}
