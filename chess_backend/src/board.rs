use crate::bit_board::{BitBoard, Position, Square};
use crate::move_gen::UndoMove;
use crate::move_gen::Move;
use crate::piece::{FenNotation, Piece};
use crate::piece::PieceType::*;

// TODO: Use or remove
//#[repr(u8)]
//#[derive(Copy, Clone, Debug, PartialEq, Eq)]
//pub enum Square {
//    A1 = 0,  B1 = 1,  C1 = 2,  D1 = 3,  E1 = 4,  F1 = 5,  G1 = 6,  H1 = 7,
//    A2 = 8,  B2 = 9,  C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
//    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
//    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
//    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
//    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
//    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
//    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
//}

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

            board_rep: [None; 64],
        }
    }
}

impl Board {
    pub fn get_bitboard_pieces(&self) -> [(BitBoard, Piece); 12] {
        return [
            (self.white_pawn, Piece::White(Pawn)),
            (self.white_rook, Piece::White(Rook)),
            (self.white_knight, Piece::White(Knight)),
            (self.white_bishop, Piece::White(Bishop)),
            (self.white_queen, Piece::White(Queen)),
            (self.white_king, Piece::White(King)),
            (self.black_pawn, Piece::Black(Pawn)),
            (self.black_rook, Piece::Black(Rook)),
            (self.black_knight, Piece::Black(Knight)),
            (self.black_bishop, Piece::Black(Bishop)),
            (self.black_queen, Piece::Black(Queen)),
            (self.black_king, Piece::Black(King)),
        ]
    }
    pub fn get_piece_at(&self, position: Position) -> Option<Piece> {
        let square = Square::from_position(&position);

        for (bitboard, piece) in self.get_bitboard_pieces() {
            if *bitboard.bits() & *square as u64 != 0 {
                return Some(piece);
            }
        }
        None
    }
    pub fn get_cached_piece_at(&self, position: &Position) -> Option<Piece> {
        let square = Square::from_position(position);
        return self.board_rep[*square as usize];
    }
    pub fn update_board_rep(&mut self) {
        self.board_rep = [None; 64];

        for (bitboard, piece) in self.get_bitboard_pieces() {
            for square in bitboard.iter_set_bits() {
                self.board_rep[square as usize] = Some(piece);
            }
        }
    }
    fn get_white_peices(&self) -> BitBoard {
        self.white_pawn | 
            self.white_bishop |
            self.white_rook | 
            self.white_knight |
            self.white_pawn |
            self.white_queen | 
            self.white_king
    }

    fn get_black_peices(&self) -> BitBoard {
        self.black_pawn | 
            self.black_bishop |
            self.black_rook | 
            self.black_knight |
            self.black_queen | 
            self.black_king
    }

    pub fn pretty_print(&self) -> [char; 64] {
        let mut board_rep = ['.'; 64];
        for (i, cell) in board_rep.iter_mut().enumerate() {
            let mut set_count = 0;
            if self.white_pawn.is_set(i as u8) {
                *cell = 'P';
                set_count += 1;
            }
            if self.white_bishop.is_set(i as u8) {
                *cell = 'B';
                set_count += 1;
            }
            if self.white_rook.is_set(i as u8) {
                *cell = 'R';
                set_count += 1;
            }
            if self.white_queen.is_set(i as u8) {
                *cell = 'Q';
                set_count += 1;
            }
            if self.white_king.is_set(i as u8) {
                *cell = 'K';
                set_count += 1;
            }
            if self.white_knight.is_set(i as u8) {
                *cell = 'N';
                set_count += 1;
            }
            if self.black_pawn.is_set(i as u8) {
                *cell = 'p';
                set_count += 1;
            }
            if self.black_bishop.is_set(i as u8) {
                *cell = 'b';
                set_count += 1;
            }
            if self.black_knight.is_set(i as u8) {
                *cell = 'n';
                set_count += 1;
            }
            if self.black_rook.is_set(i as u8) {
                *cell = 'r';
                set_count += 1;
            }
            if self.black_king.is_set(i as u8) {
                *cell = 'k';
                set_count += 1;
            }
            if self.black_queen.is_set(i as u8) {
                *cell = 'q';
                set_count += 1;
            }
            if set_count > 1 {
                panic!("Multiple Pieces on one square");
            }
        }
        BitBoard::print_bitboard_rep(&board_rep);   
        return board_rep;
    }

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

    pub fn move_piece_unchecked(&mut self, move_info: Move) -> UndoMove {
        let square_from = move_info.move_from();
        let square_to = move_info.move_to();
        let position_from = square_from.to_position();
        let position_to = square_to.to_position();
        let moved_piece = self.get_cached_piece_at(&position_from);
        let piece_at = self.get_cached_piece_at(&position_to);

        if let Some(piece) = moved_piece {
            if let Some(piece_at) = piece_at {
                let board_at = self.match_board(piece_at);
                board_at.clear_bit(square_to);
            }
            let board = self.match_board(piece);
            board.clear_bit(square_from);
            board.set_bit(square_to);
            self.update_board_rep();
            return UndoMove::builder()
                .move_to(square_to)
                .move_from(square_from)
                .captured_piece(piece_at)
                .build();
        }
        panic!("Piece not found");
    }
}
