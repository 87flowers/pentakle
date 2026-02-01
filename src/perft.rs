use crate::position::{MoveList, Position};
use std::time::Instant;

fn core<const PRINT: bool>(pos: &Position, depth: i32) -> u64 {
    if depth <= 0 {
        return 1;
    }

    if pos.terminal().is_some() {
        return 0;
    }

    let mut moves = MoveList::new();
    pos.generate_moves(&mut moves);

    if depth == 1 && !PRINT {
        moves.len() as u64
    } else {
        moves
            .iter()
            .map(|&mv| {
                let pos = pos.make_move(mv);

                #[cfg(test)]
                pos.verify();

                let child = core::<false>(&pos, depth - 1);

                if PRINT {
                    println!("{mv}\t: {child}");
                }

                child
            })
            .sum()
    }
}

pub fn splitperft(pos: &Position, depth: i32) {
    let start = Instant::now();
    let total = core::<true>(pos, depth);
    let mnps = (total as f64 / start.elapsed().as_secs_f64()) / 1_000_000.0;
    println!("total: {total}");
    println!("{:.1} Mnps", mnps);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn perft(pos: &Position, depth: i32) -> u64 {
        core::<false>(pos, depth)
    }

    #[test]
    fn startpos() {
        let pos = Position::default();
        let cases = [1, 36, 1260, 132720, 13586048, 1253506520];
        for (depth, &case) in cases.iter().enumerate() {
            assert_eq!(case, perft(&pos, depth as i32));
        }
    }

    #[test]
    fn complicated1() {
        let tps = "x,2,2,22S,2,111S/21S,22C,112,x,1112S,11S/x,2,112212,2,2S,2/x,2,121122,x,1112,211/21C,x,1,2S,21S,x/2S,x,212,1S,12S,1S 1 33";
        let pos = Position::from_str(tps).unwrap();
        let cases = [1, 56, 17322, 1419637, 280504959];
        for (depth, &case) in cases.iter().enumerate() {
            assert_eq!(case, perft(&pos, depth as i32));
        }
    }

    #[test]
    fn complicated2() {
        let tps = "x2,2,22,2C,1/21221S,1112,x,2211,1,2/x2,111S,x,11S,12S/11S,1S,2S,2,12S,1211C/x,12S,2,122S,x,212S/12,x2,1S,22222S,21121 2 31";
        let pos = Position::from_str(tps).unwrap();
        let cases = [1, 108, 11169, 991034, 92392763];
        for (depth, &case) in cases.iter().enumerate() {
            assert_eq!(case, perft(&pos, depth as i32));
        }
    }

    #[test]
    fn complicated3() {
        let tps = "2,x,2,111S,2,12/2,122S,2122,1S,x,1/x,111,1,11S,x2/21122112C,x,212S,2S,2,1212S/1,112S,21221S,2S,x2/21,222,x,12S,x2 2 30";
        let pos = Position::from_str(tps).unwrap();
        let cases = [1, 197, 15300, 2616619, 215768669];
        for (depth, &case) in cases.iter().enumerate() {
            assert_eq!(case, perft(&pos, depth as i32));
        }
    }

    #[test]
    fn max_stacks() {
        let tps = "x6/x6/x6/x3,111222111222111222111222111222111222111222111222111222111222C,x2/x6/x6 2 31";
        let pos = Position::from_str(tps).unwrap();
        let cases = [1, 194, 11334, 963760, 56988512, 6439471288];
        for (depth, &case) in cases.iter().enumerate() {
            assert_eq!(case, perft(&pos, depth as i32));
        }
    }

    #[test]
    fn capstone_blocking() {
        let tps = "x6/x4,1S,x/x2,21111S,1C,22122C,x/x6/x6/x6 2 11";
        let pos = Position::from_str(tps).unwrap();
        let cases = [1, 95, 11683, 1035124, 111863932];
        for (depth, &case) in cases.iter().enumerate() {
            assert_eq!(case, perft(&pos, depth as i32));
        }
    }
}
