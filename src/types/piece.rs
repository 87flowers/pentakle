use crate::types::Color;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Default)]
pub enum PieceType {
    Flat,
    Wall,
    Cap,
    #[default]
    None,
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Default)]
pub enum Piece {
    P1Flat,
    P2Flat,
    P1Wall,
    P2Wall,
    P1Cap,
    P2Cap,
    #[default]
    None,
}

impl PieceType {
    pub const NUM: usize = 3;

    #[must_use]
    pub const fn from_index(index: usize) -> PieceType {
        debug_assert!(index < Self::NUM);
        unsafe { std::mem::transmute(index as u8) }
    }

    #[must_use]
    pub fn is_some(&self) -> bool {
        *self != PieceType::None
    }

    #[must_use]
    pub const fn to_index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub fn some_or(self, or: PieceType) -> PieceType {
        if self.is_some() { self } else { or }
    }
}

impl Piece {
    pub const NUM: usize = 6;

    #[must_use]
    pub fn new(c: Color, pt: PieceType) -> Piece {
        if pt == PieceType::None {
            Piece::None
        } else {
            Piece::from_index(c.to_index() | (pt.to_index() << 1))
        }
    }

    #[must_use]
    pub const fn from_index(index: usize) -> Piece {
        debug_assert!(index < Self::NUM);
        unsafe { std::mem::transmute(index as u8) }
    }

    #[must_use]
    pub const fn to_index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub fn is_some(&self) -> bool {
        *self != Piece::None
    }

    #[must_use]
    pub fn is_none(&self) -> bool {
        *self == Piece::None
    }

    #[must_use]
    pub fn piece_type(&self) -> PieceType {
        PieceType::from_index(self.to_index() >> 1)
    }
}
