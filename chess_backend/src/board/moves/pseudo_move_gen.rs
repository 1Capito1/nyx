use crate::{
    bit_board::{BitBoard, Rank},
    board::{Board, Color, Move, PieceType},
    Square,
};

impl Board {
    fn add_pawn_pushes(&self, pawns: BitBoard, moves: &mut Vec<Move>) {
        for pawn in pawns.iter_squares().map(Square::to_position) {
            let direction = match self.side_to_move {
                Color::White => 1,
                Color::Black => -1,
            };

            if let Some(pos_to) = pawn.offset_pos((0, direction)) {
                if self.get_cached_piece_at(pos_to).is_some() {
                    continue;
                }
                let mov = Move::new(None, pos_to.to_square(), pawn.to_square());
                moves.push(mov);
            }
        }
    }

    fn add_pawn_double_push(&self, pawns: BitBoard, moves: &mut Vec<Move>) {
        for pawn in pawns.iter_squares().map(Square::to_position) {
            let (direction, allowed_rank) = match self.side_to_move {
                Color::White => (2, 1),
                Color::Black => (-2, 6),
            };

            if pawn.rank() != Rank(allowed_rank) {
                continue;
            }

            if let Some(pos_to) = pawn.offset_pos((0, direction)) {
                if self.get_cached_piece_at(pos_to).is_some() {
                    continue;
                }
                let mov = Move::new(None, pos_to.to_square(), pawn.to_square());
                moves.push(mov);
            }
        }
    }
    fn add_pawn_captures(&self, pawns: BitBoard, moves: &mut Vec<Move>) {
        for pawn in pawns.iter_squares().map(Square::to_position) {
            let offset = match self.side_to_move {
                Color::White => 1,
                Color::Black => -1,
            };

            if let Some(pos_to) = pawn.offset_pos((1, offset)) {
                if !self
                    .en_passant_square
                    .is_some_and(|x| x == Square(pawn.to_square().0 + 1))
                {
                    continue;
                }
                match self.get_cached_piece_at(pos_to) {
                    Some(piece) => {
                        if piece.get_color() != self.side_to_move {
                            continue;
                        }
                    }
                    None => continue,
                }
                let mov = Move::new(None, pos_to.to_square(), pawn.to_square());
                moves.push(mov);
            }
        }
    }
    fn add_pawn_moves(&self, moves: &mut Vec<Move>) {
        let pawns = self.get_bitboard(PieceType::Pawn, &self.side_to_move);

        self.add_pawn_pushes(pawns, moves);
        self.add_pawn_double_push(pawns, moves);
        self.add_pawn_captures(pawns, moves);
    }

    fn add_pawn_promotions(&self, pawns: BitBoard, moves: &mut Vec<Move>) {}

    fn add_rook_moves(&self, moves: &mut Vec<Move>) {
        let rooks = self.get_bitboard(PieceType::Rook, &self.side_to_move);

        for rook in rooks.iter_squares() {}
    }

    pub(crate) fn generate_pseudolegal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);

        self.add_pawn_moves(&mut moves);

        moves
    }
}
