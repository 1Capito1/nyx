use typed_builder::TypedBuilder;

use crate::bit_board::Square;
use crate::board::{Board, Color, Piece};

pub(crate) enum SpecialMove {
    Promotion(Piece),
    EnPassant(Square),
    Castling { rook_to: u8, rook_from: u8 },
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
        Self {
            promotion,
            move_to,
            move_from,
        }
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
    undo: Vec<UndoChange>,
    #[builder(default)]
    special: Option<SpecialMove>,
    #[builder(default=Color::White)]
    side_before: Color,
}

#[derive(Clone, Debug)]
pub(crate) struct UndoChange {
    at: Square,
    piece_before: Option<Piece>,
}

impl UndoChange {
    pub(crate) fn new(at: Square, piece_before: Option<Piece>) -> Self {
        Self { at, piece_before }
    }

    pub(crate) fn at(&self) -> Square {
        self.at
    }

    pub(crate) fn piece_before(&self) -> Option<Piece> {
        self.piece_before
    }
}

impl UndoMove {
    pub(crate) fn new(undo: Vec<UndoChange>, special: Option<SpecialMove>) -> Self {
        Self {
            undo,
            special,
            side_before: Color::White,
        }
    }

    pub const fn get_changes(&self) -> &Vec<UndoChange> {
        &self.undo
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

    pub(crate) fn get_side(&self) -> Color {
        self.side_before
    }
    pub(crate) fn set_side(&mut self, side: Color) {
        self.side_before = side;
    }
}
