#[cfg(test)]
mod tests {
    use crate::bit_board::{FileChars, Square};
    use crate::board::{Piece, PieceType};
    use crate::game_state::GameState;
    use crate::{res_assert, File, Move, Position, Rank};

    #[test]
    fn test_white_pawn_push_and_double_push() {
        const DEFAULT_STATE: &str = "8/8/8/8/8/8/4P3/8 w - - 0 1";
        // Setup board with a white pawn on e2 (file 4, rank 1)
        let mut game = GameState::from_fen(DEFAULT_STATE);

        let from = Square::from_position(&Position::from_notation(FileChars::E, 2)); // e2
        let to_single = Square::from_position(&Position::from_notation(FileChars::E, 3)); // e3
        let to_double = Square::from_position(&Position::from_notation(FileChars::E, 4)); // e4

        // Single push
        let mv = Move::builder().move_from(from).move_to(to_single).build();
        let undo = game.board.move_pawn(mv);
        assert!(undo.is_ok(), "Pawn should be able to single push");

        // Reset game state
        let mut game = GameState::from_fen(DEFAULT_STATE);

        // Double push
        let mv = Move::builder().move_from(from).move_to(to_double).build();
        let undo = game.board.move_pawn(mv);
        assert!(undo.is_ok(), "Pawn should be able to double push");
    }

    #[test]
    fn test_white_pawn_double_push_blocked() {
        // Pawn on e2, blocking piece on e3
        let mut game = GameState::from_fen("8/8/8/8/8/4P3/4P3/8 w - - 0 1");
        game.board.pretty_print();

        let from = Square::from_position(&Position::new(File(4), Rank(1))); // e2
        let to_double = Square::from_position(&Position::new(File(4), Rank(3))); // e4

        let mv = Move::builder().move_from(from).move_to(to_double).build();
        let undo = game.board.move_pawn(mv);
        game.board.pretty_print();
        assert!(
            undo.is_err(),
            "Pawn double push should fail due to blocking piece"
        );
    }

    #[test]
    fn test_white_pawn_capture_right() {
        let a = File(4);
        let b = File(4);
        // White pawn on e5, black pawn on f6
        let mut game = GameState::from_fen("8/8/5p2/4P3/8/8/8/8 w - - 0 1");

        game.board.pretty_print();

        let from = Square::from_position(&Position::from_notation(FileChars::E, 5));
        let to = Square::from_position(&Position::from_notation(FileChars::F, 6));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_pawn(mv);

        game.board.pretty_print();

        res_assert!(result);
    }

    #[test]
    fn test_white_pawn_capture_left() {
        // White pawn on e4, black pawn on d5
        let mut game = GameState::from_fen("8/8/8/3p4/4P3/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::E, 4)); // e4
        let to = Square::from_position(&Position::from_notation(FileChars::D, 5)); // d5

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_pawn(mv);

        res_assert!(result);
    }

    #[test]
    fn test_black_pawn_capture_right() {
        // Black pawn on e5, white pawn on d4
        let mut game = GameState::from_fen("8/8/8/4p3/3P4/8/8/8 b - - 0 1");

        game.board.pretty_print();

        let from = Square::from_position(&Position::from_notation(FileChars::E, 5));
        let to = Square::from_position(&Position::from_notation(FileChars::D, 4));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_pawn(mv);

        res_assert!(result);
    }

    #[test]
    fn test_black_pawn_capture_left() {
        // Black pawn on e5, white pawn on f4
        let mut game = GameState::from_fen("8/8/8/4p3/5P2/8/8/8 b - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::E, 5));
        let to = Square::from_position(&Position::from_notation(FileChars::F, 4));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_pawn(mv);

        res_assert!(result);
    }

    #[test]
    fn test_pawn_illegal_diagonal_without_target() {
        // White pawn on e4, nothing on d5
        let mut game = GameState::from_fen("8/8/8/3P4/8/8/8/8 w - - 0 1");
        game.board.pretty_print();

        let from = Square::from_position(&Position::new(File(3), Rank(4))); // d4
        let to = Square::from_position(&Position::new(File(2), Rank(5))); // c5

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_pawn(mv);

        game.board.pretty_print();

        assert!(result.is_err());
    }

    #[test]
    fn test_pawn_cannot_capture_same_color() {
        // White pawn on e4, white pawn on f5
        let mut game = GameState::from_fen("8/8/5P2/4P3/8/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::new(File(4), Rank(4))); // e4
        let to = Square::from_position(&Position::new(File(5), Rank(5))); // f5

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_pawn(mv);

        assert!(result.is_err());
    }

    #[test]
    fn test_white_pawn_cannot_capture_left_on_file_a() {
        // White pawn on a4, nothing to capture
        let mut game = GameState::from_fen("8/8/8/8/P7/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::A, 4));
        let to = Square::from_position(&Position::from_notation(FileChars::H, 5)); // totally wrong direction

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_pawn(mv);

        assert!(result.is_err())
    }

    #[test]
    fn test_white_pawn_capture_from_a4_to_b5() {
        // White pawn on a4, black pawn on b5
        let mut game = GameState::from_fen("8/8/8/1p6/P7/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::A, 4)); // a4
        let to = Square::from_position(&Position::from_notation(FileChars::B, 5)); // b5

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_pawn(mv);

        res_assert!(result);
    }

    #[test]
    fn test_white_pawn_capture_from_h4_to_g5() {
        // White pawn on h4, black pawn on g5
        let mut game = GameState::from_fen("8/8/8/6p1/7P/8/8/8 w - - 0 1");
        game.board.pretty_print();

        let from = Square::from_position(&Position::from_notation(FileChars::H, 4)); // h4
        let to = Square::from_position(&Position::from_notation(FileChars::G, 5)); // g5

        let mv = Move::builder().move_from(from).move_to(to).build();
        let result = game.board.move_pawn(mv);

        game.board.pretty_print();

        res_assert!(result);
    }

    #[test]
    pub fn test_pawn_promotion_with_capture() {
        // Setup: White pawn on H7, black rook on G8
        let mut game = GameState::from_fen("6r1/7P/8/8/8/8/8/8 w - - 0 1");
        println!("Before promotion with capture:");
        game.board.pretty_print();

        let expected = GameState::from_fen("6Q1/8/8/8/8/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::H, 7));
        let to = Square::from_position(&Position::from_notation(FileChars::G, 8));

        let move_info = Move::builder()
            .move_from(from)
            .move_to(to)
            .promotion(Piece::White(PieceType::Queen))
            .build();

        let result = game.board.move_pawn(move_info);
        game.board.build_cache();
        res_assert!(result);

        println!("After promotion with capture:");
        game.board.pretty_print();

        // Check that the piece at G8 is now a white queen
        let promoted = game.board.get_cached_piece_at(to);
        assert_eq!(
            promoted,
            Some(Piece::White(PieceType::Queen)),
            "Expected a white queen at G8 after promotion"
        );

        // Optional: Deep compare board state against expected (if you implemented eq)
        // assert_eq!(game.board, expected.board, "Board state mismatch after promotion");
    }

    #[test]
    pub fn test_white_promotion_without_capture() {
        let mut game = GameState::from_fen("8/7P/8/8/8/8/8/8 w - - 0 1");
        println!("Before white promotion without capture:");
        game.board.pretty_print();

        let expected = GameState::from_fen("7Q/8/8/8/8/8/8/8 w - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::H, 7));
        let to = Square::from_position(&Position::from_notation(FileChars::H, 8));

        let mv = Move::builder()
            .move_from(from)
            .move_to(to)
            .promotion(Piece::White(PieceType::Queen))
            .build();

        let result = game.board.move_pawn(mv);
        game.board.build_cache();
        res_assert!(result);

        println!("After white promotion without capture:");
        game.board.pretty_print();

        let promoted = game.board.get_cached_piece_at(to);
        assert_eq!(promoted, Some(Piece::White(PieceType::Queen)));
    }

    #[test]
    pub fn test_black_promotion_with_capture() {
        let mut game = GameState::from_fen("8/8/8/8/8/8/p7/1P6 b - - 0 1");
        println!("Before black promotion with capture:");
        game.board.pretty_print();

        let expected = GameState::from_fen("8/8/8/8/8/8/8/1q6 b - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::A, 2));
        let to = Square::from_position(&Position::from_notation(FileChars::B, 1));

        let mv = Move::builder()
            .move_from(from)
            .move_to(to)
            .promotion(Piece::Black(PieceType::Queen))
            .build();

        let result = game.board.move_pawn(mv);
        res_assert!(result);
        game.board.build_cache();

        println!("After black promotion with capture:");
        game.board.pretty_print();

        let promoted = game.board.get_cached_piece_at(to);
        assert_eq!(promoted, Some(Piece::Black(PieceType::Queen)));
    }

    #[test]
    #[should_panic]
    pub fn test_promotion_without_specifying_piece_should_fail() {
        let mut game = GameState::from_fen("8/7P/8/8/8/8/8/8 w - - 0 1");
        println!("Before illegal promotion without specifying piece:");
        game.board.pretty_print();

        let from = Square::from_position(&Position::from_notation(FileChars::H, 7));
        let to = Square::from_position(&Position::from_notation(FileChars::H, 8));

        let mv = Move::builder()
            .move_from(from)
            .move_to(to)
            // no promotion() call here
            .build();

        let result = game.board.move_pawn(mv);
        println!("After illegal promotion without specifying piece:");
        game.board.pretty_print();
        res_assert!(result);
    }

    #[test]
    pub fn test_black_promotion_without_capture() {
        let mut game = GameState::from_fen("8/8/8/8/8/8/p7/8 b - - 0 1");
        println!("Before black promotion without capture:");
        game.board.pretty_print();

        let expected = GameState::from_fen("q7/8/8/8/8/8/8/8 b - - 0 1");

        let from = Square::from_position(&Position::from_notation(FileChars::A, 2)); // a2
        let to = Square::from_position(&Position::from_notation(FileChars::A, 1)); // a1

        let mv = Move::builder()
            .move_from(from)
            .move_to(to)
            .promotion(Piece::Black(PieceType::Queen))
            .build();

        let result = game.board.move_pawn(mv);

        game.board.build_cache();

        println!("After black promotion without capture:");
        game.board.pretty_print();

        let promoted = game.board.get_cached_piece_at(to);
        assert_eq!(
            promoted,
            Some(Piece::Black(PieceType::Queen)),
            "Expected a black queen at A1 after promotion"
        );
    }

    #[test]
    fn test_white_en_passant_capture() {
        let mut game = GameState::from_fen("8/8/8/3pP3/8/8/8/8 w - d6 0 1");
        game.board.pretty_print();
        let from = Position::from_notation(FileChars::E, 5);
        let to = Position::from_notation(FileChars::D, 6);
        let mv = Move::builder()
            .move_from(from.to_square())
            .move_to(to.to_square())
            .build();

        let result = game.board.move_pawn(mv);
        game.board.build_cache();
        game.board.pretty_print();
        res_assert!(result);

        assert_eq!(game.board.get_cached_piece_at(from), None); // from
        assert_eq!(
            game.board
                .get_cached_piece_at(Position::from_notation(FileChars::D, 5)),
            None
        ); // captured
        assert_eq!(
            game.board.get_cached_piece_at(to),
            Some(Piece::White(PieceType::Pawn))
        ); // to
    }
}
