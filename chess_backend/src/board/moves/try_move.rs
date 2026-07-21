use crate::{
    bit_board::{Position, Square},
    board::{Board, Color, Move, MoveBuilder, Piece, PieceType, UndoMove},
    errors::MoveError,
};

use super::SpecialMove;

impl Board {
    pub(crate) fn try_move(&mut self, mv: &Move) -> Result<UndoMove, MoveError> {
        let to: Position = mv.move_to().into();
        let from: Position = mv.move_from().into();
        let moving_piece = self
            .get_cached_piece_at(from)
            .ok_or(MoveError::PieceNotFound(from))?;

        if let Some(piece_to) = self.get_cached_piece_at(to) {
            if piece_to.is_same_color(&moving_piece) {
                return Err(MoveError::CaptureSameColor(to));
            }
            if piece_to.is_type(PieceType::King) {
                return Err(MoveError::CaptureKing(from, to));
            }
        }

        let handler: fn(&mut Self, &Move) -> Result<UndoMove, MoveError> =
            match moving_piece.get_type() {
                PieceType::Pawn => Self::move_pawn,
                PieceType::Rook => Self::move_rook,
                PieceType::Knight => Self::move_knight,
                PieceType::Bishop => Self::move_bishop,
                PieceType::Queen => Self::move_queen,
                PieceType::King => Self::move_king,
            };

        let mut ret = handler(self, mv);
        if let Ok(undo) = ret.as_mut() {
            self.cache_set(to.into(), Some(moving_piece));
            self.cache_set(from.into(), None);
            undo.set_side(self.side_to_move);
            let king_pos = match self.side_to_move {
                Color::White => self.white_king.iter_squares().nth(0).map_or_else(
                    || {
                        println!("KING MISSING BOARD");
                        self.pretty_print();
                        panic!("White King not found");
                    },
                    Square::to_position,
                ),
                Color::Black => self.black_king.iter_squares().nth(0).map_or_else(
                    || {
                        println!("KING MISSING BOARD");
                        self.pretty_print();
                        println!("BEFORE BOARD");
                        self.undo_move(undo);
                        self.pretty_print();
                        panic!("Black King not found");
                    },
                    Square::to_position,
                ),
            };
            let enemy_color = match self.side_to_move {
                Color::White => Piece::Black(PieceType::Pawn),
                Color::Black => Piece::White(PieceType::Pawn),
            };
            if self.is_check(&king_pos, enemy_color) {
                self.undo_move(undo);
                return Err(MoveError::KingLeftInCheck(from, to));
            }
            self.side_to_move = match self.side_to_move {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
        }
        ret
    }

    pub(crate) fn undo_move(&mut self, undo: &UndoMove) {
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
        if let Some(special) = undo.special() {
            match special {
                SpecialMove::Promotion(piece) => {}
                SpecialMove::EnPassant(square) => self.en_passant_square = *square,
                SpecialMove::Castling { rook_to, rook_from } => {}
            }
        }
        self.side_to_move = undo.get_side();
    }
}
