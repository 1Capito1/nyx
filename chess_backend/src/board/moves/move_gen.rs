use crate::{
    bit_board::Square,
    board::{Board, Color, Move, Piece, PieceType},
};

impl Board {
    fn generate_legal_moves(&mut self) -> Vec<Move> {
        let mut legals = Vec::new();
        let pseudo = self.generate_pseudolegal_moves();

        for mv in pseudo {
            let undo = self.try_move(mv.move_from(), mv.move_to()).unwrap();
            let king_pos = match self.side_to_move {
                Color::White => self.white_king.iter_squares().nth(0).unwrap().to_position(),
                Color::Black => self.black_king.iter_squares().nth(0).unwrap().to_position(),
            };
            let enemy_color = match self.side_to_move {
                Color::White => Piece::White(PieceType::Pawn),
                Color::Black => Piece::Black(PieceType::Pawn),
            };
            if self.is_check(&king_pos, enemy_color) {
                continue;
            }
            self.undo_move(undo);
            legals.push(mv);
        }

        legals
    }

    pub fn perft(&mut self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }

        let moves = self.generate_legal_moves();

        let mut n_moves = 0;
        for mov in moves {
            let undo = self
                .try_move(mov.move_from(), mov.move_to())
                .expect("Already checked");
            n_moves += self.perft(depth - 1);
            self.undo_move(undo);
        }
        n_moves
    }
}
