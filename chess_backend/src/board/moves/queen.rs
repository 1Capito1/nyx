use crate::board::PieceType::Queen;
use crate::board::{Board, UndoChange};
use crate::errors::MoveError;
use crate::ray::{Direction, Ray};
use crate::Position;

use super::{is_bishop_move, is_rook_move, Move, UndoMove};

impl Board {
    pub fn move_queen(&mut self, move_info: Move) -> Result<UndoMove, MoveError> {
        let current_pos = move_info.move_from().to_position();
        let pos_to = move_info.move_to().to_position();

        let current_piece = self
            .get_cached_piece_at(current_pos)
            .ok_or(MoveError::PieceNotFound(current_pos))?;
        let piece_at = self.get_cached_piece_at(pos_to);
        if !current_piece.is_type(Queen) {
            return Err(MoveError::IncorrectPiece(Queen, current_piece.get_type()));
        }
        if piece_at.is_some_and(|p| current_piece.is_same_color(&p)) {
            return Err(MoveError::CaptureSameColor(pos_to));
        }

        let dir = Direction::get_direction(
            &current_pos,
            &pos_to,
            is_queen_move,
            MoveError::InvalidMove(Queen, current_pos, pos_to),
        )?;

        if let Some(blocking_piece) = Ray::new(current_pos.to_square(), dir)
            .take_while(|x| *x != pos_to.to_square())
            .map(|s| s.to_position())
            .find(|pos| self.get_cached_piece_at(*pos).is_some())
        {
            return Err(MoveError::PieceBlockingMovement(
                current_pos,
                blocking_piece,
            ));
        }
        self.move_piece_unchecked(&move_info);
        Ok(UndoMove::new(
            vec![
                UndoChange::new(current_pos.to_square(), Some(current_piece)),
                UndoChange::new(pos_to.to_square(), piece_at),
            ],
            None,
        ))
    }
}

fn is_queen_move(from: &Position, to: &Position) -> bool {
    is_rook_move(from, to) || is_bishop_move(from, to)
}
