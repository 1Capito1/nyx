#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_bishop_valid_diagonal_move() {
        const FEN: &str = "8/8/8/8/8/8/8/2B5 w - - 0 1"; // Bishop on c1
        let mut game = GameState::from_fen(FEN);

        let from = Square::from_position(&Position::from_notation(FileChars::C, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::F, 4));

        let mv = Move::builder().move_from(from).move_to(to).build();
        res_assert!(game.board.move_bishop(&mv));
    }
    #[test]
    fn test_bishop_cannot_move_straight_line() {
        const FEN: &str = "8/8/8/8/8/8/8/2B5 w - - 0 1";
        let mut game = GameState::from_fen(FEN);

        let from = Square::from_position(&Position::from_notation(FileChars::C, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::C, 4));

        let mv = Move::builder().move_from(from).move_to(to).build();
        assert!(game.board.move_bishop(&mv).is_err());
    }
    #[test]
    fn test_bishop_blocked_by_own_piece() {
        const FEN: &str = "8/8/8/8/8/4P3/8/2B5 w - - 0 1"; // Bishop c1, pawn d2
        let mut game = GameState::from_fen(FEN);
        game.board.pretty_print();
        let e3 = *Position::from_notation(FileChars::E, 3).to_square() as usize;
        println!("{:?}", game.board.get_cache()[e3]);

        let from = Square::from_position(&Position::from_notation(FileChars::C, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::F, 4));

        let mv = Move::builder().move_from(from).move_to(to).build();
        let uv = game.board.move_bishop(&mv);
        game.board.pretty_print();

        assert!(uv.is_err());
    }
    #[test]
    fn test_bishop_can_capture_enemy_piece() {
        const FEN: &str = "8/8/8/8/8/8/5n2/2B5 w - - 0 1"; // Bishop on c1, black knight on f4
        let mut game = GameState::from_fen(FEN);

        let from = Square::from_position(&Position::from_notation(FileChars::C, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::F, 4));

        let mv = Move::builder().move_from(from).move_to(to).build();
        res_assert!(game.board.move_bishop(&mv));
    }
    #[test]
    fn test_bishop_cannot_capture_friendly_piece() {
        const FEN: &str = "8/8/8/8/5N2/8/8/2B5 w - - 0 1"; // Bishop on c1, white knight on f4
        let mut game = GameState::from_fen(FEN);
        game.board.pretty_print();

        let from = Square::from_position(&Position::from_notation(FileChars::C, 1));
        let to = Square::from_position(&Position::from_notation(FileChars::F, 4));

        let mv = Move::builder().move_from(from).move_to(to).build();
        assert!(game.board.move_bishop(&mv).is_err());
    }

    #[test]
    fn test_bishop_move_to_edge() {
        const FEN: &str = "rnbqkbnr/ppp1ppp1/3p4/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 1";
        let mut game = GameState::from_fen(FEN);

        let from = Position::from_notation(FileChars::C, 1).to_square();
        let to = Position::from_notation(FileChars::H, 6).to_square();

        let mut rep = game.board.board_rep().clone();

        rep[from.0 as usize] = None;
        rep[to.0 as usize] = Some(Piece::White(PieceType::Bishop));

        game.board.pretty_print();

        let mv = Move::builder().move_from(from).move_to(to).build();

        let result = game.board.move_bishop(&mv);

        res_assert!(result);

        game.board.pretty_print();

        assert_eq!(rep, *game.board.board_rep());
    }
}
