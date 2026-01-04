#[cfg(test)]
mod tests {
    use crate::{
        bit_board::{FileChars, Position},
        board::{Board, PieceType},
        game_state::GameState,
        tests::check,
    };

    fn check_test(fen: &str, king_position: Position, expected_result: bool) {
        let game = GameState::from_fen(fen);
        assert_eq!(
            game.board
                .is_check(&king_position, crate::board::Piece::Black(PieceType::Pawn)),
            expected_result
        );
    }

    #[test]
    pub fn rook_same_file_check() {
        let fen = "8/8/8/3K2r1/8/8/8/8 w - - 0 1";
        let king_position = Position::from_notation(FileChars::D, 5);
        check_test(fen, king_position, true)
    }

    #[test]
    pub fn rook_same_rank_check() {
        let fen = "8/3r4/8/3K4/8/8/8/8 w - - 0 1";
        let king_position = Position::from_notation(FileChars::D, 5);
        check_test(fen, king_position, true)
    }

    #[test]
    pub fn rook_no_check() {
        let fen = "8/4r3/8/3K4/8/8/8/8 w - - 0 1";
        let king_position = Position::from_notation(FileChars::D, 5);
        check_test(fen, king_position, false);
    }

    #[test]
    pub fn rook_check_blocked() {
        let fen = "8/3r4/3P4/3K4/8/8/8/8 w - - 0 1";
        let king_position = Position::from_notation(FileChars::D, 5);
        check_test(fen, king_position, false);
    }
    #[test]
    pub fn bishop_check() {
        let fen = "8/5b2/8/3K4/8/8/8/8 w - - 0 1";
        let king_position = Position::from_notation(FileChars::D, 5);
        check_test(fen, king_position, true)
    }
    #[test]
    pub fn bishop_check_blocked() {
        let fen = "8/5b2/4P3/3K4/8/8/8/8 w - - 0 1";
        let king_position = Position::from_notation(FileChars::D, 5);
        check_test(fen, king_position, false)
    }
    #[test]
    pub fn bishop_no_check() {
        let fen = "8/8/5b2/3K4/8/8/8/8 w - - 0 1";
        let king_position = Position::from_notation(FileChars::D, 5);
        check_test(fen, king_position, false)
    }
    #[test]
    pub fn knight_check() {
        let fen = "8/8/5n2/3K4/8/8/8/8 w - - 0 1";
        let king_position = Position::from_notation(FileChars::D, 5);
        check_test(fen, king_position, true)
    }
    #[test]
    pub fn knight_no_check() {
        let fen = "8/8/4n3/3K4/8/8/8/8 w - - 0 1";
        let king_position = Position::from_notation(FileChars::D, 5);
        check_test(fen, king_position, false)
    }

    #[test]
    pub fn knight_check_board_edge() {
        let fen = "8/8/5n2/7K/8/8/8/8 w - - 0 1";
        let king_position = Position::from_notation(FileChars::H, 5);
        check_test(fen, king_position, true)
    }

    #[test]
    pub fn black_pawn_check() {
        let fen = "8/8/2p5/3K4/8/8/8/8 w - - 0 1";
        let king_position = Position::from_notation(FileChars::D, 5);
        check_test(fen, king_position, true)
    }

    // TODO: is correct, but check_test uses opposite color
    // #[test]
    // pub fn white_pawn_check() {
    //     let fen = "8/8/8/3k4/2P5/8/8/8 w - - 0 1";
    //     let king_position = Position::from_notation(FileChars::D, 5);
    //     check_test(fen, king_position, true)
    // }
}
