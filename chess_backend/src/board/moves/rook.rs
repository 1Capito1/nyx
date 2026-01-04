use crate::board::{piece, Board, PieceType, UndoChange};
use crate::errors::MoveError;
use crate::ray::{self, Direction, Ray};
use crate::PieceType::Rook;
use crate::Position;

use super::{Move, UndoMove};

const ROOK_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Board {
    pub fn move_rook(&mut self, move_info: Move) -> Result<UndoMove, MoveError> {
        let current_pos = move_info.move_from().to_position();
        let pos_to = move_info.move_to().to_position();

        let current_piece = self
            .get_cached_piece_at(current_pos)
            .ok_or(MoveError::PieceNotFound(current_pos))?;
        let piece_at = self.get_cached_piece_at(pos_to);

        if !current_piece.is_type(Rook) {
            return Err(MoveError::IncorrectPiece(Rook, current_piece.get_type()));
        }

        if piece_at.is_some_and(|p| current_piece.is_same_color(&p)) {
            return Err(MoveError::CaptureSameColor(pos_to));
        }

        let dir = Direction::get_direction(
            &current_pos,
            &pos_to,
            is_rook_move,
            MoveError::InvalidMove(Rook, current_pos, pos_to),
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

        let undo_change = vec![
            UndoChange::new(move_info.move_from(), Some(current_piece)),
            UndoChange::new(move_info.move_to(), piece_at),
        ];

        Ok(UndoMove::new(undo_change, None))
    }
}

pub(crate) fn is_rook_move(from: &Position, to: &Position) -> bool {
    let rank_diff = from.rank().0 as i8 - to.rank().0 as i8;
    let file_diff = from.file().0 as i8 - to.file().0 as i8;

    let rank_change = rank_diff.abs() > 0;
    let file_change = file_diff.abs() > 0;

    rank_change ^ file_change
}
