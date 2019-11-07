extern crate pgn_reader;
extern crate shakmaty;

use std::io;
use pgn_reader::{Visitor, Skip, BufferedReader, SanPlus};

struct MoveReader {
    last_move: Option<shakmaty::san::San>,
}

impl MoveReader {
    fn new() -> Self {
        MoveReader { last_move: None }
    }
}

impl Visitor for MoveReader {
    type Result = shakmaty::san::San;

    fn san(&mut self, san_plus: SanPlus) {
        self.last_move = Some(san_plus.san);
    }

    fn end_game(&mut self) -> Self::Result {
        self.last_move.take().unwrap()
    }
}

fn main() {
    let input = io::stdin();
    let mut reader = BufferedReader::new(input);

    while let Ok(last_move) = reader.read_game(&mut MoveReader::new()) {
        println!("Read move {:?}!", last_move);
    }

}
