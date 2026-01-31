use super::Position;
use crate::types::{Color, Move, Piece, PieceType};

fn ones(count: u32) -> u64 {
    debug_assert!(count < 64);
    (1 << count) - 1
}

impl Position {
    #[must_use]
    pub fn make_move(&self, mv: Move) -> Position {
        let mut pos = self.clone();

        let stm = if pos.ply < 2 { !pos.stm } else { pos.stm };

        let sq = mv.sq().to_index();
        let bit = mv.sq().to_bitboard();

        if mv.is_place() {
            let pt = mv.piece_type();

            debug_assert!(pos.heights[sq] == 0);
            debug_assert!(pos.mailbox[sq].is_none());

            pos.heights[sq] = 1;
            pos.stacks[sq] = stm.to_index() as u64;
            pos.mailbox[sq] = Piece::new(stm, pt);

            pos.colors[stm.to_index()] |= bit;
            pos.tops[pt.to_index()] |= bit;

            let remaining = match mv.piece_type() {
                PieceType::Flat | PieceType::Wall => &mut pos.remaining_stones[pos.stm.to_index()],
                PieceType::Cap => &mut pos.remaining_caps[pos.stm.to_index()],
                _ => unreachable!("Invalid piece to place"),
            };
            *remaining -= 1;
        } else {
            debug_assert!(mv.splat() != 0);

            let mut splat = mv.splat();
            let mut hand_height = splat.bit_width();
            let mut hand_colors = pos.stacks[sq] & ones(hand_height);
            let top = pos.mailbox[sq];

            pos.stacks[sq] >>= hand_height;
            pos.heights[sq] -= hand_height as u8;
            pos.mailbox[sq] = if pos.heights[sq] == 0 {
                Piece::None
            } else {
                Piece::new(
                    Color::from_index((pos.stacks[sq] & 1) as u8),
                    PieceType::Flat,
                )
            };

            let dir = mv.dir();
            let mut curr = mv.sq();

            while splat != 0 {
                curr = curr.step(dir);

                let drop_size = splat.trailing_zeros() + 1;
                let drop_colors = hand_colors >> (hand_height - drop_size);
                let drop_top_color = Color::from_index((drop_colors & 1) as u8);

                pos.heights[curr.to_index()] += drop_size as u8;
                pos.stacks[curr.to_index()] <<= drop_size;
                pos.stacks[curr.to_index()] |= drop_colors;
                pos.mailbox[curr.to_index()] = Piece::new(drop_top_color, PieceType::Flat);
                pos.colors[drop_top_color.to_index()] |= curr.to_bitboard();
                pos.colors[!drop_top_color.to_index()] &= !curr.to_bitboard();
                pos.tops[PieceType::Flat.to_index()] |= curr.to_bitboard();

                splat >>= drop_size;
                hand_height -= drop_size;
                hand_colors >>= drop_size;
            }

            if top.piece_type() != PieceType::Flat {
                pos.tops[PieceType::Flat.to_index()] &= !curr.to_bitboard();
                pos.tops[PieceType::Wall.to_index()] &= !curr.to_bitboard();
                pos.tops[PieceType::Cap.to_index()] &= !curr.to_bitboard();

                pos.tops[top.piece_type().to_index()] |= curr.to_bitboard();

                pos.mailbox[curr.to_index()] = top;
            }
        }

        pos.stm = !pos.stm;
        pos.ply += 1;

        pos
    }
}
