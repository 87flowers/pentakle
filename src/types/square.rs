use crate::types::Bitboard;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
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
    pub const fn to_index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn to_bitboard(self) -> Bitboard {
        Bitboard(1u64 << self as u64)
    }
}
