use crate::types::Square;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
#[repr(transparent)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const MASK: Self = Self(0xFFFFFFFFF);

    pub fn rank_mask(rank: usize) -> Bitboard {
        Bitboard(0b000000_000000_000000_000000_000000_111111 << (rank * 6))
    }

    pub fn file_mask(file: usize) -> Bitboard {
        Bitboard(0b000001_000001_000001_000001_000001_000001 << file)
    }

    pub fn set(&mut self, sq: Square) {
        *self |= sq.to_bitboard();
    }

    pub fn unset(&mut self, sq: Square) {
        *self &= !sq.to_bitboard();
    }

    #[must_use]
    pub fn get(self, sq: Square) -> bool {
        (self & sq.to_bitboard()).is_some()
    }

    #[must_use]
    pub fn is_some(self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub fn lsb(self) -> Square {
        if self.is_empty() {
            Square::None
        } else {
            Square::new(self.0.trailing_zeros() as u8)
        }
    }

    #[must_use]
    pub fn msb(self) -> Square {
        if self.is_empty() {
            Square::None
        } else {
            Square::new((self.0.bit_width() - 1) as u8)
        }
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            let sq = self.lsb();
            self.0 &= self.0 - 1;
            Some(sq)
        }
    }
}

impl std::ops::Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0) & Bitboard::MASK
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl std::ops::Add for Bitboard {
    type Output = Bitboard;

    fn add(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 + rhs.0) & Bitboard::MASK
    }
}

impl std::ops::Sub for Bitboard {
    type Output = Bitboard;

    fn sub(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 - rhs.0) & Bitboard::MASK
    }
}
