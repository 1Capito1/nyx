#[cfg(test)]
mod tests {
    use crate::bit_board::{FileChars, Square};
    use crate::board::{Piece, PieceType};
    use crate::game_state::GameState;
    use crate::{File, Move, Position, Rank, res_assert};

    #[test]
    fn test_knight_simple_move() {
        // Knight on d4, moving to e6
        let mut game = GameState::from_fen("8/8/8/8/3N4/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::D, 4));
        let to = Square::from_position(&Position::from_notation(FileChars::E, 6));

        let mv = Move::builder().move_from(from).move_to(to).build();
        res_assert!(game.board.move_knight(&mv));
    }

    #[test]
    fn test_knight_capture_enemy() {
        // White knight on d4, black pawn on e6
        let mut game = GameState::from_fen("8/8/4p3/8/3N4/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::D, 4));
        let to = Square::from_position(&Position::from_notation(FileChars::E, 6));

        let mv = Move::builder().move_from(from).move_to(to).build();
        res_assert!(game.board.move_knight(&mv));
    }

    #[test]
    fn test_knight_cannot_capture_same_color() {
        // White knight on d4, white pawn on e6
        let mut game = GameState::from_fen("8/8/4P3/8/3N4/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::D, 4));
        let to = Square::from_position(&Position::from_notation(FileChars::E, 6));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_knight(&mv);
        assert!(result.is_err(), "Knight should not capture same color");
    }

    #[test]
    fn test_knight_invalid_move() {
        // Knight on d4 trying to move like a bishop
        let mut game = GameState::from_fen("8/8/8/8/3N4/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::D, 4));
        let to = Square::from_position(&Position::from_notation(FileChars::F, 6)); // wrong move

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_knight(&mv);
        assert!(result.is_err(), "Knight should not move invalidly");
    }

    #[test]
    fn test_knight_edge_file_a() {
        // Knight on a4 trying illegal move wrapping to h3
        let mut game = GameState::from_fen("8/8/8/8/N7/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::A, 4));
        let to = Square::from_position(&Position::from_notation(FileChars::H, 3));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_knight(&mv);
        assert!(result.is_err(), "Knight should not wrap across board");
    }

    #[test]
    fn test_knight_edge_file_h() {
        // Knight on h4 trying illegal move wrapping to a5
        let mut game = GameState::from_fen("8/8/8/8/7N/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::H, 4));
        let to = Square::from_position(&Position::from_notation(FileChars::A, 5));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_knight(&mv);
        assert!(result.is_err(), "Knight should not wrap across board");
    }

    #[test]
    fn test_knight_edge_rank_1() {
        // Knight on d1 moving upward
        let mut game = GameState::from_fen("8/8/8/8/8/8/8/3N4 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::D, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::E, 3)); // legal

        let mv = Move::builder().move_from(from).move_to(to).build();
        res_assert!(game.board.move_knight(&mv));
    }

    #[test]
    fn test_knight_edge_rank_8() {
        // Knight on d8 moving downward
        let mut game = GameState::from_fen("3N4/8/8/8/8/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::D, 8));
        let to = Square::from_position(&Position::from_notation(FileChars::E, 6)); // legal

        let mv = Move::builder().move_from(from).move_to(to).build();
        res_assert!(game.board.move_knight(&mv));
    }
}
