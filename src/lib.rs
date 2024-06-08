mod board;
mod parser;
mod pieces;
mod turn;

use board::{ChessBoard, DrawType, GameState, TurnError, Win, WinType};
use parser::parse_move;
use turn::Turn;

use std::error::Error;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct ChessGame {
    board: ChessBoard,
    game_state: GameState,
    game_hist: Vec<Turn>,
    rotate_board: RotateBoard,
    allow_undo: bool,
    players: (String, String),
    enforce_flags: bool,
}

impl ChessGame {
    pub fn builder() -> ChessGameBuilder {
        ChessGameBuilder::default()
    }
    pub fn play_game(&mut self) {
        self.display();
        loop {
            let outcome: Result<(), Box<dyn Error>> = (|| {
                let mut buf = String::new();
                io::stdin().read_line(&mut buf)?;

                if self.allow_undo && buf.trim().to_lowercase() == "u" {
                    self.game_hist.pop();
                    self.board = ChessBoard::default();
                    for turn in self.game_hist.clone() {
                        self.make_move(&turn)?;
                    }
                    return Ok(());
                }

                let turn = parse_move(buf.trim())?;
                let full_turn = self.make_move(&turn)?;

                self.game_hist.push(full_turn);
                Ok(())
            })();
            if let Err(e) = outcome {
                print_all_errors(e.as_ref());
                continue;
            };

            self.display();

            if self.game_state == GameState::Continue {
                continue;
            } else {
                self.display_win_message();
                return;
            }
        }
    }
    pub fn play_from_pgn(&mut self, filename: &str) {
        let mut buf = String::new();
        let Ok(file_string) = fs::read_to_string(filename) else {
            println!("Failed to open file `{}`", filename);
            return;
        };
        let scan_between = |open_delimiter: char, close_delimiter: char, keep: bool| {
            move |is_between: &mut bool, ch: char| {
                if open_delimiter == close_delimiter && ch == open_delimiter {
                    *is_between = !*is_between;
                    Some('�')
                } else if ch == open_delimiter {
                    *is_between = true;
                    Some('�')
                } else if ch == close_delimiter {
                    *is_between = false;
                    Some('�')
                } else if *is_between {
                    if keep {
                        Some(ch)
                    } else {
                        Some('�')
                    }
                } else if keep {
                    Some('�')
                } else {
                    Some(ch)
                }
            }
        };
        let extract_field = |field: &str| {
            Some(
                file_string
                    .split_terminator('\n')
                    .find(|line| line.contains(field))?
                    .chars()
                    .scan(false, scan_between('"', '"', true))
                    .filter(|&ch| ch != '�')
                    .collect::<String>(),
            )
        };
        if let Some(white_name) = extract_field("White") {
            self.players.0 = white_name;
        }
        if let Some(black_name) = extract_field("Black") {
            self.players.1 = black_name;
        }

        let pgn_read_result = (|| -> Result<GameState, Box<dyn Error>> {
            let moves: Vec<_> = file_string
                .split_terminator('\n')
                .skip_while(|line| line.starts_with('[') || line.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
                .chars()
                .scan(false, scan_between('{', '}', false))
                .filter(|&ch| ch != '�')
                .collect::<String>()
                .split_whitespace()
                .map(|substr| {
                    substr
                        .split('.')
                        .last()
                        .expect("split always produces an iterator")
                })
                .filter_map(|turn| turn.parse::<Turn>().ok())
                .collect();

            (1..12).for_each(|_| println!());
            self.display();

            for r#move in moves {
                io::stdin().read_line(&mut buf)?;
                let full_turn = self.make_move(&r#move)?;
                self.game_hist.push(full_turn);
                self.display();
            }

            let game_result = match extract_field("Result") {
                Some(result) => result,
                None => file_string
                    .split_whitespace()
                    .last()
                    .ok_or("Empty file")?
                    .to_string(),
            };

            match game_result.as_str() {
                "1/2-1/2" => Ok(GameState::Draw(DrawType::Offer)),
                "1-0" => Ok(GameState::Win(board::Win {
                    is_white: true,
                    kind: WinType::Resign,
                })),
                "0-1" => Ok(GameState::Win(board::Win {
                    is_white: false,
                    kind: WinType::Resign,
                })),
                "*" => Ok(GameState::Continue),
                _ => Err("Did not find a result for the game".into()),
            }
        })();

        match pgn_read_result {
            Ok(game_result) => match self.game_state {
                GameState::Continue => match game_result {
                    GameState::Continue => self.play_game(),
                    _ => {
                        self.game_state = game_result;
                        self.display_win_message();
                    }
                },
                _ => self.display_win_message(),
            },
            Err(e) => {
                print_all_errors(e.as_ref());
            }
        }
    }
    pub fn create_pgn(&self, filename: &str) -> Result<(), io::Error> {
        let mut contents = String::new();
        let result = match self.board.check_gamestate() {
            GameState::Continue => "*",
            GameState::Win(Win { is_white: true, .. }) => "1-0",
            GameState::Win(Win {
                is_white: false, ..
            }) => "0-1",
            GameState::Draw(_) => "1/2-1/2",
        };
        let mut test_board = ChessBoard::default();
        for (turn_num, moves) in self.game_hist.chunks(2).enumerate() {
            contents.push_str(&format!("{}. ", turn_num + 1));
            for r#move in moves {
                let minimum_move = test_board.get_minimum_move(r#move);
                test_board.update_board(r#move);
                contents.push_str(&minimum_move.to_string());
                contents.push(' ');
            }
            if turn_num % 10 == 9 {
                contents.push('\n');
            }
        }
        contents.push_str(result);

        fs::write(filename, contents)
    }
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    pub fn set_rotate_board(&mut self, rotate_board: RotateBoard) {
        self.rotate_board = rotate_board;
    }
    pub fn set_allow_undo(&mut self, allow_undo: bool) {
        self.allow_undo = allow_undo;
    }
    fn display_win_message(&self) {
        match self.game_state {
            GameState::Win(win) => {
                match win.is_white {
                    true => print!("{} wins by ", self.players.0),
                    false => print!("{} wins by ", self.players.1),
                }
                match win.kind {
                    WinType::Checkmate => println!("checkmate"),
                    WinType::Resign => println!("resignation"),
                }
            }
            GameState::Draw(draw) => {
                print!("The game is a draw by ");
                match draw {
                    DrawType::Stalemate => println!("stalemate"),
                    DrawType::FiftyMove => println!("the fifty move rule"),
                    DrawType::ThreefoldRepitition => println!("threefold repitition"),
                    DrawType::InsufficientMaterial => println!("insufficient material"),
                    DrawType::Offer => println!("draw offer"),
                }
            }
            GameState::Continue => (),
        };
    }
    fn make_move(&mut self, turn: &Turn) -> Result<Turn, TurnError> {
        let full_turn = self.board.validate_and_complete_turn(*turn)?;
        if let Turn::Move(r#move) = full_turn {
            let Some(board::Source::Square(_)) = r#move.src else {
                panic!("Invalid output from validate_and_complete_turn");
            };
        }
        let full_turn = if self.enforce_flags {
            self.board.enforce_flags(&full_turn)?;
            full_turn
        } else {
            self.board.gen_flags(full_turn)
        };
        self.board.update_board(&full_turn);
        self.game_state = self.board.check_gamestate();
        Ok(full_turn)
    }
    fn display(&self) {
        // const ED2: &str = "\x1b[2J";
        const ED0: &str = "\x1b[J";
        const CUP: &str = "\x1b[H";
        let buf = match self.rotate_board {
            RotateBoard::White => format!("{}", self.board),
            RotateBoard::Black => format!("{:#}", self.board),
            RotateBoard::Rotate if self.board.is_white() => format!("{}", self.board),
            RotateBoard::Rotate => format!("{:#}", self.board),
        };
        // print!("{}", ED2);
        print!("{}", CUP);
        print!("{}", ED0);
        print!("{}", buf);
        let curr_player = if self.board.is_white() {
            &self.players.0
        } else {
            &self.players.1
        };
        println!("{}'s turn", curr_player);
    }
}
impl Default for ChessGame {
    fn default() -> Self {
        ChessGame {
            board: ChessBoard::default(),
            game_state: GameState::default(),
            game_hist: Vec::default(),
            rotate_board: RotateBoard::White,
            allow_undo: false,
            players: ("White".to_string(), "Black".to_string()),
            enforce_flags: true,
        }
    }
}
fn print_all_errors<T: Error + ?Sized>(err: &T) {
    println!("{}", err);
    let mut next = err.source();
    while let Some(e) = next {
        println!("{}", e);
        next = e.source();
    }
}

pub struct ChessGameBuilder {
    rotate_board: RotateBoard,
    allow_undo: bool,
    players: (String, String),
    enforce_flags: bool,
}
impl Default for ChessGameBuilder {
    fn default() -> Self {
        ChessGameBuilder {
            rotate_board: RotateBoard::White,
            allow_undo: false,
            players: (String::from("White"), String::from("Black")),
            enforce_flags: true,
        }
    }
}
impl ChessGameBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn rotate_board(&mut self, val: RotateBoard) -> &mut Self {
        self.rotate_board = val;
        self
    }
    pub fn allow_undo(&mut self, val: bool) -> &mut Self {
        self.allow_undo = val;
        self
    }
    pub fn players(&mut self, names: (String, String)) -> &mut Self {
        self.players = names;
        self
    }
    pub fn enforce_flags(&mut self, val: bool) -> &mut Self {
        self.enforce_flags = val;
        self
    }
    pub fn build(&mut self) -> ChessGame {
        ChessGame {
            rotate_board: self.rotate_board,
            allow_undo: self.allow_undo,
            players: self.players.clone(),
            enforce_flags: self.enforce_flags,
            ..ChessGame::default()
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RotateBoard {
    White,
    Black,
    Rotate,
}
