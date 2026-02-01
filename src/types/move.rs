use crate::types::{PieceType, Square, SquareParseError};
use std::num::NonZeroU16;
use std::str::FromStr;
use thiserror::Error;

#[derive(Copy, Clone)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub const NUM: usize = 4;

    #[must_use]
    pub const fn from_index(index: usize) -> Dir {
        debug_assert!(index < Self::NUM);
        unsafe { std::mem::transmute(index as u8) }
    }
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::North => '+',
                Dir::South => '-',
                Dir::East => '>',
                Dir::West => '<',
            }
        )
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Move(NonZeroU16);

impl Move {
    const SPLAT_SHIFT: usize = 6;
    const FLAGS_SHIFT: usize = 12;

    #[must_use]
    pub fn place(pt: PieceType, sq: Square) -> Move {
        let pt = (pt as u16) + 1;
        let sq = sq.to_index() as u16;
        let raw = (pt << Self::FLAGS_SHIFT) | sq;
        Move(NonZeroU16::new(raw).unwrap())
    }

    #[must_use]
    pub fn spread(sq: Square, dir: Dir, splat: u8) -> Move {
        debug_assert!(splat != 0);
        debug_assert!((splat & 0x3F) == splat);
        let dir = dir as u16;
        let splat = splat as u16;
        let sq = sq.to_index() as u16;
        let raw = (dir << Self::FLAGS_SHIFT) | (splat << Self::SPLAT_SHIFT) | sq;
        Move(NonZeroU16::new(raw).unwrap())
    }

    #[must_use]
    pub fn raw(self) -> u16 {
        self.0.get()
    }

    #[must_use]
    pub fn splat(self) -> u8 {
        (self.raw() >> Self::SPLAT_SHIFT & 0x3F) as u8
    }

    #[must_use]
    pub fn flags(self) -> u16 {
        self.raw() >> Self::FLAGS_SHIFT
    }

    #[must_use]
    pub fn is_place(self) -> bool {
        self.splat() == 0
    }

    #[must_use]
    pub fn is_spread(self) -> bool {
        self.splat() != 0
    }

    #[must_use]
    pub fn sq(self) -> Square {
        Square::new((self.raw() & 0x3F) as u8)
    }

    #[must_use]
    pub fn piece_type(self) -> PieceType {
        debug_assert!(self.is_place());
        PieceType::from_index((self.flags() - 1) as usize)
    }

    #[must_use]
    pub fn dir(self) -> Dir {
        debug_assert!(self.is_spread());
        Dir::from_index(self.flags() as usize)
    }
}

#[derive(Error, Debug)]
pub enum MoveParseError {
    #[error("move string too short")]
    TooShort,
    #[error("invalid lift count at start of move")]
    InvalidLiftCount,
    #[error("non-existing or invalid direction in move string")]
    InvalidDirection,
    #[error("invalid drop counts in move string")]
    InvalidSplat,
    #[error("invalid or extra trailing characters at end of move string")]
    InvalidTrailingCharacter,
    #[error(transparent)]
    InvalidSquare(#[from] SquareParseError),
}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Move, MoveParseError> {
        let mut it = s.chars().peekable();

        let (ptype, count) = match it.peek() {
            Some('F') => {
                it.next();
                (PieceType::Flat, None)
            }
            Some('S') => {
                it.next();
                (PieceType::Wall, None)
            }
            Some('C') => {
                it.next();
                (PieceType::Cap, None)
            }
            Some(&c) if c.is_ascii_digit() => {
                it.next();
                if ('1'..='6').contains(&c) {
                    (PieceType::None, Some(c as usize - '0' as usize))
                } else {
                    return Err(MoveParseError::InvalidLiftCount);
                }
            }
            Some(_) => (PieceType::None, None),
            None => return Err(MoveParseError::TooShort),
        };

        let Ok(sq) = it.next_chunk::<2>() else {
            return Err(MoveParseError::TooShort);
        };
        let sq = sq.iter().collect::<String>().parse()?;

        if it.peek().is_none() && count.is_none() {
            return Ok(Move::place(ptype.some_or(PieceType::Flat), sq));
        }

        let dir = match it.next() {
            Some('+') => Dir::North,
            Some('-') => Dir::South,
            Some('<') => Dir::West,
            Some('>') => Dir::East,
            _ => return Err(MoveParseError::InvalidDirection),
        };

        let count = count.unwrap_or(1);
        let mut splat: u8 = 0;
        let mut dropped: usize = 0;

        if it.peek().is_none() {
            let splat = 1 << (count - 1);
            return Ok(Move::spread(sq, dir, splat));
        }

        for ch in it {
            if !('1'..='6').contains(&ch) {
                return Err(MoveParseError::InvalidTrailingCharacter);
            }

            let cur = ch as usize - '0' as usize;
            dropped += cur;
            if dropped > count {
                return Err(MoveParseError::InvalidSplat);
            }

            splat |= 1u8 << (dropped - 1);
        }

        if dropped != count {
            return Err(MoveParseError::InvalidSplat);
        }

        Ok(Move::spread(sq, dir, splat))
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_place() {
            match self.piece_type() {
                PieceType::Flat => write!(f, "{}", self.sq()),
                PieceType::Wall => write!(f, "S{}", self.sq()),
                PieceType::Cap => write!(f, "C{}", self.sq()),
                PieceType::None => unreachable!(),
            }
        } else {
            let mut splat = self.splat();
            let size = splat.bit_width();

            if size == 1 {
                write!(f, "{}{}", self.sq(), self.dir())
            } else if splat.count_ones() == 1 {
                write!(f, "{}{}{}", size, self.sq(), self.dir())
            } else {
                let mut counts = Vec::new();
                while splat != 0 {
                    let drop_size = splat.trailing_zeros() + 1;
                    counts.push(drop_size);
                    splat >>= drop_size;
                }
                let counts = counts.iter().map(u32::to_string).collect::<String>();
                write!(f, "{}{}{}{}", size, self.sq(), self.dir(), counts)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_moves() {
        let cases = ["a1", "Cb4", "Sd3", "a1>", "d1-", "4c3>", "3b2+111", "5e4<23", "5b4>212"];

        for case in cases {
            let mv = Move::from_str(case).unwrap();
            let mstr = mv.to_string();
            assert_eq!(case, mstr);
        }
    }
}
