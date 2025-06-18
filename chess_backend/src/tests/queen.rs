#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_queen_straight_and_diagonal_moves() {
        const DEFAULT_STATE: &str = "8/8/8/3Q4/8/8/8/8 w - - 0 1"; // Queen at d5
        let from = Square::from_position(&Position::from_notation(FileChars::D, 5));

        // Straight move (d5 → d8)
        let mut game = GameState::from_fen(DEFAULT_STATE);
        let to = Square::from_position(&Position::from_notation(FileChars::D, 8));
        let mv = Move::builder().move_from(from).move_to(to).build();
        res_assert!(
            game.board.move_queen(mv)
        );

        // Diagonal move (d5 → g8)
        let mut game = GameState::from_fen(DEFAULT_STATE);
        let to = Square::from_position(&Position::from_notation(FileChars::G, 8));
        let mv = Move::builder().move_from(from).move_to(to).build();
        res_assert!(
            game.board.move_queen(mv)
        );
    }

    #[test]
    fn test_queen_blocked_by_piece() {
        const BLOCKED_STATE: &str = "8/8/3P4/3Q4/8/8/8/8 w - - 0 1"; // Queen at d5, pawn at d6
        let from = Square::from_position(&Position::from_notation(FileChars::D, 5));
        let to = Square::from_position(&Position::from_notation(FileChars::D, 7));

        let mut game = GameState::from_fen(BLOCKED_STATE);
        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_queen(mv);
        assert!(result.is_err(), "Queen should not move through pieces");
    }

    #[test]
    fn test_queen_invalid_direction() {
        const DEFAULT_STATE: &str = "8/8/8/3Q4/8/8/8/8 w - - 0 1"; // Queen at d5
        let from = Square::from_position(&Position::from_notation(FileChars::D, 5));
        let to = Square::from_position(&Position::from_notation(FileChars::E, 7)); // Not straight or diagonal

        let mut game = GameState::from_fen(DEFAULT_STATE);
        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_queen(mv);
        assert!(result.is_err(), "Queen can't move in invalid direction");
    }

    #[test]
    fn test_queen_capture_same_color() {
        const SAME_COLOR_STATE: &str = "8/3P4/8/3Q4/8/8/8/8 w - - 0 1"; // Queen at d5, white pawn at d7
        let from = Square::from_position(&Position::from_notation(FileChars::D, 5));
        let to = Square::from_position(&Position::from_notation(FileChars::D, 7));

        let mut game = GameState::from_fen(SAME_COLOR_STATE);
        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_queen(mv);
        assert!(result.is_err(), "Queen shouldn't capture same color");
    }

    #[test]
    fn test_queen_capture_enemy() {
        const CAPTURE_STATE: &str = "6p1/8/8/3Q4/8/8/8/8 w - - 0 1"; // Queen at d5, black pawn at g8
        let from = Square::from_position(&Position::from_notation(FileChars::D, 5));
        let to = Square::from_position(&Position::from_notation(FileChars::G, 8));

        let mut game = GameState::from_fen(CAPTURE_STATE);
        let mv = Move::builder().move_from(from).move_to(to).build();
        res_assert!(
            game.board.move_queen(mv)
        );
    }
}
