#![feature(iter_next_chunk)]
#![feature(portable_simd)]
#![feature(uint_bit_width)]

mod perft;
mod position;
mod tei;
mod types;

pub const KOMI: u32 = 2;

fn main() {
    let mut interface = tei::Interface::default();

    let mut line = String::new();
    while let Ok(read_bytes) = std::io::stdin().read_line({
        line.clear();
        &mut line
    }) && read_bytes > 0
    {
        interface.parse_line(&line);
    }
}
