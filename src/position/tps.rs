use crate::types::{Bitboard, Color, Piece, PieceType, Square};
use std::str::FromStr;
use thiserror::Error;

use super::Position;

#[derive(Error, Debug)]
pub enum TpsError {
    #[error("incorrect number of whitespace-separated components in tps")]
    WrongPartCount,
    #[error("incorrect number of ranks in tps board component")]
    WrongRankCount,
    #[error("incorrect number of files in tps board component")]
    WrongFileCount,
    #[error("invalidly specified empty square in tps board component")]
    InvalidEmptySquare,
    #[error("S or C found in the middle of a stack in tps board component")]
    NonTrailingPieceType,
    #[error("invalid character")]
    InvalidCharacter,
    #[error("too many stones on board in tps board component")]
    TooManyStones,
    #[error("too many capstones on board in tps board component")]
    TooManyCaps,
    #[error("invalid tps side-to-move component")]
    InvalidSideToMove,
    #[error("invalid tps full-move counter component")]
    InvalidFullMoveCounter,
}

impl FromStr for Position {
    type Err = TpsError;

    fn from_str(s: &str) -> Result<Position, TpsError> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        if parts.len() == 3 {
            Position::parse_from_parts(parts[0], parts[1], parts[2])
        } else {
            Err(TpsError::WrongPartCount)
        }
    }
}

impl Position {
    pub fn parse_from_parts(board: &str, stm: &str, fullmove: &str) -> Result<Position, TpsError> {
        let ranks: Vec<_> = board.split('/').collect();
        if ranks.len() != 6 {
            return Err(TpsError::WrongRankCount);
        }

        let mut colors = [Bitboard::default(); Color::NUM];
        let mut tops = [Bitboard::default(); PieceType::NUM];
        let mut mailbox = [Piece::None; Square::NUM];
        let mut stacks = [0u64; Square::NUM];
        let mut heights = [0u8; Square::NUM];
        let mut stone_count = [0u8; Color::NUM];
        let mut cap_count = [0u8; Color::NUM];

        for (inv_rank, rank_str) in ranks.iter().enumerate() {
            let rank = 5 - inv_rank;
            let elements: Vec<_> = rank_str.split(',').collect();
            let mut file = 0;

            for element in elements {
                if file >= 6 {
                    return Err(TpsError::WrongFileCount);
                }

                if element.is_empty() {
                    return Err(TpsError::InvalidEmptySquare);
                } else if let Some(count) = element.strip_prefix("x") {
                    let count = match count {
                        "" => 1,
                        _ => match count.parse::<usize>() {
                            Ok(count) => count,
                            Err(_) => return Err(TpsError::InvalidEmptySquare),
                        },
                    };
                    file += count;
                } else {
                    let sq = Square::from_file_and_rank(file, rank);
                    file += 1;

                    let mut height: u8 = 0;
                    let mut stack: u64 = 0;
                    let mut top = PieceType::None;

                    for ch in element.chars() {
                        if top.is_some() {
                            return Err(TpsError::NonTrailingPieceType);
                        }

                        match ch {
                            'S' => top = PieceType::Wall,
                            'C' => top = PieceType::Cap,
                            '1' => {
                                stone_count[0] += 1;
                                height += 1;
                                stack <<= 1;
                            }
                            '2' => {
                                stone_count[1] += 1;
                                height += 1;
                                stack = (stack << 1) | 1;
                            }
                            _ => return Err(TpsError::InvalidCharacter),
                        }
                    }

                    if height == 0 {
                        return Err(TpsError::InvalidEmptySquare);
                    }

                    let top = if top.is_some() { top } else { PieceType::Flat };
                    let top_color = Color::from_index((stack & 1) as u8);

                    if top == PieceType::Cap {
                        stone_count[top_color.to_index()] -= 1;
                        cap_count[top_color.to_index()] += 1;
                    }

                    colors[top_color.to_index()].set(sq);
                    tops[top.to_index()].set(sq);

                    mailbox[sq.to_index()] = Piece::new(top_color, top);
                    stacks[sq.to_index()] = stack;
                    heights[sq.to_index()] = height;
                }
            }

            if file != 6 {
                return Err(TpsError::WrongFileCount);
            }
        }

        let max_stones = 30;
        let max_caps = 1;

        if stone_count.iter().any(|&x| x > max_stones) {
            return Err(TpsError::TooManyStones);
        }
        if cap_count.iter().any(|&x| x > max_caps) {
            return Err(TpsError::TooManyCaps);
        }

        let remaining_stones = [max_stones - stone_count[0], max_stones - stone_count[1]];
        let remaining_caps = [max_caps - cap_count[0], max_caps - cap_count[1]];

        let stm = match stm {
            "1" => Color::P1,
            "2" => Color::P2,
            _ => return Err(TpsError::InvalidSideToMove),
        };

        let Ok(fullmove) = fullmove.parse::<u16>() else {
            return Err(TpsError::InvalidFullMoveCounter);
        };
        let ply = (fullmove - 1) * 2 + stm.to_index() as u16;

        Ok(Position {
            stm,
            ply,
            colors,
            tops,
            mailbox,
            stacks,
            heights,
            remaining_stones,
            remaining_caps,
        })
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = {
            let mut ranks = Vec::new();
            for rank in (0..6).rev() {
                let mut empty_squares = 0;
                let mut elements = Vec::new();

                for file in 0..6 {
                    let sq = Square::from_file_and_rank(file, rank);

                    if self.is_empty(sq) {
                        empty_squares += 1;
                        continue;
                    }

                    if empty_squares > 0 {
                        elements.push(if empty_squares == 1 {
                            "x".to_string()
                        } else {
                            format!("x{}", empty_squares)
                        });
                        empty_squares = 0;
                    }

                    let height = self.heights[sq.to_index()];
                    let stack = self.stacks[sq.to_index()];

                    let mut element: String = (0..height)
                        .rev()
                        .map(|i| match (stack >> i) & 1 {
                            0 => '1',
                            1 => '2',
                            _ => unreachable!(),
                        })
                        .collect();

                    element += match self.mailbox[sq.to_index()].piece_type() {
                        PieceType::Flat | PieceType::None => "",
                        PieceType::Cap => "C",
                        PieceType::Wall => "S",
                    };

                    elements.push(element);
                }

                if empty_squares > 0 {
                    elements.push(if empty_squares == 1 {
                        "x".to_string()
                    } else {
                        format!("x{}", empty_squares)
                    });
                }

                ranks.push(elements.join(","));
            }

            ranks.join("/")
        };

        let fullmove = (self.ply / 2) + 1;

        write!(f, "{board} {} {fullmove}", self.stm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_tps() {
        let cases = [        "x6/x6/x6/x6/x6/x6 1 1",
        "2,2,2,2,2,x/x6/x6/x6/x6/x6 1 1",
        "x3,12,2S,x/x,22S,22C,11,21,x/121,212,12,1121C,1212S,x/21S,1,21,211S,12S,x/x,21S,2,x3/x6 1 26",
        "1,x,1,1,1,x/1,11112C,111121C,2S,x,1/2,x,1122,2S,1,1/2,x,2S,x2,2/2,2,1S,2,2221S,2/2,x,112,x2,2 1 33",
        "1,x,1,x3/x,2,1121C,x3/1112,x,2,x3/2,2,x,2,x2/x2,1212,x3/2C,1,1,x3 1 18",
        "2,x2,1,2,1/x3,1,2C,1/x3,121C,12,1/x2,2,2,2,2/x4,1,x/x4,1,x 1 12",
        "1,x,1,x3/x2,1111212,2,x2/x,1,21,212,x,1/x2,21,2,2,2C/x2,21C,x2,2/2,x5 2 22",
        "x2,2,x2,1/x,122,121C,212,x2/x2,2,2,2,2C/1,x2,2,x,1/x2,1,x2,1/1,x2,2111112,x,1 2 24",
        "2,x,222212,x,2,12C/x,221S,x2,2,x/21,x3,1,x/22,221,1121S,x,1,x/2112,221S,x4/x,1C,1,1,x2 2 48",
        "2,x2,2,1,x/2,x,12,x,1,112S/2,21S,221C,211111,2,1/x,1S,x,22212C,x2/1S,2,2,x2,21/x,21121,2,12,2,2 2 36",];

        for case in cases {
            let position = Position::from_str(case).unwrap();
            let tps = position.to_string();
            assert_eq!(case, tps);
        }
    }
}
