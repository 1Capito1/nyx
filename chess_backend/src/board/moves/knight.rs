use crate::board::Board;
use crate::board::Piece;
use crate::board::PieceType;
use crate::bit_board::Square;
use crate::bit_board::Offset;
use crate::errors::{MoveError::*, MoveError};


use super::{Move, UndoMove};

pub(crate) const KNIGHT_OFFSETS: [i8; 8] = [15, 17, 6, 10, -6, -10, -15, -17];

impl Board {
    pub fn knight_move(&mut self, move_info: Move) -> Result<UndoMove, MoveError> {
        let from = move_info.move_from();
        let to = move_info.move_to();

        let knight = self.get_cached_piece_at(&from.to_position())
            .ok_or(PieceNotFound(from.to_position()))?;

        if !knight.is_type(PieceType::Knight) {
            return Err(IncorrectPiece(PieceType::Knight, knight.get_type()));
        }

        let delta = (*to as i8) - (*from as i8);

        if KNIGHT_OFFSETS.contains(&delta) {
            let piece_at = self.get_cached_piece_at(&to.to_position());
            if let Some(piece) = piece_at.filter(|p| knight.is_same_color(p)) {
                return Err(CaptureSameColor(to.to_position()));
            }
        }
        else {
            return Err(MoveError::InvalidMove(PieceType::Knight, from.to_position(), to.to_position()));
        }

        Ok(self.move_piece_unchecked(&move_info))
    }
}
