// chess
// new features:
//    customizable board, can place pieces wherever and then start playing
//
//    move a cursor around with arrow keys to select pieces, and show legal moves
// TODO refactor once it works
//
use chess::ChessGame;
use std::io;

fn main() {
   let mut game = ChessGame::builder()
      .rotate_board(false)
      .allow_undo(true)
      .players(("White".to_owned(), "Black".to_owned()))
      .enforce_flags(true)
      .build();

   // game.play_from_pgn("my_pgn.pgn");
   game.play_game();

   // let mut filename = String::new();
   // println!("Enter pgn filename");
   // io::stdin().read_line(&mut filename).unwrap();
   // game.create_pgn(filename.trim()).unwrap();
}
