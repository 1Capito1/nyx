use crate::bit_board::{FileChars, Square};
use crate::board::PieceType::King;
use crate::board::{Board, CastlingSide, Color, Piece, UndoChange};
use crate::errors::MoveError;
use crate::ray::{Direction, Ray};
use crate::{Position, Rank};

use super::{Move, UndoMove};

impl Board {
    pub(crate) fn move_king(&mut self, move_info: &Move) -> Result<UndoMove, MoveError> {
        let current_pos = move_info.move_from().to_position();
        let pos_to = move_info.move_to().to_position();

        let current_piece = self
            .get_cached_piece_at(current_pos)
            .ok_or(MoveError::PieceNotFound(current_pos))?;

        let piece_at = self.get_cached_piece_at(pos_to);

        if !current_piece.is_type(King) {
            return Err(MoveError::IncorrectPiece(King, current_piece.get_type()));
        }

        if piece_at.is_some_and(|p| current_piece.is_same_color(&p)) {
            return Err(MoveError::CaptureSameColor(pos_to));
        }

        if !self.is_king_move(&current_pos, &pos_to) {
            return Err(MoveError::InvalidMove(King, current_pos, pos_to));
        }

        let mut undos = Vec::with_capacity(4);
        let mut castle = false;
        if let Some(side) = self.is_castle(&current_pos, &pos_to) {
            castle = true;
            let rank = move_info.move_to().to_position().rank();
            let undo_change = self.castle_rook(current_piece, side, rank);
            undos.extend_from_slice(&undo_change);
        }

        self.move_piece_unchecked(&move_info);

        undos.push(UndoChange::new(move_info.move_from(), Some(current_piece)));
        undos.push(UndoChange::new(move_info.move_to(), piece_at));

        Ok(UndoMove::new(undos, None))
    }

    fn castle_rook(&mut self, king: Piece, side: CastlingSide, rank: Rank) -> [UndoChange; 2] {
        let (rook_from_file, rook_to_file) = match side {
            CastlingSide::KingSide => (FileChars::H, FileChars::F),
            CastlingSide::QueenSide => (FileChars::A, FileChars::D),
        };

        match side {
            CastlingSide::KingSide => match king.get_color() {
                Color::White => self.castling_rights.white_kingside = false,
                Color::Black => self.castling_rights.black_kingside = false,
            },
            CastlingSide::QueenSide => match king.get_color() {
                Color::White => self.castling_rights.white_queenside = false,
                Color::Black => self.castling_rights.black_queenside = false,
            },
        }

        let rank_u8: u8 = u8::from(rank) + 1;
        let rook_from = Position::from_notation(rook_from_file, rank_u8);
        let rook_to = Position::from_notation(rook_to_file, rank_u8);

        let castling_move = Move::builder()
            .move_to(rook_to.to_square())
            .move_from(rook_from.to_square())
            .build();

        let rook = self.get_cached_piece_at(rook_from);

        self.move_piece_unchecked(&castling_move);

        [
            UndoChange::new(rook_to.to_square(), None),
            UndoChange::new(rook_from.to_square(), rook),
        ]
    }

    fn is_normal_move(from: &Position, to: &Position) -> bool {
        let (file_diff, rank_diff) = from.diff(to);
        file_diff.max(rank_diff) == 1
    }

    fn is_king_move(&self, from: &Position, to: &Position) -> bool {
        Self::is_normal_move(from, to) || self.is_castle(from, to).is_some()
    }

    fn check_blocking_piece(&self, start: &Square, end: &Square, dir: Direction) -> bool {
        let ray = Ray::new(*start, dir);
        for idx in ray {
            if idx == *end {
                break;
            }
            if self.get_cached_piece_at(idx).is_some() {
                return true;
            }
        }
        false
    }

    pub fn get_castle_file(side: &CastlingSide) -> FileChars {
        use FileChars::*;
        match side {
            CastlingSide::KingSide => G,
            CastlingSide::QueenSide => C,
        }
    }

    fn try_castle(&self, piece: &Piece, side: CastlingSide, from: &Square) -> Option<CastlingSide> {
        let rank = from.to_position().rank().0 + 1;
        let castle_file = Self::get_castle_file(&side);
        let end = Position::from_notation(castle_file, rank).to_square();
        let dir = Self::get_direction(&side);
        if self.check_blocking_piece(from, &end, dir) {
            return None;
        }
        Some(side)
    }

    fn is_castle(&self, from: &Position, to: &Position) -> Option<CastlingSide> {
        let df = to.file().0 as i8 - from.file().0 as i8;
        let dr = to.rank().0 as i8 - from.rank().0 as i8;

        if dr != 0 || df.abs() != 2 {
            return None;
        }

        use FileChars::{C, G};
        let side = match to.file().into() {
            G => CastlingSide::KingSide,
            C => CastlingSide::QueenSide,
            _ => return None,
        };
        let piece = self.get_cached_piece_at(*from).unwrap();

        let is_same_rank = dr == 0;
        let is_castling_attempt = is_same_rank && df.abs() == 2;

        if !is_castling_attempt {
            return None;
        }

        if self.castling_rights.can_castle_kingside(piece) {
            return self.try_castle(&piece, CastlingSide::KingSide, &from.to_square());
        }
        if self.castling_rights.can_castle_queenside(piece) {
            return self.try_castle(&piece, CastlingSide::QueenSide, &from.to_square());
        }

        None
    }
    // fuck this function -- had a dumbass bug
    fn get_direction(side: &CastlingSide) -> Direction {
        use CastlingSide::*;
        use Direction::*;
        match side {
            KingSide => East,
            QueenSide => West,
        }
    }
}
