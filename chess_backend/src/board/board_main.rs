use crate::bit_board::{BitBoard, FileChars, Offset, Position, Rank, Square};
use crate::board::PieceType::*;
use crate::board::{FenNotation, Piece, PieceType};
use crate::board::{Move, SpecialMove, UndoMove};
use crate::errors::MoveError;

use super::CastlingRights;

#[derive(PartialEq, Eq)]
pub(crate) enum Color {
    White,
    Black,
}

pub(crate) struct Board {
    pub(crate) white_pawn: BitBoard,
    pub(crate) white_rook: BitBoard,
    pub(crate) white_knight: BitBoard,
    pub(crate) white_bishop: BitBoard,
    pub(crate) white_queen: BitBoard,
    pub(crate) white_king: BitBoard,

    pub(crate) black_pawn: BitBoard,
    pub(crate) black_rook: BitBoard,
    pub(crate) black_knight: BitBoard,
    pub(crate) black_bishop: BitBoard,
    pub(crate) black_queen: BitBoard,
    pub(crate) black_king: BitBoard,

    pub(crate) en_passant_square: Option<Square>,
    pub(crate) castling_rights: CastlingRights,
    pub(crate) side_to_move: Color,

    board_rep: [Option<Piece>; 64],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white_pawn: BitBoard::default(),
            white_rook: BitBoard::default(),
            white_knight: BitBoard::default(),
            white_bishop: BitBoard::default(),
            white_queen: BitBoard::default(),
            white_king: BitBoard::default(),

            black_pawn: BitBoard::default(),
            black_rook: BitBoard::default(),
            black_knight: BitBoard::default(),
            black_bishop: BitBoard::default(),
            black_queen: BitBoard::default(),
            black_king: BitBoard::default(),

            side_to_move: Color::White,

            castling_rights: CastlingRights::new(false, false, false, false),

            en_passant_square: None,

            board_rep: [None; 64],
        }
    }
}

impl Board {
    pub fn match_board(&mut self, piece: impl Into<FenNotation>) -> &mut BitBoard {
        match piece.into() {
            FenNotation::WhitePawn => &mut self.white_pawn,
            FenNotation::WhiteRook => &mut self.white_rook,
            FenNotation::WhiteKnight => &mut self.white_knight,
            FenNotation::WhiteBishop => &mut self.white_bishop,
            FenNotation::WhiteKing => &mut self.white_king,
            FenNotation::WhiteQueen => &mut self.white_queen,
            FenNotation::BlackPawn => &mut self.black_pawn,
            FenNotation::BlackRook => &mut self.black_rook,
            FenNotation::BlackKnight => &mut self.black_knight,
            FenNotation::BlackBishop => &mut self.black_bishop,
            FenNotation::BlackKing => &mut self.black_king,
            FenNotation::BlackQueen => &mut self.black_queen,
        }
    }

    pub fn get_bitboard(&self, piece_type: PieceType, color: &Color) -> BitBoard {
        match color {
            Color::White => match piece_type {
                Pawn => self.white_pawn,
                Rook => self.white_rook,
                Knight => self.white_knight,
                Bishop => self.white_bishop,
                King => self.white_king,
                Queen => self.white_queen,
            },
            Color::Black => match piece_type {
                Pawn => self.black_pawn,
                Rook => self.black_rook,
                Knight => self.black_knight,
                Bishop => self.black_bishop,
                King => self.black_king,
                Queen => self.black_queen,
            },
        }
    }

    pub fn place_piece(&mut self, piece: impl Into<Piece>, position: &Position) {
        self.match_board(piece.into()).place(position);
    }

    /// uses cached piece to determine if a piece is at [```Position```]
    pub fn collides(&self, pos: &Position) -> bool {
        self.get_cached_piece_at(*pos).is_some()
    }

    pub(crate) fn board_rep_mut(&mut self) -> &mut [Option<Piece>; 64] {
        &mut self.board_rep
    }
    pub(crate) fn board_rep(&self) -> &[Option<Piece>; 64] {
        &self.board_rep
    }
}
