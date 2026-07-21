use std::fmt::Debug;
use std::fmt::Display;

use super::File;

pub enum FileChars {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl Display for FileChars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FileChars::A => "A".to_string(),
            FileChars::B => "B".to_string(),
            FileChars::C => "C".to_string(),
            FileChars::D => "D".to_string(),
            FileChars::E => "E".to_string(),
            FileChars::F => "F".to_string(),
            FileChars::G => "G".to_string(),
            FileChars::H => "H".to_string(),
        };
        write!(f, "{str}")
    }
}

impl Debug for FileChars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl From<File> for FileChars {
    fn from(value: File) -> Self {
        match value {
            File(0) => Self::A,
            File(1) => Self::B,
            File(2) => Self::C,
            File(3) => Self::D,
            File(4) => Self::E,
            File(5) => Self::F,
            File(6) => Self::G,
            File(7) => Self::H,
            _ => panic!("Invalid file: {value}"),
        }
    }
}
