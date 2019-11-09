extern crate pgn_reader;
extern crate shakmaty;

mod pgn2fen;
mod tests;

use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("png2fen") => pgn2fen::pgn2fen(&mut io::stdin(), &mut io::stdout()),
        Some(s) => eprintln!("{} is not a pgn-tools command.", s),
        None => eprintln!("Usage: pgn-tools png2fen"),
    }
}
