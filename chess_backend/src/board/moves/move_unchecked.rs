use crate::Piece;
use crate::PieceType;
use crate::PieceType::Pawn;
use crate::bit_board::Offset;
use crate::bit_board::Position;
use crate::bit_board::Rank;
use crate::bit_board::Square;
use crate::board::{Move, UndoMove};

use crate::board::Board;

impl Board {
    pub fn move_piece_unchecked(&mut self, move_info: &Move) {
        let square_from = move_info.move_from();
        let square_to = move_info.move_to();
        let position_from = square_from.to_position();
        let position_to = square_to.to_position();
        let moved_piece = self.get_cached_piece_at(position_from);
        let piece_at = self.get_cached_piece_at(position_to);

        if let Some(piece) = moved_piece {
            if let Some(piece_at) = piece_at {
                let board_at = self.match_board(piece_at);
                board_at.clear_bit(square_to);
            }
            let board = self.match_board(piece);
            board.clear_bit(square_from);
            board.set_bit(square_to);
            self.cache_set(square_from, None);
            self.cache_set(square_to, Some(piece));
        } else {
            self.pretty_print();
            panic!("Piece not found: {}", position_from);
        }
    }
}
