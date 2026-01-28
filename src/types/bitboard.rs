use crate::types::Square;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
#[repr(transparent)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const MASK: Self = Self(0xFFFFFFFFF);

    pub fn set(&mut self, sq: Square) {
        *self |= sq.to_bitboard();
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
