use crate::types::{Bitboard, Dir};
use std::str::FromStr;
use thiserror::Error;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1,
    A2, B2, C2, D2, E2, F2,
    A3, B3, C3, D3, E3, F3,
    A4, B4, C4, D4, E4, F4,
    A5, B5, C5, D5, E5, F5,
    A6, B6, C6, D6, E6, F6,
    #[default]
    None,
}

impl Square {
    pub const NUM: usize = 36;

    #[must_use]
    pub const fn new(value: u8) -> Square {
        debug_assert!(value < Self::NUM as u8);

        unsafe { std::mem::transmute(value) }
    }

    #[must_use]
    pub const fn from_file_and_rank(file: usize, rank: usize) -> Square {
        debug_assert!(file < 6);
        debug_assert!(rank < 6);

        let sq = file + rank * 6;
        Square::new(sq as u8)
    }

    #[must_use]
    pub const fn to_file_and_rank(self) -> (usize, usize) {
        (self.file(), self.rank())
    }

    #[must_use]
    pub const fn file(self) -> usize {
        self.to_index() % 6
    }

    #[must_use]
    pub const fn rank(self) -> usize {
        self.to_index() / 6
    }

    #[must_use]
    pub fn is_none(self) -> bool {
        self == Square::None
    }

    #[must_use]
    pub fn is_some(self) -> bool {
        self != Square::None
    }

    #[must_use]
    pub const fn to_index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn to_bitboard(self) -> Bitboard {
        Bitboard(1u64 << self as u64)
    }

    #[must_use]
    pub fn step(self, dir: Dir) -> Square {
        let (file, rank) = self.to_file_and_rank();
        match dir {
            Dir::North => Square::from_file_and_rank(file, rank + 1),
            Dir::South => Square::from_file_and_rank(file, rank - 1),
            Dir::East => Square::from_file_and_rank(file + 1, rank),
            Dir::West => Square::from_file_and_rank(file - 1, rank),
        }
    }
}

#[derive(Error, Debug)]
pub enum SquareParseError {
    #[error("invalid length of square specifier")]
    InvalidLength,
    #[error("invalid file in square specifier")]
    InvalidFile,
    #[error("invalid rank in square specifier")]
    InvalidRank,
}

impl FromStr for Square {
    type Err = SquareParseError;

    fn from_str(s: &str) -> Result<Square, SquareParseError> {
        let [file, rank] = s.as_bytes() else {
            return Err(SquareParseError::InvalidLength);
        };

        let file = if (b'a'..=b'f').contains(file) {
            file - b'a'
        } else {
            return Err(SquareParseError::InvalidFile);
        };
        let rank = if (b'1'..=b'6').contains(rank) {
            rank - b'1'
        } else {
            return Err(SquareParseError::InvalidRank);
        };

        Ok(Square::from_file_and_rank(file as usize, rank as usize))
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (file, rank) = self.to_file_and_rank();
        write!(
            f,
            "{}{}",
            (b'a' + file as u8) as char,
            (b'1' + rank as u8) as char
        )
    }
}
