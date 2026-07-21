#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::*;

    use super::*;
    #[test]
    fn test_rook_simple_straight_move() {
        const FEN: &str = "8/8/8/8/8/8/8/R7 w - - 0 1"; // White rook on a1
        let mut game = GameState::from_fen(FEN);

        let from = Square::from_position(&Position::from_notation(FileChars::A, 1)); // a1
        let to = Square::from_position(&Position::from_notation(FileChars::A, 4)); // a4

        let mv = Move::builder().move_from(from).move_to(to).build();
        let undo = game.board.move_rook(&mv);
        assert!(undo.is_ok(), "Rook should be able to move vertically");
    }

    #[test]
    fn test_rook_capture_enemy_piece() {
        const FEN: &str = "8/8/8/8/8/8/8/R6p w - - 0 1"; // White rook a1, Black pawn h1
        let mut game = GameState::from_fen(FEN);

        let from = Square::from_position(&Position::from_notation(FileChars::A, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::H, 1));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let undo = game.board.move_rook(&mv);
        assert!(
            undo.is_ok(),
            "Rook should be able to capture enemy pawn horizontally"
        );
    }

    #[test]
    fn test_rook_cannot_capture_friendly_piece() {
        const FEN: &str = "8/8/8/8/8/8/8/R6P w - - 0 1"; // White rook a1, White pawn h1
        let mut game = GameState::from_fen(FEN);

        let from = Square::from_position(&Position::from_notation(FileChars::A, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::H, 1));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let undo = game.board.move_rook(&mv);
        assert!(
            undo.is_err(),
            "Rook should not be able to capture friendly pawn"
        );
    }

    #[test]
    fn test_rook_cannot_jump_over_piece() {
        const FEN: &str = "8/8/8/8/8/8/8/R4P2 w - - 0 1"; // White rook a1, White pawn f1
        let mut game = GameState::from_fen(FEN);

        let from = Square::from_position(&Position::from_notation(FileChars::A, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::H, 1));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let undo = game.board.move_rook(&mv);
        assert!(
            undo.is_err(),
            "Rook should not be able to jump over a blocking piece"
        );
    }

    #[test]
    fn test_rook_cannot_move_diagonally() {
        const FEN: &str = "8/8/8/8/8/8/8/R7 w - - 0 1"; // White rook a1
        let mut game = GameState::from_fen(FEN);

        let from = Square::from_position(&Position::from_notation(FileChars::A, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::D, 4)); // illegal diagonal move

        let mv = Move::builder().move_from(from).move_to(to).build();
        let undo = game.board.move_rook(&mv);
        assert!(undo.is_err(), "Rook should not be able to move diagonally");
    }

    #[test]
    fn test_rook_from_empty_square_fails() {
        const FEN: &str = "8/8/8/8/8/8/8/8 w - - 0 1"; // Empty board
        let mut game = GameState::from_fen(FEN);

        let from = Square::from_position(&Position::from_notation(FileChars::A, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::A, 4));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let undo = game.board.move_rook(&mv);
        assert!(
            undo.is_err(),
            "Should not be able to move a rook from an empty square"
        );
    }

    #[test]
    fn test_wrong_piece_type_fails() {
        const FEN: &str = "8/8/8/8/8/8/8/N7 w - - 0 1"; // White knight on a1
        let mut game = GameState::from_fen(FEN);

        let from = Square::from_position(&Position::from_notation(FileChars::A, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::A, 4));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let undo = game.board.move_rook(&mv);
        assert!(
            undo.is_err(),
            "Should not be able to move a knight as a rook"
        );
    }
}
