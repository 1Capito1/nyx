use derive_more::Display;
use thiserror::Error;

use crate::board::{Piece, PieceType};
use crate::bit_board::{FileChars, Position};

#[derive(Error, Debug)]
pub enum MoveError {
    #[error("Attempted capture on piece of same color, Position: {0}")]
    CaptureSameColor(Position),
    #[error("Invalid Position: {0}")]
    InvalidPosition(Position),
    #[error("No piece at Position: {0}")]
    PieceNotFound(Position),
    #[error("Incorrect Piece Type, expected: {0}, got {1}")]
    IncorrectPiece(PieceType, PieceType),
    #[error("OOB Access: file: {0}, line: {1}")]
    OutOfBoundsPositionAccess(&'static str, u32),
    #[error("Illegal Collision")]
    IllegalCollision,
    #[error("Illegal Pawn double pawn push: from: {0}, to: {1}")]
    IllegalDoublePawnPush(Position, Position),
    #[error("Pawn promotion without specifying piece: {0}:{1}")]
    PromotionPieceMissing(Position, Position),
    #[error("Invalid move: Piece: {0}, from: {1}, to: {2}")]
    InvalidMove(PieceType, Position, Position),
    #[error("Piece Blocking Sliding Movement: from: {0}, to: {1}")]
    PieceBlockingMovement(Position, Position),
    #[error("CRITICAL: Unknown Error: {0}:{1}")]
    Unknown(&'static str, u32),
}
