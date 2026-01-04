use crate::bit_board::{Offset, Square};
use crate::board::{Board, UndoChange};
use crate::board::{Move, SpecialMove, UndoMove};
use crate::errors::MoveError;
use crate::PieceType::Pawn;
use crate::{Piece, Position, Rank};

impl Board {
    fn pawn_capture(
        &mut self,
        pos_to: Position,
        pos_from: Position,
        piece: Piece,
        move_info: &Move,
    ) -> Result<UndoMove, MoveError> {
        let left = pos_to
            .file()
            .offset(-1)
            .ok_or(MoveError::Unknown(file!(), line!()))?;
        let right = pos_to
            .file()
            .offset(1)
            .ok_or(MoveError::Unknown(file!(), line!()))?;
        // check that pos_to is only one to the left or right
        if !(left == pos_from.file() || right == pos_from.file()) {
            return Err(MoveError::InvalidPosition(pos_to));
        }
        let step_rank = piece.pawn_step(pos_to.rank());

        // make sure pos_to is only one rank from pos_from
        if step_rank.is_some() && step_rank.unwrap() == pos_to.rank() {
            return Err(MoveError::InvalidPosition(pos_to));
        }

        let piece_at = self.get_cached_piece_at(pos_to);

        let piece_at = match piece_at {
            Some(p) => p,
            None => {
                if self
                    .en_passant_square
                    .is_some_and(|f| f == pos_to.to_square())
                {
                    let step_inter: i8 = match piece {
                        Piece::White(_) => -1,
                        Piece::Black(_) => 1,
                    };
                    let mut pawn_pos = pos_to;
                    let position = pos_to
                        .rank()
                        .offset(step_inter)
                        .expect("invalid en passant rank offset");
                    pawn_pos.set_rank(position.0);
                    let captured = self.get_cached_piece_at(pawn_pos);
                    if let Some(p) = captured {
                        self.match_board(p).clear_bit(pawn_pos.to_square());
                        // promotion and en passant is impossible in normal play,
                        // so returning early is ok
                        self.move_piece_unchecked(move_info);
                        self.update_cache();
                        return Ok(UndoMove::new(
                            vec![
                                UndoChange::new(move_info.move_from(), Some(piece)),
                                UndoChange::new(move_info.move_to(), None),
                                UndoChange::new(pos_to.to_square(), Some(p)),
                            ],
                            Some(SpecialMove::EnPassant(pos_to.to_square())),
                        ));
                    }
                    return Err(MoveError::PieceNotFound(pawn_pos));
                } else {
                    return Err(MoveError::PieceNotFound(pos_to));
                }
            }
        };

        // make sure the piece at the new position isn't the same color
        if piece.is_same_color(&piece_at) {
            return Err(MoveError::CaptureSameColor(pos_to));
        }

        self.move_piece_unchecked(move_info);
        Ok(UndoMove::new(
            vec![
                UndoChange::new(move_info.move_from(), Some(piece)),
                UndoChange::new(move_info.move_to(), Some(piece_at)),
            ],
            None,
        ))
    }
    /// moves a pawn to the position designated in [```move_info```]
    /// checks that piece should be a pawn, but will panic if it isn't, piece
    /// checks should be from the caller
    pub fn move_pawn(&mut self, move_info: Move) -> Result<UndoMove, MoveError> {
        println!("Move Pawn");
        let from = move_info.move_from();
        let to = move_info.move_to();
        let pos_to = to.to_position();
        let pos_from = from.to_position();
        let piece = self
            .get_cached_piece_at(pos_from)
            .ok_or(MoveError::PieceNotFound(pos_from))?;
        let mut undo;

        if !piece.is_type(Pawn) {
            return Err(MoveError::IncorrectPiece(Pawn, piece.get_type()));
        }

        if pos_to.file() != pos_from.file() {
            undo = self.pawn_capture(pos_to, pos_from, piece, &move_info)?;
        }
        // Double push
        else if piece.pawn_can_double_push(&pos_from, &pos_to) {
            let step_rank = piece
                .pawn_step(pos_from.rank())
                .ok_or(MoveError::IllegalDoublePawnPush(pos_from, pos_to))?;

            let step_pos = Position::new(pos_to.file(), step_rank);

            if self.collides(&step_pos) || self.collides(&pos_to) {
                return Err(MoveError::OutOfBoundsPositionAccess(file!(), line!()));
            }
            self.move_piece_unchecked(&move_info);
            undo = UndoMove::new(
                vec![
                    UndoChange::new(move_info.move_from(), Some(piece)),
                    UndoChange::new(move_info.move_to(), None),
                ],
                None,
            );
            undo.set_special(Some(SpecialMove::EnPassant(pos_to.to_square())));
            self.en_passant_square = Some(pos_to.to_square());
        }
        // push
        else if piece.pawn_can_push(&pos_from, &pos_to) {
            if self.collides(&pos_to) {
                return Err(MoveError::IllegalCollision);
            }
            self.move_piece_unchecked(&move_info);
            undo = UndoMove::new(
                vec![
                    UndoChange::new(move_info.move_from(), Some(piece)),
                    UndoChange::new(move_info.move_to(), None),
                ],
                None,
            );
        } else {
            return Err(MoveError::Unknown(file!(), line!()));
        } // some edge case i haven't thought about

        // promotion logic
        let top_rank = match piece {
            Piece::White(_) => Rank(7),
            Piece::Black(_) => Rank(0),
        };

        if pos_to.rank() == top_rank {
            self.promote_pawn(move_info, &mut undo)?;
        }

        Ok(undo)
    }

    fn promote_pawn(&mut self, move_info: Move, undo: &mut UndoMove) -> Result<(), MoveError> {
        let pos_to = move_info.move_to();
        let piece = move_info
            .promotion()
            .ok_or(MoveError::PromotionPieceMissing(
                move_info.move_from().to_position(),
                pos_to.to_position(),
            ))?;
        println!("{piece}");
        let pawn = self
            .get_cached_piece_at(pos_to)
            .expect("Pawn should exist at pos_to");
        let board = self.match_board(pawn);

        assert!(pawn.is_same_color(&piece));
        assert!(pawn.is_type(Pawn));

        board.clear_bit(pos_to);
        let promotion_board = self.match_board(piece);
        promotion_board.set_bit(pos_to);
        self.update_cache();

        undo.set_special(Some(SpecialMove::Promotion(piece)));

        Ok(())
    }
}
