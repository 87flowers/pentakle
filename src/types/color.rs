#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Color {
    P1,
    P2,
}

impl Color {
    pub const NUM: usize = 2;

    #[must_use]
    pub const fn from_index(value: u8) -> Color {
        debug_assert!(value < Self::NUM as u8);

        unsafe { std::mem::transmute(value) }
    }

    #[must_use]
    pub const fn to_index(self) -> usize {
        self as usize
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::P1 => '1',
                Color::P2 => '2',
            }
        )
    }
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        Color::from_index((self.to_index() ^ 1) as u8)
    }
}
