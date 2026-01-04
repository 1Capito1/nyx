use crate::{
    bit_board::Position,
    board::{Board, Move, MoveBuilder, PieceType, UndoMove},
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

        handler(self, move_ident)
    }

    pub(crate) fn undo_move(&mut self, undo: UndoMove) {
        for change in undo.get_changes() {
            if let Some(piece) = change.piece_before() {
                let board = self.match_board(piece);
                board.set_bit(change.at());
            }
        }
    }
}
