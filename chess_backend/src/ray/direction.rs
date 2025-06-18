use derive_more::AddAssign;

use crate::errors::MoveError;
use crate::Position;

#[repr(i8)]
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North = -8,
    NorthWest = -9,
    West = -1,
    SouthWest = 7,
    South = 8,
    SouthEast = 9,
    East = 1,
    NorthEast = -7,
}

impl Direction {
    pub(crate) fn get_direction<F>(
        from: &Position,
        to: &Position,
        error_predicate: F,
        error: MoveError,
    ) -> Result<Direction, MoveError>
    where
        F: Fn(&Position, &Position) -> bool,
    {
        // 1) Validate shape
        if !error_predicate(from, to) {
            return Err(error);
        }

        // 2) Compute diffs in the same sense as your Direction::delta
        let file_diff = (to.file().0 as i8) - (from.file().0 as i8);
        let rank_diff = (to.rank().0 as i8) - (from.rank().0 as i8);

        let abs_file = file_diff.abs();
        let abs_rank = rank_diff.abs();

        // 3) Diagonals
        if abs_file == abs_rank && abs_file > 0 {
            return match (file_diff.signum(), rank_diff.signum()) {
                (1, 1) => Ok(Direction::SouthEast),
                (1, -1) => Ok(Direction::NorthEast),
                (-1, 1) => Ok(Direction::SouthWest),
                (-1, -1) => Ok(Direction::NorthWest),
                _ => unreachable!(),
            };
        }

        // 4) Straight vertical
        if file_diff == 0 && abs_rank > 0 {
            return if rank_diff > 0 {
                Ok(Direction::South)
            } else {
                Ok(Direction::North)
            };
        }

        // 5) Straight horizontal
        if rank_diff == 0 && abs_file > 0 {
            return if file_diff > 0 {
                Ok(Direction::East)
            } else {
                Ok(Direction::West)
            };
        }

        // 6) Otherwise it’s not a valid ray move
        Err(error)
    }
    pub(crate) fn delta(&self) -> (i8, i8) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::NorthEast => (1, -1),
            Direction::NorthWest => (-1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
        }
    }
}
