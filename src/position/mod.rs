use crate::types::{Bitboard, Color, Piece, PieceType, Square};

mod make_move;
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

impl Default for Position {
    fn default() -> Position {
        Position {
            stm: Color::P1,
            ply: 0,
            colors: Default::default(),
            tops: Default::default(),
            mailbox: [Piece::None; Square::NUM],
            stacks: [0; Square::NUM],
            heights: [0; Square::NUM],
            remaining_stones: [Position::STARTING_STONES; Color::NUM],
            remaining_caps: [Position::STARTING_CAPS; Color::NUM],
        }
    }
}

impl Position {
    pub const STARTING_STONES: u8 = 30;
    pub const STARTING_CAPS: u8 = 1;

    #[must_use]
    pub fn is_empty(&self, sq: Square) -> bool {
        self.mailbox[sq.to_index()].is_none()
    }
}
