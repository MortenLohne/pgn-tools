use std::fs;
use std::io;
use std::io::Read;
use crate::pgn2fen;

#[test]
fn pgn2fen_test() {
    let mut input = fs::File::open("src/tests/rasher_vs_curryfish.pgn").unwrap();
    let expected_output = fs::File::open("src/tests/rasher_vs_curryfish_fens.txt").unwrap();

    let mut output = io::Cursor::new(Vec::new());
    pgn2fen::pgn2fen(&mut input, &mut output);

    assert!(expected_output.bytes().map(Result::unwrap)
        .eq(output.into_inner().into_iter()));
}

#[test]
fn pgn2fen_test2() {
    let mut input = fs::File::open("src/tests/TCEC_Season_16_-_Superfinal.pgn").unwrap();
    let expected_output = fs::File::open("src/tests/TCEC_Season_16_-_Superfinal_fens.txt").unwrap();

    let mut output = io::Cursor::new(Vec::new());
    pgn2fen::pgn2fen(&mut input, &mut output);

    assert!(expected_output.bytes().map(Result::unwrap)
        .eq(output.into_inner().into_iter()));
}