use crate::types::{Bitboard, Color, Piece, PieceType, Square};

mod tps;

#[derive(Clone)]
pub struct Position {
    stm: Color,
    ply: u16,
    colors: [Bitboard; Color::NUM],
    tops: [Bitboard; PieceType::NUM],
    mailbox: [Piece; Square::NUM],
    stacks: [u64; Square::NUM],
    heights: [u8; Square::NUM],
    remaining_stones: [u8; Color::NUM],
    remaining_caps: [u8; Color::NUM],
}

impl Position {
    #[must_use]
    pub fn is_empty(&self, sq: Square) -> bool {
        self.mailbox[sq.to_index()].is_none()
    }
}
