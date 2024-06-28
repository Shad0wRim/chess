#![warn(missing_docs)]
//! Chess board representation
//!
//! Provides a board representation to create a chess game
/// Module that concerns the board state
pub mod board;
/// Module that parses algebraic chess notation into a [Turn]
pub mod parser;
/// Module that concerns the pieces
pub mod pieces;
/// Module that concerns the turn/move descriptions
pub mod turn;
/// Utility structs and functions for miscellaneous tasks
pub mod utils;

use board::{ChessBoard, DrawType, GameState, TurnError, Win, WinType};
use turn::Turn;

use utils::Counter;

#[derive(Debug, Clone)]
/// Structure that holds the chess board, game history, and configuration data
pub struct ChessGame {
    board: ChessBoard,
    /// The current game state [GameState]
    pub game_state: GameState,
    position_counter: Counter<String>,
    game_hist: Vec<Turn>,
    /// Sets the perspective that the game is played from, White, Black, or switching between them
    pub rotate_board: RotateBoard,
    /// Sets whether move undos are allowed
    pub allow_undo: bool,
    /// The names of the players playing
    pub players: (String, String),
    /// Sets whether or not the check `+`, capture `x`, and checkmate `#` flags must be specified or
    /// will be autogenerated for the user input
    pub enforce_flags: bool,
}

impl ChessGame {
    /// associated function to make a builder for configuration data
    pub fn builder() -> ChessGameBuilder {
        ChessGameBuilder::default()
    }
    /// generates a fen string for the current board state
    pub fn gen_fen(&self) -> String {
        self.board.gen_fen()
    }
    /// generates a pgn string for the current game history
    pub fn gen_pgn(&self) -> String {
        let mut contents = String::new();
        let result = match self.board.check_gamestate(&self.position_counter) {
            GameState::Continue | GameState::Stop => "*",
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
    /// resets the state of the board, without resetting the configuration
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
    /// Displays the ending message describing the type of win, prints nothing if the game is ongoing
    pub fn display_end_message(&self) {
        match self.game_state {
            GameState::Win(win) => {
                match win.is_white {
                    true => print!("{} wins by ", self.players.0),
                    false => print!("{} wins by ", self.players.1),
                }
                match win.kind {
                    WinType::Checkmate => println!("checkmate"),
                    WinType::Resign => println!("resignation"),
                    WinType::Timeout => println!("timeout"),
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
            GameState::Stop => println!("The game was aborted"),
            GameState::Continue => (),
        };
    }
    /// Makes a move based on the inputted [Turn]
    ///
    /// # Side effects
    ///
    /// On success, updates the contained [ChessBoard], game_hist, game position_counter, and
    /// gamestate
    ///
    /// # Errors
    ///
    /// Returns an error if a move is not a legal chess move. If the enforce flags field is true,
    /// then will also return an error if the flags are incorrect
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
    /// Undoes the last move if the allow_undo flag is set
    ///
    /// # Side effects
    ///
    /// On success, reverts the state to the exact state before the last move was made
    ///
    /// # Errors
    ///
    /// Returns an error if the allow_undo flag is false
    pub fn undo_move(&mut self) -> Option<()> {
        if !self.allow_undo {
            return None;
        }
        self.game_hist.pop();
        let history = self.game_hist.clone();

        self.game_hist = Vec::new();
        self.position_counter = Counter::new();
        self.board = ChessBoard::default();
        for turn in history {
            self.make_move(&turn).unwrap();
        }
        Some(())
    }
    /// Displays the visual state of the board, depending on the perspective set in rotate_board
    pub fn display(&self) {
        const ED0: &str = "\x1b[J";
        const CUP: &str = "\x1b[H";
        let board = match self.rotate_board {
            RotateBoard::White => format!("{}", self.board),
            RotateBoard::Black => format!("{:#}", self.board),
            RotateBoard::Rotate if self.is_white() => format!("{}", self.board),
            RotateBoard::Rotate => format!("{:#}", self.board),
        };
        print!("{}", CUP);
        print!("{}", ED0);
        print!("{}", board);
        let curr_player = if self.board.is_white() {
            &self.players.0
        } else {
            &self.players.1
        };
        println!("{curr_player}'s turn");
    }
    /// Returns the string that representst the visual state of the board, depending on the
    /// perspective set in rotate_board
    pub fn board_string(&self) -> String {
        match self.rotate_board {
            RotateBoard::White => format!("{}", self.board),
            RotateBoard::Black => format!("{:#}", self.board),
            RotateBoard::Rotate if self.is_white() => format!("{}", self.board),
            RotateBoard::Rotate => format!("{:#}", self.board),
        }
    }
    /// Returns the string that says which player's turn it is
    pub fn player_string(&self) -> String {
        if self.is_white() {
            &self.players.0
        } else {
            &self.players.1
        }
        .clone()
            + "'s turn"
    }
    /// Returns true if the current player is white, false if it is black
    pub fn is_white(&self) -> bool {
        self.board.is_white()
    }
    /// Returns a reference to the game history
    pub fn game_hist(&self) -> &Vec<Turn> {
        &self.game_hist
    }
    /// Returns the turn with the least amount of information to fully specify a move, given a
    /// fully qualified move.
    ///
    /// # Panics
    ///
    /// Panics if the input move does not have a [board::Source::Square] as the source.
    pub fn get_minimum_move(&self, turn: &Turn) -> Turn {
        self.board.get_minimum_move(turn)
    }
    /// Returns an immutable reference to the board
    pub fn board(&self) -> &ChessBoard {
        &self.board
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
/// Enum that specifies the orientation of the board when displayed
pub enum RotateBoard {
    /// White prints the board with `a1` in the bottom left
    White,
    /// Black prints the board with `h8` in the bottom left
    Black,
    /// Rotate prints the board with `a1` or `h8` depending on whose turn it is
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

/// builder struct for setting configuration on a ChessGame
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
    /// initializes a ChessGameBuilder
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the rotate_board field with the given [RotateBoard] input
    ///
    /// # Default
    ///
    /// [RotateBoard::White]
    pub fn rotate_board(&mut self, val: RotateBoard) -> &mut Self {
        self.rotate_board = val;
        self
    }
    /// Sets the allow_undo flag
    ///
    /// # Default
    ///
    /// `false`
    pub fn allow_undo(&mut self, val: bool) -> &mut Self {
        self.allow_undo = val;
        self
    }
    /// Sets the player names, white first, then black
    ///
    /// # Default
    ///
    /// ("White", "Black")
    pub fn players(&mut self, names: (String, String)) -> &mut Self {
        self.players = names;
        self
    }
    /// Sets the enforce_flags field
    ///
    /// # Default
    ///
    /// `true`
    pub fn enforce_flags(&mut self, val: bool) -> &mut Self {
        self.enforce_flags = val;
        self
    }
    /// Builds a [ChessGame] with the specified configuration data
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
