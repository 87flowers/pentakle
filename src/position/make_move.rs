use super::Position;
use crate::types::{Color, Move, Piece, PieceType, Square};

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
                PieceType::Flat | PieceType::Wall => &mut pos.remaining_stones[stm.to_index()],
                PieceType::Cap => &mut pos.remaining_caps[stm.to_index()],
                _ => unreachable!("Invalid piece to place"),
            };
            *remaining -= 1;
        } else {
            debug_assert!(mv.splat() != 0);

            let mut splat = mv.splat();
            let mut hand_height = splat.bit_width();
            let hand_colors = pos.stacks[sq] & ones(hand_height);
            let top = pos.mailbox[sq];

            pos.stacks[sq] >>= hand_height;
            pos.heights[sq] -= hand_height as u8;
            pos.mailbox[sq] = if pos.heights[sq] == 0 {
                pos.colors[top.color().to_index()] &= !bit;
                pos.tops[top.piece_type().to_index()] &= !bit;

                Piece::None
            } else {
                pos.colors[top.color().to_index()] &= !bit;
                pos.tops[top.piece_type().to_index()] &= !bit;

                let color = Color::from_index((pos.stacks[sq] & 1) as u8);
                pos.colors[color.to_index()] |= bit;
                pos.tops[PieceType::Flat.to_index()] |= bit;

                Piece::new(color, PieceType::Flat)
            };

            let dir = mv.dir();
            let mut curr = mv.sq();

            while splat != 0 {
                curr = curr.step(dir);

                let drop_size = splat.trailing_zeros() + 1;
                let drop_colors = (hand_colors >> (hand_height - drop_size)) & ones(drop_size);
                let drop_top_color = Color::from_index((drop_colors & 1) as u8);

                pos.heights[curr.to_index()] += drop_size as u8;
                pos.stacks[curr.to_index()] <<= drop_size;
                pos.stacks[curr.to_index()] |= drop_colors;
                pos.mailbox[curr.to_index()] = Piece::new(drop_top_color, PieceType::Flat);
                pos.colors[drop_top_color.to_index()] |= curr.to_bitboard();
                pos.colors[(!drop_top_color).to_index()] &= !curr.to_bitboard();
                pos.tops[PieceType::Flat.to_index()] |= curr.to_bitboard();

                splat >>= drop_size;
                hand_height -= drop_size;
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

    pub fn verify(&self) {
        for i in 0..Square::NUM {
            let sq = Square::new(i as u8);
            let height = self.heights[i];
            if height == 0 {
                assert_eq!(self.stacks[i], 0);
                assert_eq!(self.mailbox[i], Piece::None);
                for c in self.colors {
                    assert!(!c.get(sq));
                }
                for t in self.tops {
                    assert!(!t.get(sq));
                }
            } else {
                assert_eq!(self.stacks[i], self.stacks[i] & ones(height as u32));
                let top_color = Color::from_index((self.stacks[i] & 1) as u8);
                let top = self.mailbox[i];
                assert_eq!(top.color(), top_color);
                assert!(self.colors[top_color.to_index()].get(sq));
                assert!(!self.colors[(!top_color).to_index()].get(sq));
                match top.piece_type() {
                    PieceType::None => assert!(false),
                    PieceType::Flat => {
                        assert!(self.tops[PieceType::Flat.to_index()].get(sq));
                        assert!(!self.tops[PieceType::Wall.to_index()].get(sq));
                        assert!(!self.tops[PieceType::Cap.to_index()].get(sq));
                    }
                    PieceType::Wall => {
                        assert!(self.tops[PieceType::Wall.to_index()].get(sq));
                        assert!(!self.tops[PieceType::Flat.to_index()].get(sq));
                        assert!(!self.tops[PieceType::Cap.to_index()].get(sq));
                    }
                    PieceType::Cap => {
                        assert!(self.tops[PieceType::Cap.to_index()].get(sq));
                        assert!(!self.tops[PieceType::Wall.to_index()].get(sq));
                        assert!(!self.tops[PieceType::Flat.to_index()].get(sq));
                    }
                }
            }
        }

        let board_count: u8 = self.heights.iter().sum();
        let p2_count: u8 = self.stacks.iter().map(|x| x.count_ones() as u8).sum();
        let p1_count = board_count - p2_count;

        assert_eq!(
            p1_count + self.remaining_stones[0] + self.remaining_caps[0],
            Position::STARTING_STONES + Position::STARTING_CAPS
        );
        assert_eq!(
            p2_count + self.remaining_stones[1] + self.remaining_caps[1],
            Position::STARTING_STONES + Position::STARTING_CAPS
        );
        assert_eq!(
            self.caps(Color::P1).count_ones() as u8 + self.remaining_caps[0],
            Position::STARTING_CAPS
        );
        assert_eq!(
            self.caps(Color::P2).count_ones() as u8 + self.remaining_caps[1],
            Position::STARTING_CAPS
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn make_move() {
        let position = Position::from_str(
            "x6/2C,1,1,1,1,1/2,x,111121S,x3/2,x,11,x,1,x/2,1C,12,2,2,2/x,112,x4 2 22",
        )
        .unwrap();

        let mv = Move::from_str("3b1>12").unwrap();
        let expected = "x6/2C,1,1,1,1,1/2,x,111121S,x3/2,x,11,x,1,x/2,1C,12,2,2,2/x2,1,12,x2 1 23";
        let position = position.make_move(mv);
        assert_eq!(expected, position.to_string());
        position.verify();

        let mv = Move::from_str("6c4-213").unwrap();
        let expected = "x6/2C,1,1,1,1,1/2,x5/2,x,1111,x,1,x/2,1C,121,2,2,2/x2,1121S,12,x2 2 23";
        let position = position.make_move(mv);
        assert_eq!(expected, position.to_string());
        position.verify();

        let mv = Move::from_str("e4").unwrap();
        let expected = "x6/2C,1,1,1,1,1/2,x3,2,x/2,x,1111,x,1,x/2,1C,121,2,2,2/x2,1121S,12,x2 1 24";
        let position = position.make_move(mv);
        assert_eq!(expected, position.to_string());
        position.verify();
    }
}
