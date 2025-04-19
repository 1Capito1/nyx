use crate::bit_board::{BitBoard, File, Position, Rank, CheckedSub};
use crate::board::Board;
use crate::piece::FenNotation;

const VALID_FEN_PEICE_NOTATIONS: [char; 12] =
['p', 'n', 'b', 'r', 'q', 'k', 'P', 'N', 'B', 'R', 'Q', 'K'];

enum Player {
    White,
    Black,
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
            "w" => Player::White,
            "b" => Player::Black,
            _ => panic!("Invalid active color"),
        };
        board.update_cache();
        // TODO: rest of FEN
        Self { board, player }
    }
}
