#![cfg(test)]
mod tests {
    use crate::{
        board::{Board, FenNotation},
        game_state::GameState,
        STARTING_POSITION_FEN,
    };

    #[test]
    fn depth_1() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(1), 20);
    }
    #[test]
    fn depth_2() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(2), 400);
    }
    #[test]
    fn depth_3() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(3), 8902);
    }
    #[test]
    fn depth_4() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(4), 197_281);
    }
    #[test]
    fn depth_5() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(5), 4_865_609);
    }
    #[test]
    fn depth_6() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(6), 119_060_324);
    }
    #[test]
    fn depth_7() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(7), 3_195_901_860);
    }
    #[test]
    fn depth_8() {
        let mut board = GameState::from_fen(STARTING_POSITION_FEN).board;
        assert_eq!(board.perft(8), 84_998_978_956);
    }
}
