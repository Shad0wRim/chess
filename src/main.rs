// chess
// new features:
//    customizable board, can place pieces wherever and then start playing
//
//    move a cursor around with arrow keys to select pieces, and show legal moves
// TODO refactor once it works
//
use chess::*;
use std::io;

fn main() {
    let mut buf = String::new();
    println!("Whose perspective? (W, B, R)");
    io::stdin().read_line(&mut buf).unwrap();
    let rotate_option = match buf.to_lowercase().trim() {
        "w" => RotateBoard::White,
        "b" => RotateBoard::Black,
        "r" => RotateBoard::Rotate,
        _ => RotateBoard::White,
    };

    let mut game = ChessGame::builder()
        .rotate_board(RotateBoard::White)
        .allow_undo(true)
        .rotate_board(rotate_option)
        .players(("White".to_owned(), "Black".to_owned()))
        .enforce_flags(true)
        .build();

    // game.play_from_pgn("my_pgn.pgn");
    game.play_game();
}
