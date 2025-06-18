use crate::board::Board;
use crate::board::PieceType::King;
use crate::errors::MoveError;
use crate::Position;

use super::{Move, UndoMove};

impl Board {
    fn move_king(&mut self, move_info: Move) -> Result<UndoMove, MoveError> {
        let current_pos = move_info.move_from().to_position();
        let pos_to = move_info.move_to().to_position();

        let current_piece = self
            .get_cached_piece_at(&current_pos)
            .ok_or(MoveError::PieceNotFound(current_pos))?;
        let piece_at = self.get_cached_piece_at(&pos_to);
        if !current_piece.is_type(King) {
            return Err(MoveError::IncorrectPiece(King, current_piece.get_type()));
        }
        if piece_at.is_some_and(|p| current_piece.is_same_color(&p)) {
            return Err(MoveError::CaptureSameColor(pos_to));
        }
        if !Self::is_king_move(&current_pos, &pos_to) {
            return Err(MoveError::InvalidMove(King, current_pos, pos_to));
        }
        Ok(self.move_piece_unchecked(&move_info))
    }

    fn is_king_move(from: &Position, to: &Position) -> bool {
        let f_diff = (from.file().0 as i8 - to.file().0 as i8).abs();
        let r_diff = (from.rank().0 as i8 - to.rank().0 as i8).abs();
        let normal_move = f_diff <= 1 && r_diff <= 1 && (f_diff != 0 || r_diff != 0);

        if !normal_move {}
        normal_move
    }
    fn is_castle(&mut self, from: &Position, to: &Position) -> bool {
        let df = to.file().0 as i8 - from.file().0 as i8;
        let dr = to.rank().0 as i8 - from.rank().0 as i8;

        let piece = self.get_cached_piece_at(from).unwrap();

        let is_king = piece.is_type(King);
        let is_same_rank = dr == 0;
        let is_castling_attempt = is_king && is_same_rank && df.abs() == 2;

        if df == 2 {
            return is_castling_attempt && self.castling_rights.can_castle_kingside(piece);
        }
        is_castling_attempt && self.castling_rights.can_castle_queenside(piece)
    }
}
