use crate::types::{Bitboard, Color, Piece, PieceType, Square};

mod make_move;
mod movegen;
mod road;
mod tps;

pub use movegen::MoveList;

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

    #[must_use]
    pub fn occupied(&self) -> Bitboard {
        self.colors[0] | self.colors[1]
    }

    #[must_use]
    pub fn piece_on(&self, sq: Square) -> Piece {
        self.mailbox[sq.to_index()]
    }

    #[must_use]
    pub fn height(&self, sq: Square) -> u8 {
        self.heights[sq.to_index()]
    }

    #[must_use]
    pub fn color(&self, c: Color) -> Bitboard {
        self.colors[c.to_index()]
    }

    #[must_use]
    pub fn all_flats(&self) -> Bitboard {
        self.tops[PieceType::Flat.to_index()]
    }

    #[must_use]
    pub fn all_walls(&self) -> Bitboard {
        self.tops[PieceType::Wall.to_index()]
    }

    #[must_use]
    pub fn all_caps(&self) -> Bitboard {
        self.tops[PieceType::Cap.to_index()]
    }

    #[must_use]
    pub fn all_royals(&self) -> Bitboard {
        self.all_walls() | self.all_caps()
    }

    #[must_use]
    pub fn all_roads(&self) -> Bitboard {
        self.all_flats() | self.all_caps()
    }

    #[must_use]
    pub fn flats(&self, c: Color) -> Bitboard {
        self.all_flats() & self.color(c)
    }

    #[must_use]
    pub fn caps(&self, c: Color) -> Bitboard {
        self.all_caps() & self.color(c)
    }

    #[must_use]
    pub fn roads(&self, c: Color) -> Bitboard {
        self.all_roads() & self.color(c)
    }
}
