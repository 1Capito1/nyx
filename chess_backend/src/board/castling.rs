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

    pub(crate) fn get_from_fen(&mut self, c: char) {
        eprintln!("character {c}");
        match c {
            'k' => self.black_kingside = true,
            'q' => self.black_queenside = true,
            'K' => self.white_kingside = true,
            'Q' => self.white_queenside = true,
            '-' => (),
            _ => panic!("Invalid FEN"),
        }
    }

    pub(crate) fn set_castling_rights(&mut self, s: &str) {
        s.chars().for_each(|c| self.get_from_fen(c));
    }
}
