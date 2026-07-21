use crate::{
    bit_board::{File, FileChars, Position, Rank, Square},
    board::{Board, Color, Move, Piece, PieceType, UndoMove},
};

impl Board {
    fn generate_legal_moves(&mut self) -> Vec<(Move, UndoMove)> {
        let mut legals = Vec::with_capacity(256);
        let pseudo = self.generate_pseudolegal_moves();

        for mv in pseudo {
            match self.try_move(&mv) {
                Ok(undo) => {
                    self.undo_move(&undo);
                    legals.push((mv, undo));
                }
                Err(e) => {}
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

        for (mov, undo) in moves {
            let undo = self
                .try_move(&mov)
                .inspect_err(|x| {
                    self.pretty_print();
                })
                .expect("Legal move should never fail");
            n_moves += self.perft(depth - 1);
            self.undo_move(&undo);
        }
        n_moves
    }

    pub fn perft_divide(&mut self, depth: u32) -> u64 {
        let moves = self.generate_legal_moves();
        let mut total = 0;

        for (mv, undo) in moves {
            let undo = self.try_move(&mv).expect("Legal move already checked");
            let nodes = self.perft(depth - 1);
            self.undo_move(&undo);

            println!("{mv}: {nodes}");
            total += nodes;
        }

        println!("Total: {total}");
        total
    }
    pub fn perft_divide_after(&mut self, moves_str: &[&str], depth: u32) -> u64 {
        // Make all the initial moves
        let mut undos = Vec::new();
        for move_str in moves_str {
            let moves = self.generate_legal_moves();
            let (mv, undo) = moves
                .iter()
                .find(|m| m.0.to_string().to_lowercase() == move_str.to_lowercase())
                .expect(&format!("Move {} not found", move_str));

            let undo = self.try_move(mv).expect("Legal move should work");
            undos.push(undo);
        }

        // Now run perft divide from this position
        let mut total = 0;
        let moves = self.generate_legal_moves();
        for (mv, undo) in moves {
            let undo = self.try_move(&mv).expect("Legal move already checked");
            let nodes = self.perft(depth - 1);
            self.undo_move(&undo);
            println!("{mv}: {nodes}");
            total += nodes;
        }
        println!("Total after {:?}: {total}", moves_str);

        // Undo all the initial moves in reverse order
        for undo in undos.into_iter().rev() {
            self.undo_move(&undo);
        }

        total
    }
}
