use crate::{
    Square,
    bit_board::{BitBoard, File, Offset, Rank},
    board::{Board, Color, KNIGHT_OFFSETS, Move, Piece, PieceType},
    ray::{Direction, Ray},
};

impl Board {
    fn get_promotion_moves(&self, pos_to: Square, pawn: Square) -> [Move; 4] {
        if self.side_to_move == Color::White {
            [
                Move::new(Some(Piece::White(PieceType::Queen)), pos_to, pawn),
                Move::new(Some(Piece::White(PieceType::Rook)), pos_to, pawn),
                Move::new(Some(Piece::White(PieceType::Knight)), pos_to, pawn),
                Move::new(Some(Piece::White(PieceType::Bishop)), pos_to, pawn),
            ]
        } else {
            [
                Move::new(Some(Piece::Black(PieceType::Queen)), pos_to, pawn),
                Move::new(Some(Piece::Black(PieceType::Rook)), pos_to, pawn),
                Move::new(Some(Piece::Black(PieceType::Knight)), pos_to, pawn),
                Move::new(Some(Piece::Black(PieceType::Bishop)), pos_to, pawn),
            ]
        }
    }
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
                let top_rank = match self.side_to_move {
                    Color::White => Rank(7),
                    Color::Black => Rank(0),
                };
                if pos_to.rank() == top_rank {
                    let promotions = self.get_promotion_moves(pos_to.to_square(), pawn.to_square());
                    moves.extend(promotions);
                } else {
                    let mov = Move::new(None, pos_to.to_square(), pawn.to_square());
                    moves.push(mov);
                }
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
                let top_rank = match self.side_to_move {
                    Color::White => Rank(7),
                    Color::Black => Rank(0),
                };

                if pos_to.rank() == top_rank {
                    let promotions = self.get_promotion_moves(pos_to.to_square(), pawn.to_square());
                    moves.extend(promotions);
                } else {
                    let mov = Move::new(None, pos_to.to_square(), pawn.to_square());
                    moves.push(mov);
                }
            }
        }
    }

    fn add_knight_moves(&self, moves: &mut Vec<Move>) {
        let bitboard = match self.side_to_move {
            Color::White => self.white_knight,
            Color::Black => self.black_knight,
        };
        for knight in bitboard.iter_set_bits().map(Square) {
            let mut move_offsets = Vec::with_capacity(KNIGHT_OFFSETS.len());
            let knight_pos = knight.to_position();
            for e in &KNIGHT_OFFSETS {
                if let Some(square) = knight.offset(*e) {
                    let diff = square.to_position().diff(&knight_pos);
                    if diff.0.abs() <= 2 && diff.1.abs() <= 2 {
                        move_offsets.push(square);
                    }
                }
            }
            for m in move_offsets {
                moves.push(Move::new(None, m, knight));
            }
        }
    }
    fn add_pawn_moves(&self, moves: &mut Vec<Move>) {
        let pawns = self.get_bitboard(PieceType::Pawn, &self.side_to_move);

        self.add_pawn_pushes(pawns, moves);
        self.add_pawn_double_push(pawns, moves);
        self.add_pawn_captures(pawns, moves);
    }

    fn check_direction(&self, rook: Square, dir: Direction, moves: &mut Vec<Move>) {
        for square in Ray::new(rook, dir) {
            let piece_opt = self.board_rep()[square.0 as usize];

            if let Some(piece) = piece_opt
                && piece.is_color(self.side_to_move)
            {
                return;
            }

            moves.push(Move::new(None, rook, square));
        }
    }
    fn add_rook_moves(&self, moves: &mut Vec<Move>) {
        const DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        let rooks = self.get_bitboard(PieceType::Rook, &self.side_to_move);

        for rook in rooks.iter_squares() {
            for dir in DIRECTIONS {
                self.check_direction(rook, dir, moves);
            }
        }
    }

    pub(crate) fn generate_pseudolegal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);

        self.add_pawn_moves(&mut moves);
        self.add_knight_moves(&mut moves);
        self.add_rook_moves(&mut moves);

        moves
    }
}
