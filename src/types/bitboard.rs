use crate::types::Square;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
#[repr(transparent)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const MASK: Self = Self(0xFFFFFFFFF);

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
}

impl std::ops::Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
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
