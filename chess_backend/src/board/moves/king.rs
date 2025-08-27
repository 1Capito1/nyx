use crate::bit_board::{FileChars, Square};
use crate::board::PieceType::King;
use crate::board::{Board, CastlingSide, Piece};
use crate::errors::MoveError;
use crate::ray::{Direction, Ray};
use crate::{Position, Rank};

use super::{Move, UndoMove};

type KingUndo = Result<[Option<UndoMove>; 2], MoveError>;

impl Board {
    pub(crate) fn move_king(&mut self, move_info: Move) -> KingUndo {
        let current_pos = move_info.move_from().to_position();
        let pos_to = move_info.move_to().to_position();

        let mut ret: [Option<UndoMove>; 2] = [None, None];

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

        if !self.is_king_move(&current_pos, &pos_to) {
            return Err(MoveError::InvalidMove(King, current_pos, pos_to));
        }

        if let Some(side) = self.is_castle(&current_pos, &pos_to) {
            let rank = move_info.move_to().to_position().rank();
            ret[1] = Some(self.castle_rook(side, rank));
        }

        ret[0] = Some(self.move_piece_unchecked(&move_info));

        Ok(ret)
    }

    fn castle_rook(&mut self, side: CastlingSide, rank: Rank) -> UndoMove {
        let (rook_from_file, rook_to_file) = match side {
            CastlingSide::KingSide => (FileChars::H, FileChars::F),
            CastlingSide::QueenSide => (FileChars::A, FileChars::D),
        };

        let rank_u8: u8 = u8::from(rank) + 1;
        let square_start = Position::from_notation(rook_from_file, rank_u8);
        let square_end = Position::from_notation(rook_to_file, rank_u8);

        let castling_move = Move::builder()
            .move_to(square_end.to_square())
            .move_from(square_start.to_square())
            .build();

        self.move_piece_unchecked(&castling_move)
    }

    fn is_normal_move(from: &Position, to: &Position) -> bool {
        let (file_diff, rank_diff) = from.diff(to);
        file_diff.max(rank_diff) == 1
    }

    fn is_king_move(&self, from: &Position, to: &Position) -> bool {
        if Self::is_normal_move(from, to) {
            println!("normal move")
        };
        if self.is_castle(from, to).is_none() {
            println!("not castle")
        };
        Self::is_normal_move(from, to) || self.is_castle(from, to).is_some()
    }

    fn check_blocking_piece(&self, start: &Square, end: &Square, dir: Direction) -> bool {
        println!(
            "start: {}, end: {}, dir: {:?}",
            start.to_position(),
            end.to_position(),
            dir
        );
        let ray = Ray::new(*start, dir);
        for idx in ray {
            println!("{}", idx.to_position());
            if idx == *end {
                break;
            }
            if self.get_cached_piece_at(&idx.to_position()).is_some() {
                println!("idx: {}", idx.to_position());
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
        println!("rank: {rank}");
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
        println!("df: {}, dr: {}", df, dr);

        if dr != 0 || df.abs() != 2 {
            return None;
        }

        use FileChars::{C, G};
        let side = match to.file().into() {
            G => CastlingSide::KingSide,
            C => CastlingSide::QueenSide,
            _ => return None,
        };
        let piece = self.get_cached_piece_at(from).unwrap();

        let is_same_rank = dr == 0;
        let is_castling_attempt = is_same_rank && df.abs() == 2;

        if !is_castling_attempt {
            return None;
        }

        if self.castling_rights.can_castle_kingside(piece) {
            println!("kingside");
            return self.try_castle(&piece, CastlingSide::KingSide, &from.to_square());
        }
        if self.castling_rights.can_castle_queenside(piece) {
            println!("queenside");
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
