use super::Position;
use crate::{
    KOMI,
    types::{Bitboard, Color},
};
use std::{cmp::Ordering, simd::u64x4};

#[derive(Copy, Clone, Debug)]
pub enum Terminal {
    P1Won,
    P2Won,
    Draw,
}

impl Terminal {
    fn winner(self) -> Option<Color> {
        match self {
            Terminal::P1Won => Some(Color::P1),
            Terminal::P2Won => Some(Color::P2),
            Terminal::Draw => None,
        }
    }
}

impl Position {
    pub fn road_completed(&self, c: Color) -> bool {
        let bb = self.roads(c);

        const NORTH_EDGE: Bitboard = Bitboard::rank_mask(5);
        const EAST_EDGE: Bitboard = Bitboard::file_mask(5);
        const SOUTH_EDGE: Bitboard = Bitboard::rank_mask(0);
        const WEST_EDGE: Bitboard = Bitboard::file_mask(0);
        const EDGES: u64x4 =
            u64x4::from_array([NORTH_EDGE.0, EAST_EDGE.0, SOUTH_EDGE.0, WEST_EDGE.0]);

        let bb = u64x4::splat(bb.0);
        let mut curr = bb & EDGES;

        loop {
            let next = curr
                | ((curr & u64x4::splat((!EAST_EDGE).0)) << 1)
                | ((curr & u64x4::splat((!WEST_EDGE).0)) >> 1)
                | (curr << 6)
                | (curr >> 6);
            let next = next & bb;

            if next == curr {
                return false;
            }

            if (next.rotate_elements_right::<2>() & next) != u64x4::splat(0) {
                return true;
            }

            curr = next;
        }
    }

    pub fn terminal(&self) -> Option<Terminal> {
        if self.road_completed(Color::P1) {
            Some(Terminal::P1Won)
        } else if self.road_completed(Color::P2) {
            Some(Terminal::P2Won)
        } else if self
            .remaining_stones
            .iter()
            .zip(self.remaining_caps.iter())
            .any(|x| x == (&0, &0))
        {
            let p1flats = self.flats(Color::P1).count_ones();
            let p2flats = self.flats(Color::P2).count_ones();
            match p1flats.cmp(&(p2flats + KOMI)) {
                Ordering::Less => Some(Terminal::P2Won),
                Ordering::Greater => Some(Terminal::P1Won),
                Ordering::Equal => Some(Terminal::Draw),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test1() {
        let tps = "2,x2,2,x2/x3,2,2,x/x,2,221,2,x,1/21,1,1,1C,121,1/x2,112C,x3/x5,1 2 16";
        let pos = Position::from_str(tps).unwrap();
        assert!(pos.road_completed(Color::P1));
        assert!(!pos.road_completed(Color::P2));
    }

    #[test]
    fn test2() {
        let tps = "2,x2,2,x2/x3,2,2,x/x,2,221,2,x,1/2,1,1,1C,121,1/1,x,112C,x3/x5,1 1 16";
        let pos = Position::from_str(tps).unwrap();
        assert!(!pos.road_completed(Color::P1));
        assert!(!pos.road_completed(Color::P2));
    }

    #[test]
    fn test3() {
        let tps = "2,x2,2,x2/x3,2,2,x/x,2,221,2,2,1/2,1,1,1C,1,1/1,x,1,12C,1,x/x5,1 2 14";
        let pos = Position::from_str(tps).unwrap();
        assert!(!pos.road_completed(Color::P1));
        assert!(!pos.road_completed(Color::P2));
    }

    #[test]
    fn will_tests() {
        let cases = [
            (
                Color::P2,
                "2S,211211C,x,1,2,12S/x2,1S,2S,2C,21S/x,122S,1S,x,12,122/x2,11,1S,22S,2/x2,11S,22S,21S,11112/1221222S,1,x,22122,11S,2 2 32",
            ),
            (
                Color::P1,
                "1,1,221,112,1S,x/1222,2C,11,1,1S,1S/1C,22,1,11,2,2/2S,1S,221,221S,2S,x/x,21S,1,21,2,x/x,1S,21221212212221,1S,1S,x 2 33",
            ),
            (
                Color::P2,
                "1211S,121S,212,x2,111S/1,211S,2C,1C,22S,212S/221,211S,2,x,2,x/22,2,2,2S,1S,1S/2,x,11,12,1,1S/112,1S,2,x,1112,22S 1 31",
            ),
            (
                Color::P2,
                "1,x3,221,111S/222S,12,2,112,212C,1/2,2,1,2S,2,21S/2S,1,2S,1,12,12122112/21S,12,2S,1,2122211C,x/x,1S,11,x,12,x 1 35",
            ),
            (
                Color::P2,
                "2,2,112,2,21,1S/x2,2,21221,2S,x/2S,1,22,1,211121C,22/122S,x,2,x,1,1S/11,2S,2,12,112,122C/1211S,11,122S,1,x,2S 1 34",
            ),
            (
                Color::P2,
                "2,2,2,2,2,x/x4,2,x/x,2,2,2,2,x/x,2,x4/x,2,2,2,2,2/x6 2 16",
            ),
            (
                Color::P1,
                "x,1,x4/x,1,x,1,1,1/x,1,x,1,x,1/x,1,x,1,x,1/x,1,1,1,x,1/x5,1 1 16",
            ),
        ];
        for (c, tps) in cases {
            let pos = Position::from_str(tps).unwrap();
            assert!(pos.road_completed(c));
            assert!(!pos.road_completed(!c));
            assert_eq!(pos.terminal().unwrap().winner(), Some(c));
        }
    }
}
