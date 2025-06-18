use crate::bit_board::{BitBoard, FileChars, Offset, Position, Square, Rank};
use crate::errors::MoveError;
use crate::board::{Move, UndoMove, SpecialMove};
use crate::board::PieceType::*;
use crate::board::{FenNotation, Piece, PieceType};

use super::CastlingRights;


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

            castling_rights: CastlingRights::new(true, true, true, true),

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

    pub fn place_piece(&mut self, piece: impl Into<Piece>, position: &Position) {
        let board = self.match_board(piece.into());
        board.place(position);
    }


    /// uses cached piece to determine if a piece is at [```Position```]
    pub fn collides(&self, pos: &Position) -> bool {
        self.get_cached_piece_at(pos).is_some()
    }

    pub(crate) fn board_rep_mut(&mut self) -> &mut [Option<Piece>; 64] {
        &mut self.board_rep
    }
    pub(crate) fn board_rep(&self) -> &[Option<Piece>; 64] {
        &self.board_rep
    }
}
