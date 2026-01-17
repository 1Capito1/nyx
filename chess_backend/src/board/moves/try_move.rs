use crate::{
    bit_board::Position,
    board::{Board, Color, Move, MoveBuilder, PieceType, UndoMove},
    errors::MoveError,
};

impl Board {
    pub(crate) fn try_move(
        &mut self,
        from: impl Into<Position>,
        to: impl Into<Position>,
    ) -> Result<UndoMove, MoveError> {
        let to: Position = to.into();
        let from: Position = from.into();
        let moving_piece = self
            .get_cached_piece_at(from)
            .ok_or(MoveError::PieceNotFound(from))?;

        let move_ident = Move::builder()
            .move_to(to.to_square())
            .move_from(from.to_square())
            .build();

        let handler: fn(&mut Self, Move) -> Result<UndoMove, MoveError> =
            match moving_piece.get_type() {
                PieceType::Pawn => Self::move_pawn,
                PieceType::Rook => Self::move_rook,
                PieceType::Knight => Self::move_knight,
                PieceType::Bishop => Self::move_bishop,
                PieceType::Queen => Self::move_queen,
                PieceType::King => Self::move_king,
            };

        let mut ret = handler(self, move_ident);
        if let Ok(undo) = ret.as_mut() {
            undo.set_side(self.side_to_move);
            self.side_to_move = match self.side_to_move {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };

            self.cache_set(to.into(), Some(moving_piece));
            self.cache_set(from.into(), None);
        }
        return ret;
    }

    pub(crate) fn undo_move(&mut self, undo: UndoMove) {
        for change in undo.get_changes() {
            let sq = change.at();
            let idx = sq.0 as usize;

            let cur = self.board_rep()[idx];

            if let Some(piece) = cur {
                self.match_board(piece).clear_bit(sq);
            }

            let before = change.piece_before();
            self.board_rep_mut()[idx] = before;

            if let Some(piece) = before {
                self.match_board(piece).set_bit(sq);
            }
        }
        self.side_to_move = undo.get_side();
    }
}
