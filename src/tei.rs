use std::time::Instant;

use crate::{perft, position::Position};

#[derive(Default)]
pub struct Interface {
    position: Position,
}

impl Interface {
    pub fn parse_line(&mut self, line: &str) {
        let start_time = Instant::now();

        let mut it = line.split_ascii_whitespace();
        let Some(cmd) = it.next() else {
            return;
        };

        match cmd {
            "position" => self.parse_position(it),
            "moves" => self.parse_moves(it),
            "perft" => self.parse_perft(it),
            _ => self.print_protocol_error(cmd, "Unknown command"),
        }
    }

    fn parse_position<'a, I: Iterator<Item = &'a str>>(&mut self, mut it: I) {
        let Some(pos_type) = it.next() else {
            self.print_protocol_error("position", "Empty position");
            return;
        };

        match pos_type {
            "startpos" => {
                self.position = Position::default();
            }
            "tps" => {
                let Ok([board, stm, fullmove]) = it.next_chunk::<3>() else {
                    self.print_protocol_error(
                        "position",
                        "incorrect number of whitespace-separated components in tps",
                    );
                    return;
                };

                match Position::parse_from_parts(board, stm, fullmove) {
                    Ok(position) => self.position = position,
                    Err(err) => {
                        self.print_protocol_error("position", &format!("cannot parse tps: {err}"));
                        return;
                    }
                }
            }
            _ => {
                self.print_unrecognised_token("position", pos_type);
                return;
            }
        }

        match it.next() {
            None => {}
            Some("moves") => self.parse_moves(it),
            Some(token) => {
                self.print_unrecognised_token("position", token);
                return;
            }
        }
    }

    fn parse_moves<'a, I: Iterator<Item = &'a str>>(&mut self, mut it: I) {
        while let Some(mstr) = it.next() {
            match mstr.parse() {
                Ok(mv) => {
                    self.position = self.position.make_move(mv);
                    self.position.verify();
                }
                Err(err) => {
                    self.print_protocol_error("moves", &format!("invalid move string: {err}"));
                    return;
                }
            }
        }
    }

    fn parse_perft<'a, I: Iterator<Item = &'a str>>(&mut self, mut it: I) {
        let depth = it.next().unwrap_or("1");
        match depth.parse() {
            Ok(depth) => perft::splitperft(&self.position, depth),
            Err(err) => {
                self.print_protocol_error("perft", &format!("invalid depth argument: {err}"));
                return;
            }
        }
    }

    fn print_protocol_error(&self, cmd: &str, msg: &str) {
        println!("info error ({cmd}): {msg}");
    }

    fn print_unrecognised_token(&self, cmd: &str, token: &str) {
        self.print_protocol_error(cmd, &format!("unrecognised token `{}`", token));
    }
}
