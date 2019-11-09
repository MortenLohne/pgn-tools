use std::io::Write;
use std::{error, io};

use pgn_reader::{BufferedReader, RawHeader, SanPlus, Skip, Visitor};
use shakmaty::fen::Fen;
use shakmaty::{fen, Position};
use std::error::Error;

struct Pgn2FenVisitor<'w, W> {
    chess: Result<shakmaty::Chess, Box<dyn error::Error>>,
    writer: &'w mut W,
}

impl<'w, W> Pgn2FenVisitor<'w, W> {
    fn new(writer: &'w mut W) -> Self {
        Pgn2FenVisitor {
            chess: Ok(shakmaty::Chess::default()),
            writer,
        }
    }
}

impl<'w, W: Write> Visitor for Pgn2FenVisitor<'w, W> {
    type Result = ();

    fn begin_game(&mut self) {
        self.chess = Ok(shakmaty::Chess::default());
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        if key == b"FEN" {
            let key_str = std::str::from_utf8(value.0).unwrap();
            match key_str.parse::<Fen>() {
                Ok(setup) => {
                    self.chess = setup
                        .position()
                        .map_err(|err| Box::new(err) as Box<dyn Error>)
                }
                Err(err) => self.chess = Err(Box::new(err)),
            }
        }
    }

    fn san(&mut self, san_plus: SanPlus) {
        if self.chess.is_ok() {
            match san_plus.san.to_move(self.chess.as_ref().unwrap()) {
                Ok(mv) => self.chess.as_mut().unwrap().play_unchecked(&mv),
                Err(err) => {
                    self.chess = Err(Box::new(err));
                    return;
                }
            }
            let fen = fen::fen(self.chess.as_ref().unwrap());
            writeln!(self.writer, "{}", fen).unwrap();
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {}
}

pub fn pgn2fen<R: io::Read, W: io::Write>(reader: &mut R, writer: &mut W) {
    let mut pgn_reader = BufferedReader::new(reader);
    let mut visitor = Pgn2FenVisitor::new(writer);

    let mut successes: u64 = 0;
    let mut failures: u64 = 0;

    loop {
        let result = pgn_reader.read_game(&mut visitor);

        if let Ok(None) = result {
            break;
        }

        match visitor.chess {
            Ok(_) => successes += 1,
            Err(ref err) => {
                eprintln!("{}", err);
                failures += 1;
            }
        }
    }
    eprintln!(
        "Parsed {}/{} pgns successfully",
        successes,
        successes + failures
    );
}
