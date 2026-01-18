use crate::{
    bit_board::Square,
    board::{Board, Color, Move, Piece, PieceType},
};

impl Board {
    fn generate_legal_moves(&mut self) -> Vec<Move> {
        let mut legals = Vec::new();
        let pseudo = self.generate_pseudolegal_moves();

        for mv in pseudo {
            if let Ok(undo) = self.try_move(mv.move_from(), mv.move_to()) {
                self.undo_move(undo);
                legals.push(mv);
            }
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
            if let Ok(undo) = self.try_move(mov.move_from(), mov.move_to()) {
                n_moves += self.perft(depth - 1);
                self.undo_move(undo);
            }
        }
        n_moves
    }
}
