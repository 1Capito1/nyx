use crate::{
    Square,
    bit_board::{BitBoard, File, FileChars, Offset, Position, Rank},
    board::{Board, Color, KNIGHT_OFFSETS, Move, MoveBuilder, Piece, PieceType},
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

            for file_offset in [-1, 1] {
                if let Some(pos_to) = pawn.offset_pos((file_offset, offset)) {
                    let is_en_passant = self
                        .en_passant_square
                        .is_some_and(|x| x == pos_to.to_square());
                    let is_valid_capture = match self.get_cached_piece_at(pos_to) {
                        Some(piece) => piece.get_color() != self.side_to_move,
                        None => is_en_passant,
                    };

                    if !is_valid_capture {
                        continue;
                    }
                    let top_rank = match self.side_to_move {
                        Color::White => Rank(7),
                        Color::Black => Rank(0),
                    };

                    if pos_to.rank() == top_rank {
                        let promotions =
                            self.get_promotion_moves(pos_to.to_square(), pawn.to_square());
                        moves.extend(promotions);
                    } else {
                        let mov = Move::builder()
                            .move_from(pawn.to_square())
                            .move_to(pos_to.to_square())
                            .build();

                        moves.push(mov);
                    }
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
            for m in &move_offsets {
                if let Some(piece) = self.board_rep()[m.0 as usize]
                    && piece.is_color(self.side_to_move)
                {
                    continue;
                }
                moves.push(Move::new(None, *m, knight));
            }
        }
    }
    fn add_pawn_moves(&self, moves: &mut Vec<Move>) {
        let pawns = self.get_bitboard(PieceType::Pawn, &self.side_to_move);

        self.add_pawn_pushes(pawns, moves);
        self.add_pawn_double_push(pawns, moves);
        self.add_pawn_captures(pawns, moves);
    }

    fn check_direction(&self, from: Square, dir: Direction, moves: &mut Vec<Move>) {
        for square in Ray::new(from, dir) {
            let piece_opt = self.board_rep()[square.0 as usize];

            if let Some(piece) = piece_opt {
                if piece.is_color(self.side_to_move) {
                    return;
                } else {
                    let mv = Move::builder().move_from(from).move_to(square).build();
                    moves.push(mv);
                    return;
                }
            }
            let mv = Move::builder().move_from(from).move_to(square).build();
            moves.push(mv);
        }
    }
    fn add_rook_moves(&self, moves: &mut Vec<Move>) {
        let before = moves.len();
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

    fn add_bishop_moves(&self, moves: &mut Vec<Move>) {
        const DIRECTIONS: [Direction; 4] = [
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
        ];

        let bishops = self.get_bitboard(PieceType::Bishop, &self.side_to_move);

        for bishop in bishops.iter_squares() {
            for dir in DIRECTIONS {
                self.check_direction(bishop, dir, moves);
            }
        }
    }

    fn add_queen_moves(&self, moves: &mut Vec<Move>) {
        const DIRECTIONS: [Direction; 8] = [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ];
        let queens = self.get_bitboard(PieceType::Queen, &self.side_to_move);

        for queen in queens.iter_squares() {
            for dir in DIRECTIONS {
                self.check_direction(queen, dir, moves);
            }
        }
    }

    fn add_king_moves(&self, moves: &mut Vec<Move>) {
        const OFFSETS: [(i8, i8); 10] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (2, 0),
            (-2, 0),
        ];
        let king = match self.side_to_move {
            Color::White => self.white_king,
            Color::Black => self.black_king,
        }
        .iter_squares()
        .nth(0)
        .expect("King should always exist on board");

        for offset in OFFSETS {
            if let Some(to) = king.to_position().offset_pos(offset) {
                moves.push(
                    Move::builder()
                        .move_from(king)
                        .move_to(to.to_square())
                        .build(),
                );
            }
        }
    }

    pub(crate) fn generate_pseudolegal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(256);

        self.add_pawn_moves(&mut moves);
        self.add_knight_moves(&mut moves);
        self.add_rook_moves(&mut moves);
        self.add_bishop_moves(&mut moves);
        self.add_queen_moves(&mut moves);
        self.add_king_moves(&mut moves);

        moves
    }
}
