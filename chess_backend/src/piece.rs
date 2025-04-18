use PieceType::*;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

pub enum FenNotation {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteKing,
    WhiteQueen,

    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackKing,
    BlackQueen,
}

impl From<FenNotation> for char {
    fn from(value: FenNotation) -> Self {
        match value {
            FenNotation::WhitePawn => 'P',
            FenNotation::WhiteRook => 'R',
            FenNotation::WhiteKnight => 'N',
            FenNotation::WhiteBishop => 'B',
            FenNotation::WhiteKing => 'K',
            FenNotation::WhiteQueen => 'Q',
            FenNotation::BlackPawn => 'p',
            FenNotation::BlackRook => 'r',
            FenNotation::BlackKnight => 'n',
            FenNotation::BlackBishop => 'b',
            FenNotation::BlackKing => 'k',
            FenNotation::BlackQueen => 'q',
        }
    }
}

impl TryFrom<char> for FenNotation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' => Ok(FenNotation::WhitePawn),
            'N' => Ok(FenNotation::WhiteKnight),
            'B' => Ok(FenNotation::WhiteBishop),
            'R' => Ok(FenNotation::WhiteRook),
            'Q' => Ok(FenNotation::WhiteQueen),
            'K' => Ok(FenNotation::WhiteKing),
            'p' => Ok(FenNotation::BlackPawn),
            'n' => Ok(FenNotation::BlackKnight),
            'b' => Ok(FenNotation::BlackBishop),
            'r' => Ok(FenNotation::BlackRook),
            'q' => Ok(FenNotation::BlackQueen),
            'k' => Ok(FenNotation::BlackKing),
            _ => Err(())
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    White(PieceType),
    Black(PieceType)
}

impl From<Piece> for FenNotation {
    fn from(value: Piece) -> Self {
        match value {
            Piece::White(piece) => {
                match piece {
                    PieceType::Pawn => FenNotation::WhitePawn,
                    PieceType::Rook => FenNotation::WhiteRook,
                    PieceType::Knight => FenNotation::WhiteKnight,
                    PieceType::Bishop => FenNotation::WhiteBishop,
                    PieceType::King => FenNotation::WhiteKing,
                    PieceType::Queen => FenNotation::WhiteQueen,
                }
            },
            Piece::Black(piece) => {
                match piece {
                    PieceType::Pawn => FenNotation::BlackPawn,
                    PieceType::Rook => FenNotation::BlackRook,
                    PieceType::Knight => FenNotation::BlackKnight,
                    PieceType::Bishop => FenNotation::BlackBishop,
                    PieceType::King => FenNotation::BlackKing,
                    PieceType::Queen => FenNotation::BlackQueen,
                }
            },
        }
    }
}

impl From<FenNotation> for Piece {
    fn from(value: FenNotation) -> Self {
        match value {
            FenNotation::WhitePawn => Self::White(Pawn),
            FenNotation::WhiteRook => Self::White(Rook),
            FenNotation::WhiteKnight => Self::White(Knight),
            FenNotation::WhiteBishop => Self::White(Bishop),
            FenNotation::WhiteKing => Self::White(King),
            FenNotation::WhiteQueen => Self::White(Queen),

            FenNotation::BlackPawn => Self::Black(Pawn),
            FenNotation::BlackRook => Self::Black(Rook),
            FenNotation::BlackKnight => Self::Black(Knight),
            FenNotation::BlackBishop => Self::Black(Bishop),
            FenNotation::BlackKing => Self::Black(King),
            FenNotation::BlackQueen => Self::Black(Queen),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fen_round_trip() {
        for c in ['P', 'n', 'k', 'Q'] {
            let fen = FenNotation::try_from(c).unwrap();
            let piece: Piece = fen.into();
            let back = char::from(FenNotation::from(piece));
            assert_eq!(c, back);
        }
    }
}
