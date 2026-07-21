#![cfg(test)]
mod tests {
    use crate::{
        STARTING_POSITION_FEN,
        board::{Board, FenNotation},
        game_state::GameState,
    };

    #[test]
    fn depth_1() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(1), 20);
    }

    #[test]
    fn depth_1_castles() {
        let mut board =
            GameState::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1").board;
        board.pretty_print();
        assert_eq!(board.perft_divide(1), 25);
    }
    #[test]
    fn depth_2() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(2), 400);
    }
    #[test]
    fn depth_3() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft_divide(3), 8902);
    }
    #[test]
    fn depth_4() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(4), 197_281);
    }
    #[test]
    fn depth_5() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft_divide(5), 4_865_609);
    }
    #[test]
    #[ignore = "use when debugging sliding pieces"]
    fn depth_5_pos3() {
        let mut board = GameState::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ").board;
        assert_eq!(board.perft(5), 674624);
    }
    #[test]
    fn depth_6() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft_divide(6), 119_060_324);
    }
    #[test]
    fn depth_6_after_e2e4() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(
            board.perft_divide_after(&["E2E4", "C7C5", "F1A6", "C5C4", "D2D4"], 1),
            119_060_324
        );
    }
    #[test]
    #[ignore]
    fn depth_7() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(7), 3_195_901_860);
    }
    #[test]
    #[ignore]
    fn depth_8() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(8), 84_998_978_956);
    }
}
