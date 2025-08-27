use typed_builder::TypedBuilder;

use crate::bit_board::Square;
use crate::board::Piece;

pub(crate) enum SpecialMove {
    Promotion(Piece),
    EnPassant(Square),
    Castling{rook_to: u8, rook_from: u8},
}

#[derive(TypedBuilder, Debug)]
pub(crate) struct Move {
    #[builder(default, setter(strip_option))]
    promotion: Option<Piece>,
    move_to: Square,
    move_from: Square,
}

impl Move {
    pub(crate) fn new(promotion: Option<Piece>, move_to: Square, move_from: Square) -> Self {
        Self { promotion, move_to, move_from }
    }

    pub(crate) fn promotion(&self) -> Option<Piece> {
        self.promotion
    }

    pub(crate) fn move_to(&self) -> Square {
        self.move_to
    }

    pub(crate) fn move_from(&self) -> Square {
        self.move_from
    }
}

#[derive(TypedBuilder)]
pub(crate) struct UndoMove {
    #[builder(default, setter(into))]
    captured_piece: Option<Piece>,
    move_to: Square,
    move_from: Square,
    #[builder(default)]
    special: Option<SpecialMove>,
}

impl UndoMove {
    pub(crate) fn new(captured_piece: Option<Piece>, move_to: Square, move_from: Square, special: Option<SpecialMove>) -> Self {
        Self { captured_piece, move_to, move_from, special }
    }

    pub(crate) fn captured_piece(&self) -> Option<Piece> {
        self.captured_piece
    }

    pub(crate) fn move_to(&self) -> Square {
        self.move_to
    }

    pub(crate) fn move_from(&self) -> Square {
        self.move_from
    }

    pub(crate) fn special(&self) -> Option<&SpecialMove> {
        self.special.as_ref()
    }
    pub(crate) fn special_mut(&mut self) -> Option<&mut SpecialMove> {
        self.special.as_mut()
    }

    pub(crate) fn set_special(&mut self, special: Option<SpecialMove>) {
        self.special = special;
    }
}


