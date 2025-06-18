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
    Human
}

pub struct GameState {
    pub board: Board,
    player: Player,
}

impl GameState {
    pub fn from_fen(string: &str) -> Self {
        let mut sections = string.split_whitespace();
        let board_str = sections.next().expect("Missing FEN board");
        let active_color = sections.next().expect("Missing Active Color");
        let castling_rights = sections.next().expect("Missing Castling Rights");
        let en_passant_square = sections.next().expect("Missing En Passant Square");
        let halfmove_clock = sections.next().expect("Missing Halfmove Clock");
        let fullmove_clock = sections.next().expect("Missing Fullmove Clock");

        let mut board = Board::default();
        let mut rank: Rank = Rank(7);
        let mut file: File = File(0);

        for c in board_str.chars() {
            match c {
                '/' => {
                    if file != File(8) {
                        panic!("Too few squares in rank {}", rank);
                    }
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
        if rank != Rank(0) || file != File(8) {
            panic!("FEN parsing error: final rank is invalid");
        }
        // next should be w or b for current player turn
        let player = match active_color {
            "w" => Player::White(Agent::Human),
            "b" => Player::Black(Agent::Ai),
            _ => panic!("Invalid active color"),
        };

        if castling_rights != "-" {
            castling_rights
                .chars()
                .for_each(|c| *board.castling_rights.get_from_fen(c) = true);
        }

        let passant_square = if en_passant_square != "-" {
            let bytes = en_passant_square.as_bytes();
            assert_eq!(bytes.len(), 2, "Invalid en passant square length");
            let file = File(bytes[0] - b'a');
            let rank = Rank(bytes[1] - b'1');
            let pos = Position::new(file, rank);

            Some(pos.to_square())
        } else { None };
        board.en_passant_square = passant_square;
        board.update_cache();
        // TODO: rest of FEN
        Self { board, player }
    }
}
