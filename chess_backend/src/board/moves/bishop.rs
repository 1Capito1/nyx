use crate::board::Board;
use crate::board::PieceType::Bishop;
use crate::errors::MoveError;
use crate::ray::{Direction, Ray};
use crate::Position;

use super::{Move, UndoMove};

impl Board {
    pub fn move_bishop(&mut self, move_info: Move) -> Result<UndoMove, MoveError> {
        let current_pos = move_info.move_from().to_position();
        let pos_to = move_info.move_to().to_position();


        let current_piece = self.get_cached_piece_at(&current_pos)
            .ok_or(MoveError::PieceNotFound(current_pos))?;
        let piece_at = self.get_cached_piece_at(&pos_to);

        if !current_piece.is_type(Bishop) {
            return Err(MoveError::IncorrectPiece(Bishop, current_piece.get_type()));
        }

        if piece_at.is_some_and(|p| current_piece.is_same_color(&p)) {
            return Err(MoveError::CaptureSameColor(pos_to));
        }

        let dir = Direction::get_direction(
            &current_pos,
            &pos_to,
            is_bishop_move,
            MoveError::InvalidMove(Bishop, current_pos, pos_to
        ))?;

        println!("{dir:?}");

        if let Some(blocking_piece) = Ray::new(current_pos.to_square(), dir)
            .take_while(|x| *x != pos_to.to_square())
            .inspect(|s| println!("{s:?}"))
            .map(|s| s.to_position())
            .find(|pos| self.get_cached_piece_at(pos).is_some()) 
        {
            return Err(MoveError::PieceBlockingMovement(current_pos, blocking_piece));
        }
        Ok(self.move_piece_unchecked(&move_info))
    }

}
pub(crate) fn is_bishop_move(from: &Position, to: &Position) -> bool {
    let rank_diff = from.rank().0 as i8 - to.rank().0 as i8;
    let file_diff = from.file().0 as i8 - to.file().0 as i8;

    rank_diff.abs() == file_diff.abs()
}
