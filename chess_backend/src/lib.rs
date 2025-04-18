use self::bit_board::{Position, Square};
use self::game_state::GameState;
use self::move_gen::Move;
use self::piece::{Piece, PieceType};

mod bit_board;
mod board;
mod game_state;
mod move_gen;
mod piece;

// Starting position in FEN notation
pub const STARTING_POSITION_FEN: &'static str =
"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


#[cfg(test)]
mod test {
    use crate::board::Board;

    use super::*;
    #[test]
    fn starting_position_fen() {
        const BOARD: [char; 64] = [
            'R','N','B','Q','K','B','N','R',
            'P','P','P','P','P','P','P','P',
            '.','.','.','.','.','.','.','.',
            '.','.','.','.','.','.','.','.',
            '.','.','.','.','.','.','.','.',
            '.','.','.','.','.','.','.','.',
            'p','p','p','p','p','p','p','p',
            'r','n','b','q','k','b','n','r',
        ];
        let game_state = game_state::GameState::from_fen(STARTING_POSITION_FEN);
        let board = game_state.board.pretty_print();

        assert_eq!(board, BOARD);
    }
    #[test]
    fn midgame_position_fen() {
    let expected: [char; 64] = [
        // Rank 1
        'R', '.', 'B', 'Q', '.', 'R', 'K', '.',
        // Rank 2
        'P', 'P', 'B', 'N', '.', 'P', 'P', 'P',
        // Rank 3
        '.', '.', 'P', '.', '.', 'N', '.', '.',
        // Rank 4
        '.', 'b', '.', 'P', 'P', '.', '.', '.',
        // Rank 5
        '.', '.', '.', 'p', 'p', '.', '.', '.',
        // Rank 6
        '.', '.', 'n', '.', '.', 'n', '.', '.',
        // Rank 7
        'p', 'p', 'p', '.', '.', 'p', 'p', 'p',
        // Rank 8
        'r', '.', 'b', 'q', '.', 'r', 'k', '.',
    ];
        let game_state = game_state::GameState::from_fen("r1bq1rk1/ppp2ppp/2n2n2/3pp3/1b1PP3/2P2N2/PPBN1PPP/R1BQ1RK1 w - - 0 8");
        let board = game_state.board.pretty_print();

        assert_eq!(board, expected);
    }
    #[test]
    fn test_shift_56() {
        let shift = 56u8;
        let mask: u64 = 1u64 << shift;
        println!("{:064b}", mask);
    }
}

#[test]
fn test_move_piece_unchecked_basic() {
    // Start from simple board: white pawn on e2 (file 4, rank 1)
    let mut game = GameState::from_fen("8/8/8/8/8/8/4P3/8 w - - 0 1");

    let from = Square::from_position(&Position::new(4, 1)); // e2
    let to = Square::from_position(&Position::new(4, 3));   // e4

    let piece_before = game.board.get_cached_piece_at(&from.to_position());
    println!("Asserting on to_pos: file: {}, rank: {}", to.to_position().file(), to.to_position().rank());
    assert_eq!(piece_before, Some(Piece::White(PieceType::Pawn)));

    let piece_at_dest_before = game.board.get_cached_piece_at(&to.to_position());
    assert_eq!(piece_at_dest_before, None);

    let undo = game.board.move_piece_unchecked(
        Move::builder()
        .move_from(from)
        .move_to(to)
        .build()
    );

    // Check source is empty
    assert_eq!(game.board.get_cached_piece_at(&from.to_position()), None);

    // Check destination now has the piece
    assert_eq!(game.board.get_cached_piece_at(&to.to_position()), Some(Piece::White(PieceType::Pawn)));

    // Check undo metadata
    assert_eq!(undo.move_from(), from);
    assert_eq!(undo.move_to(), to);
    assert!(undo.captured_piece().is_none());
}
