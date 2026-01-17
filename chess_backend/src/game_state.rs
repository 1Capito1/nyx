use crate::bit_board::{BitBoard, CheckedSub, File, Position, Rank, Square};
use crate::board::Board;
use crate::board::FenNotation;

const VALID_FEN_PEICE_NOTATIONS: [char; 12] =
    ['p', 'n', 'b', 'r', 'q', 'k', 'P', 'N', 'B', 'R', 'Q', 'K'];

enum Player {
    White(Agent),
    Black(Agent),
}

enum Agent {
    Ai,
    Human,
}

pub struct GameState {
    pub board: Board,
    player: Player,
}

pub struct FenState<'a> {
    board_str: &'a str,
    active_color: &'a str,
    castling_rights: &'a str,
    en_passant_square: &'a str,
    halfmove_clock: &'a str,
    fullmove_clock: &'a str,
}

impl<'a> FenState<'a> {
    pub fn new(string: &'a str) -> Self {
        let mut sections = string.split_whitespace();
        Self {
            board_str: sections.next().expect("Missing FEN board"),
            active_color: sections.next().expect("Missing Active Color"),
            castling_rights: sections.next().expect("Missing Castling Rights"),
            en_passant_square: sections.next().expect("Missing En Passant Square"),
            halfmove_clock: sections.next().expect("Missing Halfmove Clock"),
            fullmove_clock: sections.next().expect("Missing Fullmove Clock"),
        }
    }
}

impl GameState {
    #[allow(clippy::cast_possible_truncation)]
    fn parse_board(board: &mut Board, board_str: &str) {
        let mut rank: Rank = Rank(7);
        let mut file: File = File(0);
        for c in board_str.chars() {
            match c {
                '/' => {
                    assert!((file == File(8)), "Too few squares in rank {rank}");
                    rank = rank.checked_sub(Rank(1)).expect("Too many ranks");
                    file = File(0);
                }
                '1'..='8' => {
                    file += File(c.to_digit(10).unwrap() as u8);
                }
                _ => {
                    let pos = Position::new(file, rank);
                    let piece = FenNotation::try_from(c).expect("Invalid piece");
                    board.place_piece(piece, &pos);
                    file += File(1);
                }
            }
        }
        assert!(
            !(rank != Rank(0) || file != File(8)),
            "FEN parsing error: final rank is invalid"
        );
    }

    fn set_active_color(board: &mut Board, active_color: &str) -> Player {
        match active_color {
            "w" => Player::White(Agent::Human),
            "b" => Player::Black(Agent::Ai),
            _ => panic!("Invalid active color"),
        }
    }

    fn set_castling_rights(board: &mut Board, castling_rights: &str) {
        board.castling_rights.set_castling_rights(castling_rights);
    }

    fn set_en_passant_square(board: &mut Board, en_passant_square: &str) {
        board.en_passant_square = if en_passant_square == "-" {
            None
        } else {
            let bytes = en_passant_square.as_bytes();
            assert_eq!(bytes.len(), 2, "Invalid en passant square length");
            let file = File(bytes[0] - b'a');
            let rank = Rank(bytes[1] - b'1');
            let pos = Position::new(file, rank);

            Some(pos.to_square())
        };
    }

    pub fn from_fen(string: &str) -> Self {
        let state = FenState::new(string);
        let mut board = Board::default();

        Self::parse_board(&mut board, state.board_str);

        // next should be w or b for current player turn
        let player = Self::set_active_color(&mut board, state.active_color);

        Self::set_castling_rights(&mut board, state.castling_rights);

        Self::set_en_passant_square(&mut board, state.en_passant_square);

        board.build_cache();
        // TODO: rest of FEN
        Self { board, player }
    }
}
