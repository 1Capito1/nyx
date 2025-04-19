use self::bit_board::{File, FileChars, Position, Rank, Square};
use self::game_state::GameState;
use self::moves::Move;
use self::piece::{Piece, PieceType};

mod bit_board;
mod board;
mod game_state;
mod moves;
mod piece;
mod move_gen;

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

    let from = Square::from_position(&Position::new(File(4), Rank(1))); // e2
    let to = Square::from_position(&Position::new(File(4), Rank(3)));   // e4

    let piece_before = game.board.get_cached_piece_at(&from.to_position());
    println!("Asserting on to_pos: file: {}, rank: {}", to.to_position().file(), to.to_position().rank());
    game.board.pretty_print();
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

#[test]
fn test_white_pawn_push_and_double_push() {
    // Setup board with a white pawn on e2 (file 4, rank 1)
    let mut game = GameState::from_fen("8/8/8/8/8/8/4P3/8 w - - 0 1");

    let from = Square::from_position(&Position::new(File(4), Rank(1))); // e2
    let to_single = Square::from_position(&Position::new(File(4), Rank(2))); // e3
    let to_double = Square::from_position(&Position::new(File(4), Rank(3))); // e4

    // Single push
    let mv = Move::builder().move_from(from).move_to(to_single).build();
    let undo = game.board.move_pawn(mv);
    assert!(undo.is_some(), "Pawn should be able to single push");

    // Reset game state
    let mut game = GameState::from_fen("8/8/8/8/8/8/4P3/8 w - - 0 1");

    // Double push
    let mv = Move::builder().move_from(from).move_to(to_double).build();
    let undo = game.board.move_pawn(mv);
    assert!(undo.is_some(), "Pawn should be able to double push");
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
    assert!(undo.is_none(), "Pawn double push should fail due to blocking piece");
}
#[test]

fn test_white_pawn_capture_right() {
    let a = File(4);
    let b = File(4);
    dbg!(a == b); // should print true
    // White pawn on e5, black pawn on f6
    let mut game = GameState::from_fen("8/8/5p2/4P3/8/8/8/8 w - - 0 1");
    
    game.board.pretty_print();

    let from = Square::from_position(&Position::from_notation(FileChars::E, 5));
    let to = Square::from_position(&Position::from_notation(FileChars::F, 6));

    let mv = Move::builder().move_from(from).move_to(to).build();
    let result = game.board.move_pawn(mv);

    game.board.pretty_print();

    assert!(result.is_some(), "White pawn should capture right (e5 to f6)");
}

#[test]
fn test_white_pawn_capture_left() {
    // White pawn on e4, black pawn on d5
    let mut game = GameState::from_fen("8/8/8/3p4/4P3/8/8/8 w - - 0 1");

    let from = Square::from_position(&Position::from_notation(FileChars::E, 4)); // e4
    let to = Square::from_position(&Position::from_notation(FileChars::D, 5));   // d5

    let mv = Move::builder().move_from(from).move_to(to).build();
    let result = game.board.move_pawn(mv);

    assert!(result.is_some(), "White pawn should capture left (e4 to d5)");
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

    assert!(result.is_some(), "Black pawn should capture right (e5 to d4)");
}

#[test]
fn test_black_pawn_capture_left() {
    // Black pawn on e5, white pawn on f4
    let mut game = GameState::from_fen("8/8/8/4p3/5P2/8/8/8 b - - 0 1");

    let from = Square::from_position(&Position::from_notation(FileChars::E, 5));
    let to = Square::from_position(&Position::from_notation(FileChars::F, 4));

    let mv = Move::builder().move_from(from).move_to(to).build();
    let result = game.board.move_pawn(mv);

    assert!(result.is_some(), "Black pawn should capture left (e5 to f4)");
}

#[test]
fn test_pawn_illegal_diagonal_without_target() {
    // White pawn on e4, nothing on d5
    let mut game = GameState::from_fen("8/8/8/3P4/8/8/8/8 w - - 0 1");

    let from = Square::from_position(&Position::new(File(3), Rank(4))); // d4
    let to = Square::from_position(&Position::new(File(2), Rank(5)));   // c5

    let mv = Move::builder().move_from(from).move_to(to).build();
    let result = game.board.move_pawn(mv);

    assert!(result.is_none(), "Pawn should not be able to capture if no piece is there");
}

#[test]
fn test_pawn_cannot_capture_same_color() {
    // White pawn on e4, white pawn on f5
    let mut game = GameState::from_fen("8/8/5P2/4P3/8/8/8/8 w - - 0 1");

    let from = Square::from_position(&Position::new(File(4), Rank(4))); // e4
    let to = Square::from_position(&Position::new(File(5), Rank(5)));   // f5

    let mv = Move::builder().move_from(from).move_to(to).build();
    let result = game.board.move_pawn(mv);

    assert!(result.is_none(), "Pawn should not be able to capture its own piece");
}

#[test]
fn test_white_pawn_cannot_capture_left_on_file_a() {
    // White pawn on a4, nothing to capture
    let mut game = GameState::from_fen("8/8/8/8/P7/8/8/8 w - - 0 1");

    let from = Square::from_position(&Position::from_notation(FileChars::A, 4));
    let to = Square::from_position(&Position::from_notation(FileChars::H, 5)); // totally wrong direction

    let mv = Move::builder().move_from(from).move_to(to).build();
    let result = game.board.move_pawn(mv);

    assert!(result.is_none(), "White pawn on a4 should not be able to capture off the left edge");
}

#[test]
fn test_white_pawn_capture_from_a4_to_b5() {
    // White pawn on a4, black pawn on b5
    let mut game = GameState::from_fen("8/8/8/1p6/P7/8/8/8 w - - 0 1");

    let from = Square::from_position(&Position::from_notation(FileChars::A, 4)); // a4
    let to   = Square::from_position(&Position::from_notation(FileChars::B, 5)); // b5

    let mv = Move::builder().move_from(from).move_to(to).build();
    let result = game.board.move_pawn(mv);

    assert!(result.is_some(), "White pawn should be able to capture from a4 to b5");
}

#[test]
fn test_white_pawn_capture_from_h4_to_g5() {
    // White pawn on h4, black pawn on g5
    let mut game = GameState::from_fen("8/8/8/6p1/7P/8/8/8 w - - 0 1");
    game.board.pretty_print();

    let from = Square::from_position(&Position::from_notation(FileChars::H, 4)); // h4
    let to   = Square::from_position(&Position::from_notation(FileChars::G, 5)); // g5

    let mv = Move::builder().move_from(from).move_to(to).build();
    let result = game.board.move_pawn(mv);

    game.board.pretty_print();

    assert!(result.is_some(), "White pawn should be able to capture from h4 to g5");
}
