#[cfg(test)]
mod tests {
    use crate::bit_board::Square;
    use crate::ray::{Direction, Ray};

    use super::*; // assumes Ray and Square are in the same module

    #[test]
    fn ray_north_stops_at_board_edge() {
        let from = Square(27); // d4
        let ray: Vec<_> = Ray::new(from, Direction::North).collect();

        // Should go: d5 (19), d6 (11), d7 (3)
        let expected: Vec<_> = [19, 11, 3].map(Square).to_vec();

        assert_eq!(ray, expected);
    }

    #[test]
    fn ray_south_stops_at_board_edge() {
        let from = Square(27); // d4
        let ray: Vec<_> = Ray::new(from, Direction::South).collect();

        // Should go: d3 (35), d2 (43), d1 (51), d0 (59)
        let expected: Vec<_> = [35, 43, 51, 59].map(Square).to_vec();

        assert_eq!(ray, expected);
    }

    #[test]
    fn ray_east_stops_at_file_edge() {
        let from = Square(27); // d4
        let ray: Vec<_> = Ray::new(from, Direction::East).collect();

        // Should go: e4 (28), f4 (29), g4 (30), h4 (31)
        let expected: Vec<_> = [28, 29, 30, 31].map(Square).to_vec();

        assert_eq!(ray, expected);
    }

    #[test]
    fn ray_west_stops_at_file_edge() {
        let from = Square(27); // d4
        let ray: Vec<_> = Ray::new(from, Direction::West).collect();

        // Should go: c4 (26), b4 (25), a4 (24)
        let expected: Vec<_> = [26, 25, 24].map(Square).to_vec();

        assert_eq!(ray, expected);
    }

    #[test]
    fn ray_northeast_diagonal() {
        let from = Square(27); // d4
        let ray: Vec<_> = Ray::new(from, Direction::NorthEast).collect();

        // Should go: e5 (20), f6 (13), g7 (6)
        let expected: Vec<_> = [20, 13, 6].map(Square).to_vec();

        assert_eq!(ray, expected);
    }

    #[test]
    fn ray_northwest_diagonal() {
        let from = Square(27); // d4
        let ray: Vec<_> = Ray::new(from, Direction::NorthWest).collect();

        // Should go: c5 (18), b6 (9), a7 (0)
        let expected: Vec<_> = [18, 9, 0].map(Square).to_vec();

        assert_eq!(ray, expected);
    }

    #[test]
    fn ray_southeast_diagonal() {
        let from = Square(27); // d4
        let ray: Vec<_> = Ray::new(from, Direction::SouthEast).collect();

        // Should go: e3 (36), f2 (45), g1 (54)
        let expected: Vec<_> = [36, 45, 54, 63].map(Square).to_vec();

        assert_eq!(ray, expected);
    }

    #[test]
    fn ray_southwest_diagonal() {
        let from = Square(27); // d4
        let ray: Vec<_> = Ray::new(from, Direction::SouthWest).collect();

        // Should go: c3 (34), b2 (41), a1 (48)
        let expected: Vec<_> = [34, 41, 48].map(Square).to_vec();

        assert_eq!(ray, expected);
    }
}
