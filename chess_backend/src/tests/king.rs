#[cfg(test)]
mod tests {
    use crate::bit_board::FileChars;
    use crate::bit_board::Square;
    use crate::board::{Board, PieceType};
    use crate::game_state::GameState;
    use crate::macros;
    use crate::res_assert;
    use crate::{Move, Position};

    use super::*;

    #[test]
    fn test_white_kingside_castling_success() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w K - 0 1"; // Full kingside and queenside rights
        let mut game = GameState::from_fen(fen);

        let from = Square::from_notation(FileChars::E, 1);
        let to = Square::from_notation(FileChars::G, 1);

        let mv = Move::builder().move_from(from).move_to(to).build();

        game.board.pretty_print();
        let res = game.board.move_king(mv);
        game.board.build_cache();
        res_assert!(res);
        game.board.pretty_print();

        assert_eq!(
            game.board
                .get_cached_piece_at(Position::from_notation(FileChars::G, 1))
                .unwrap()
                .get_type(),
            PieceType::King
        );
        assert_eq!(
            game.board
                .get_cached_piece_at(Position::from_notation(FileChars::F, 1))
                .unwrap()
                .get_type(),
            PieceType::Rook
        );
    }
    #[test]
    fn test_castling_fails_without_rights() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1"; // No castling rights
        let mut game = GameState::from_fen(fen);
        println!("{:?}", game.board.castling_rights);

        let from = Square::from_position(&Position::from_notation(FileChars::E, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::G, 1));

        let mv = Move::builder().move_from(from).move_to(to).build();

        game.board.pretty_print();
        let result = game.board.move_king(mv);
        game.board.pretty_print();

        res_assert!(result, err);
    }

    #[test]
    fn test_castling_blocked_by_piece() {
        let fen = "r3k2r/8/8/8/8/8/8/R3KB1R w K - 0 1"; // Bishop on f1 blocks king path
        let mut game = GameState::from_fen(fen);

        let from = Square::from_position(&Position::from_notation(FileChars::E, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::G, 1));

        let mv = Move::builder().move_from(from).move_to(to).build();

        game.board.pretty_print();
        let result = game.board.move_king(mv);
        game.board.pretty_print();

        res_assert!(result, err);
    }

    #[test]
    fn test_castling_onto_own_piece_fails() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K1NR w K - 0 1"; // Knight on g1
        let mut game = GameState::from_fen(fen);

        let from = Square::from_position(&Position::from_notation(FileChars::E, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::G, 1));

        let mv = Move::builder().move_from(from).move_to(to).build();

        let result = game.board.move_king(mv);

        res_assert!(result, err);
    }

    #[test]
    fn test_non_king_cannot_castle() {
        let fen = "r3k2r/8/8/8/8/8/8/R3Q2R w - - 0 1"; // Queen on e1
        let mut game = GameState::from_fen(fen);

        let from = Square::from_position(&Position::from_notation(FileChars::E, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::G, 1));

        let mv = Move::builder().move_from(from).move_to(to).build();

        let result = game.board.move_king(mv);

        res_assert!(result, err);
    }

    #[test]
    fn test_black_kingside_castling_success() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b k - 0 1";
        let mut game = GameState::from_fen(fen);
        game.board.pretty_print();

        let from = Square::from_notation(FileChars::E, 8);
        let to = Square::from_notation(FileChars::G, 8);

        let mv = Move::builder().move_from(from).move_to(to).build();
        let res = game.board.move_king(mv);
        game.board.build_cache();
        res_assert!(res);
        game.board.pretty_print();

        assert_eq!(
            game.board
                .get_cached_piece_at(Position::from_notation(FileChars::G, 8))
                .unwrap()
                .get_type(),
            PieceType::King
        );
        assert_eq!(
            game.board
                .get_cached_piece_at(Position::from_notation(FileChars::F, 8))
                .unwrap()
                .get_type(),
            PieceType::Rook
        );
    }
    #[test]
    fn test_black_queenside_castling_success() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b q - 0 1";
        let mut game = GameState::from_fen(fen);

        let from = Square::from_notation(FileChars::E, 8);
        let to = Square::from_notation(FileChars::C, 8);

        let mv = Move::builder().move_from(from).move_to(to).build();

        let res = game.board.move_king(mv);
        game.board.build_cache();
        res_assert!(res);

        assert_eq!(
            game.board
                .get_cached_piece_at(Position::from_notation(FileChars::C, 8))
                .unwrap()
                .get_type(),
            PieceType::King
        );
        assert_eq!(
            game.board
                .get_cached_piece_at(Position::from_notation(FileChars::D, 8))
                .unwrap()
                .get_type(),
            PieceType::Rook
        );
    }
    #[test]
    fn test_black_kingside_castling_blocked_by_piece() {
        let fen = "r3k1nr/8/8/8/8/8/8/R3K2R b k - 0 1"; // Knight on g8 blocking kingside
        let mut game = GameState::from_fen(fen);

        let from = Square::from_position(&Position::from_notation(FileChars::E, 8));
        let to = Square::from_position(&Position::from_notation(FileChars::G, 8));

        let mv = Move::builder().move_from(from).move_to(to).build();

        let result = game.board.move_king(mv);

        res_assert!(result, err); // ❌ should fail due to blocking knight
    }
    #[test]
    fn test_black_queenside_castling_blocked_by_piece() {
        let fen = "rnbqk2r/8/8/8/8/8/8/R3K2R b q - 0 1"; // Bishop on c8 blocks queenside
        let mut game = GameState::from_fen(fen);

        let from = Square::from_position(&Position::from_notation(FileChars::E, 8));
        let to = Square::from_position(&Position::from_notation(FileChars::C, 8));

        let mv = Move::builder().move_from(from).move_to(to).build();

        let result = game.board.move_king(mv);

        res_assert!(result, err); // ❌ should fail due to bishop in the way
    }
    #[test]
    fn test_black_kingside_castling_without_rights() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R b - - 0 1"; // No castling rights at all
        let mut game = GameState::from_fen(fen);

        let from = Square::from_position(&Position::from_notation(FileChars::E, 8));
        let to = Square::from_position(&Position::from_notation(FileChars::G, 8));

        let mv = Move::builder().move_from(from).move_to(to).build();

        let result = game.board.move_king(mv);

        res_assert!(result, err); // ❌ should fail — no rights
    }
}
