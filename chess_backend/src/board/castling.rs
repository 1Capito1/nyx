use super::Piece;

pub(crate) struct CastlingRights {
    black_kingside: bool,
    white_kingside: bool,

    black_queenside: bool,
    white_queenside: bool,
}

impl CastlingRights {
    pub(crate) fn new(black_kingside: bool, white_kingside: bool, black_queenside: bool, white_queenside: bool) -> Self {
        Self { black_kingside, white_kingside, black_queenside, white_queenside }
    }

    pub fn can_castle_kingside(&self, piece: Piece) -> bool {
        match piece {
            Piece::Black(_)  => self.black_kingside,
            Piece::White(_) => self.white_kingside,
        }
    }
    pub fn can_castle_queenside(&self, piece: Piece) -> bool {
        match piece {
            Piece::Black(_) => self.black_queenside,
            Piece::White(_) => self.white_queenside,
        }
    }

    pub(crate) fn get_from_fen(&mut self, c: char) -> &mut bool {
        eprintln!("character {c}");
        match c {
            'k' => &mut self.black_kingside,
            'q' => &mut self.black_queenside,
            'K' => &mut self.white_kingside,
            'Q' => &mut self.white_queenside,
            _ => panic!("Invalid FEN"),
        }
    }
}
