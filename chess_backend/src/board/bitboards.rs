use crate::bit_board::BitBoard;
use crate::bit_board::Square;
use crate::board::Piece;
use crate::board::PieceType::*;
use crate::Position;

use super::Board;

impl Board {
    pub fn get_bitboard_pieces(&self) -> [(BitBoard, Piece); 12] {
        [
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
 
}
