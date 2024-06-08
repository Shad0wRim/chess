mod board;
mod counter;
mod parser;
mod pieces;
mod turn;

pub use board::{ChessBoard, DrawType, GameState, TurnError, Win, WinType};
pub use parser::parse_move;
pub use turn::Turn;

use counter::Counter;
use std::io;

use std::error::Error;

#[derive(Debug)]
pub struct ChessGame {
    board: ChessBoard,
    game_state: GameState,
    position_counter: Counter<String>,
    game_hist: Vec<Turn>,
    pub rotate_board: RotateBoard,
    pub allow_undo: bool,
    pub players: (String, String),
    pub enforce_flags: bool,
}

impl ChessGame {
    pub fn builder() -> ChessGameBuilder {
        ChessGameBuilder::default()
    }
    pub fn gen_fen(&self) -> String {
        self.board.gen_fen()
    }
    pub fn gen_pgn(&self) -> String {
        let mut contents = String::new();
        let result = match self.board.check_gamestate(&self.position_counter) {
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
        contents
    }
    pub fn reset(&mut self) {
        *self = Self {
            board: ChessBoard::default(),
            game_state: GameState::default(),
            position_counter: Counter::default(),
            game_hist: Vec::default(),
            rotate_board: self.rotate_board,
            allow_undo: self.allow_undo,
            players: self.players.clone(),
            enforce_flags: self.enforce_flags,
        }
    }
    pub fn display_win_message(&self) {
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
    pub fn make_move(&mut self, turn: &Turn) -> Result<(), TurnError> {
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
        let trimmed_fen = self
            .board
            .gen_fen()
            .split_whitespace()
            .take(4)
            .collect::<Vec<_>>()
            .join(" ");
        self.position_counter.add(trimmed_fen);
        self.board.update_board(&full_turn);
        self.game_hist.push(full_turn);

        self.game_state = self.board.check_gamestate(&self.position_counter);
        Ok(())
    }
    pub fn undo_move(&mut self) {
        self.game_hist.pop();
        let history = self.game_hist.clone();

        self.game_hist = Vec::new();
        self.position_counter = Counter::new();
        self.board = ChessBoard::default();
        for turn in history {
            self.make_move(&turn).unwrap();
        }
    }
    pub fn display(&self) {
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

impl ChessGame {
    pub fn board(&self) -> &ChessBoard {
        &self.board
    }
    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }
    pub fn game_state_mut(&mut self) -> &mut GameState {
        &mut self.game_state
    }
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RotateBoard {
    White,
    Black,
    Rotate,
}

impl Default for ChessGame {
    fn default() -> Self {
        ChessGame {
            board: ChessBoard::default(),
            game_state: GameState::default(),
            position_counter: Counter::new(),
            game_hist: Vec::default(),
            rotate_board: RotateBoard::White,
            allow_undo: false,
            players: ("White".to_string(), "Black".to_string()),
            enforce_flags: true,
        }
    }
}
pub fn print_all_errors<T: Error + ?Sized>(err: &T) {
    println!("{}", err);
    let mut next = err.source();
    while let Some(e) = next {
        println!("{}", e);
        next = e.source();
    }
}

pub fn play_game(game: &mut ChessGame) {
    game.display();
    if *game.game_state() != GameState::Continue {
        game.display_win_message();
        return;
    }

    loop {
        let outcome: Result<bool, Box<dyn Error>> = (|| {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;

            if game.allow_undo && buf.trim().to_lowercase() == "u" {
                game.undo_move();
                return Ok(false);
            } else if !game.allow_undo && buf.trim().to_lowercase() == "u" {
                return Err("Undoing moves is not allowed".into());
            } else if buf.trim().to_lowercase() == "q" {
                return Ok(true);
            }

            let turn = parse_move(buf.trim())?;
            let _full_turn = game.make_move(&turn)?;
            Ok(false)
        })();
        match outcome {
            Ok(quit) if quit => {
                println!("Quitting game");
                return;
            }
            Err(e) => {
                print_all_errors(e.as_ref());
                continue;
            }
            _ => (),
        }

        game.display();

        if *game.game_state() == GameState::Continue {
            continue;
        } else {
            game.display_win_message();
            return;
        }
    }
}

pub fn play_from_pgn(game: &mut ChessGame, file_string: String) {
    let mut buf = String::new();
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
        game.players.0 = white_name;
    }
    if let Some(black_name) = extract_field("Black") {
        game.players.1 = black_name;
    }

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

    game.display();

    let pgn_read_result = (|| -> Result<GameState, Box<dyn Error>> {
        for r#move in moves {
            io::stdin().read_line(&mut buf)?;
            game.make_move(&r#move)?;
            game.display();
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
            "1-0" => Ok(GameState::Win(Win {
                is_white: true,
                kind: WinType::Resign,
            })),
            "0-1" => Ok(GameState::Win(Win {
                is_white: false,
                kind: WinType::Resign,
            })),
            "*" => Ok(GameState::Continue),
            _ => Err("Did not find a result for the game".into()),
        }
    })();

    match pgn_read_result {
        Ok(game_result) => match game.game_state() {
            GameState::Continue => match game_result {
                GameState::Continue => play_game(game),
                _ => {
                    *game.game_state_mut() = game_result;
                    game.display_win_message();
                }
            },
            _ => game.display_win_message(),
        },
        Err(e) => {
            print_all_errors(e.as_ref());
        }
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
