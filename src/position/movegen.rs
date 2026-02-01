use super::Position;
use crate::types::{Bitboard, Dir, Move, PieceType, Square};
use smallvec::SmallVec;

pub type MoveList = SmallVec<[Move; 256]>;

#[derive(Debug)]
struct Limits {
    distance: [usize; 4],
    blocker: [Square; 4],
}

impl Position {
    pub fn generate_moves(&self, ml: &mut MoveList) {
        ml.clear();

        if self.ply < 2 {
            self.append_placements(ml, PieceType::Flat);
            return;
        }

        if self.remaining_stones[self.stm.to_index()] > 0 {
            self.append_placements(ml, PieceType::Flat);
            self.append_placements(ml, PieceType::Wall);
        }
        if self.remaining_caps[self.stm.to_index()] > 0 {
            self.append_placements(ml, PieceType::Cap);
        }
        self.append_spreads(ml);
    }

    fn append_placements(&self, ml: &mut MoveList, pt: PieceType) {
        for sq in !self.occupied() {
            ml.push(Move::place(pt, sq));
        }
    }

    fn append_spreads(&self, ml: &mut MoveList) {
        let src_bb = self.colors[self.stm.to_index()];
        for sq in src_bb {
            let limits = self.spread_calc(sq);
            let is_cap = self.piece_on(sq).piece_type() == PieceType::Cap;
            let height = self.height(sq) as usize;

            for d in 0..4 {
                let dir = Dir::from_index(d);
                let blocker = limits.blocker[d];
                let blocker_distance = limits.distance[d];

                let max_spread_dist = height.min(blocker_distance);
                let max_pickup = height.min(6);

                let can_crush = is_cap
                    && blocker.is_some()
                    && self.piece_on(blocker).piece_type() == PieceType::Wall
                    && max_pickup > blocker_distance;

                if max_spread_dist != 0 {
                    let splat_limit: u8 = 1 << max_pickup;
                    for splat in 1..splat_limit {
                        if splat.count_ones() as usize <= max_spread_dist {
                            ml.push(Move::spread(sq, dir, splat));
                        }
                    }
                }
                if can_crush {
                    let splat_limit: u8 = 1 << (max_pickup - 1);
                    for splat in 0..splat_limit {
                        if splat.count_ones() as usize == blocker_distance {
                            let splat = splat | 1 << splat.bit_width();
                            ml.push(Move::spread(sq, dir, splat));
                        }
                    }
                }
            }
        }
    }

    fn spread_calc(&self, sq: Square) -> Limits {
        let bit = sq.to_bitboard();

        let (file, rank) = sq.to_file_and_rank();
        let file_mask = Bitboard::file_mask(file);
        let rank_mask = Bitboard::rank_mask(rank);

        let n_blocker = self.all_royals() & file_mask & (!bit + Bitboard(1)) & !bit;
        let e_blocker = self.all_royals() & rank_mask & (!bit + Bitboard(1)) & !bit;
        let s_blocker = self.all_royals() & file_mask & (bit - Bitboard(1));
        let w_blocker = self.all_royals() & rank_mask & (bit - Bitboard(1));

        let n_blocker = n_blocker.lsb();
        let e_blocker = e_blocker.lsb();
        let s_blocker = s_blocker.msb();
        let w_blocker = w_blocker.msb();

        let n_rank = if n_blocker.is_none() {
            5
        } else {
            n_blocker.rank() - 1
        };
        let e_file = if e_blocker.is_none() {
            5
        } else {
            e_blocker.file() - 1
        };
        let s_rank = if s_blocker.is_none() {
            0
        } else {
            s_blocker.rank() + 1
        };
        let w_file = if w_blocker.is_none() {
            0
        } else {
            w_blocker.file() + 1
        };

        Limits {
            distance: [n_rank - rank, e_file - file, rank - s_rank, file - w_file],
            blocker: [n_blocker, e_blocker, s_blocker, w_blocker],
        }
    }
}
