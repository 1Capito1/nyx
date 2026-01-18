use crate::bit_board::BitBoard;

use super::Board;

impl Board {
    #[allow(clippy::cast_possible_truncation)]
    pub fn pretty_print(&self) -> [char; 64] {
        let mut board_rep = ['.'; 64];
        for (i, cell) in board_rep.iter_mut().enumerate() {
            let mut set_count = 0;
            if self.white_pawn.is_set(i as u8) {
                *cell = 'P';
                set_count += 1;
            }
            if self.white_bishop.is_set(i as u8) {
                *cell = 'B';
                set_count += 1;
            }
            if self.white_rook.is_set(i as u8) {
                *cell = 'R';
                set_count += 1;
            }
            if self.white_queen.is_set(i as u8) {
                *cell = 'Q';
                set_count += 1;
            }
            if self.white_king.is_set(i as u8) {
                *cell = 'K';
                set_count += 1;
            }
            if self.white_knight.is_set(i as u8) {
                *cell = 'N';
                set_count += 1;
            }
            if self.black_pawn.is_set(i as u8) {
                *cell = 'p';
                set_count += 1;
            }
            if self.black_bishop.is_set(i as u8) {
                *cell = 'b';
                set_count += 1;
            }
            if self.black_knight.is_set(i as u8) {
                *cell = 'n';
                set_count += 1;
            }
            if self.black_rook.is_set(i as u8) {
                *cell = 'r';
                set_count += 1;
            }
            if self.black_king.is_set(i as u8) {
                *cell = 'k';
                set_count += 1;
            }
            if self.black_queen.is_set(i as u8) {
                *cell = 'q';
                set_count += 1;
            }
            if set_count > 1 {
                panic!("Multiple Pieces on one square");
            }
        }
        println!();
        BitBoard::print_bitboard_rep(&board_rep);
        board_rep
    }
}
