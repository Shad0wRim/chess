//! Local chess game in the terminal
mod tui;

use chess::{
    board::{DrawType, GameState, Square, Win, WinType},
    pieces::Piece,
    turn::Turn,
    utils::all_errors_string,
    ChessGame,
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::io;
use tui::Tui;

fn main() -> std::io::Result<()> {
    install_hook();
    let mut terminal = tui::init()?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    tui::restore()?;
    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

enum InputMode {
    Visual,
    Algebraic,
}
struct App {
    game: ChessGame,
    input: String,
    character_index: usize,
    board_location: Square,
    selected_piece: Option<(Square, Piece)>,
    input_mode: InputMode,
    error_messages: Vec<String>,
    stop: bool,
}

impl App {
    fn new() -> Self {
        Self {
            game: ChessGame::default(),
            input: String::new(),
            input_mode: InputMode::Visual,
            character_index: 0,
            board_location: Square::A1,
            selected_piece: None,
            error_messages: Vec::new(),
            stop: false,
        }
    }
    fn move_board_left(&mut self) {
        self.board_location = self.board_location.left().unwrap_or(self.board_location);
    }
    fn move_board_right(&mut self) {
        self.board_location = self.board_location.right().unwrap_or(self.board_location);
    }
    fn move_board_down(&mut self) {
        self.board_location = self.board_location.down().unwrap_or(self.board_location);
    }
    fn move_board_up(&mut self) {
        self.board_location = self.board_location.up().unwrap_or(self.board_location);
    }
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }
    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }
    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }
    fn byte_index(&mut self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }
    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }
    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }
    fn submit_message(&mut self) {
        self.error_messages.clear();
        self.handle_input();
        self.input.clear();
        self.reset_cursor();
    }
    fn handle_gamestate(&mut self) {
        match self.game.game_state {
            GameState::Continue => return,
            GameState::Win(win) => {
                let win_message = String::from(if !win.is_white {
                    &self.game.players.0
                } else {
                    &self.game.players.1
                }) + " wins by "
                    + match win.kind {
                        WinType::Checkmate => "checkmate",
                        WinType::Resign => "resignation",
                        WinType::Timeout => "timout",
                    };
                self.error_messages.clear();
                self.error_messages.push(win_message);
            }
            GameState::Draw(draw) => {
                let draw_message = String::from("The game is a draw by ")
                    + match draw {
                        DrawType::Stalemate => "stalemate",
                        DrawType::FiftyMove => "the fifty move rule",
                        DrawType::ThreefoldRepitition => "threefold repetition",
                        DrawType::InsufficientMaterial => "insufficient material",
                        DrawType::Offer => "draw offer",
                    };
                self.error_messages.clear();
                self.error_messages.push(draw_message);
            }
            GameState::Stop => {
                self.error_messages.clear();
                self.error_messages
                    .push(String::from("The game was aborted"));
            }
        }
        self.error_messages
            .push(String::from("Press any key to quit"));
        self.stop = true;
    }
    fn select_piece(&mut self) {
        let potential_piece = self.game.board().get(&self.board_location);
        self.selected_piece = match potential_piece {
            Some(pc) => Some((self.board_location, *pc)),
            None => None,
        };
    }
    fn move_piece(&mut self) {
        self.error_messages.clear();
        let Some(selected_piece) = self.selected_piece else {
            self.error_messages
                .push(String::from("There is no selected piece"));
            return;
        };
        let turn = Turn::new(selected_piece, self.board_location);
        let enforce_flags = self.game.enforce_flags;
        self.game.enforce_flags = false;
        self.handle_turn(turn);
        self.game.enforce_flags = enforce_flags;
    }
    fn handle_turn(&mut self, turn: Turn) {
        match self.game.make_move(&turn) {
            Ok(_) => {}
            Err(err) => {
                let split_errors = all_errors_string(&err)
                    .lines()
                    .map(|str| str.to_string())
                    .collect::<Vec<_>>();
                self.error_messages.extend_from_slice(&split_errors);
            }
        }
        self.handle_gamestate();
    }
    fn handle_input(&mut self) {
        match self.input.trim() {
            "undo" => match self.game.undo_move() {
                Some(_) => return,
                None => {
                    self.error_messages
                        .extend_from_slice(&["Undoing is not allowed".to_string()]);
                    return;
                }
            },
            "quit" => self.game.game_state = GameState::Stop,
            "resign" => {
                self.game.game_state = GameState::Win(Win {
                    is_white: self.game.is_white(),
                    kind: WinType::Resign,
                })
            }
            "draw" => self.game.game_state = GameState::Draw(DrawType::Offer),
            _ => (),
        }
        if self.game.game_state != GameState::Continue {
            self.handle_gamestate();
            return;
        }

        let turn = match self.input.parse::<Turn>() {
            Ok(turn) => turn,
            Err(err) => {
                let split_errors = all_errors_string(&err)
                    .lines()
                    .map(|str| str.to_string())
                    .collect::<Vec<_>>();
                self.error_messages.extend_from_slice(&split_errors);
                return;
            }
        };
        self.handle_turn(turn);
    }
}

fn install_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        tui::restore().unwrap();
        original_hook(panic);
    }));
}

fn run_app(terminal: &mut Tui, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Visual => match key.code {
                    KeyCode::Char('a') => {
                        app.input_mode = InputMode::Algebraic;
                    }
                    KeyCode::Char('h') | KeyCode::Left => app.move_board_left(),
                    KeyCode::Char('l') | KeyCode::Right => app.move_board_right(),
                    KeyCode::Char('j') | KeyCode::Down => app.move_board_down(),
                    KeyCode::Char('k') | KeyCode::Up => app.move_board_up(),
                    KeyCode::Char(' ') => app.select_piece(),
                    KeyCode::Enter => app.move_piece(),
                    _ => {}
                },
                InputMode::Algebraic if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => app.submit_message(),
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Visual;
                    }
                    _ => {}
                },
                InputMode::Algebraic => {}
            }
        }
        if app.stop {
            terminal.draw(|f| ui(f, &app))?;
            event::read()?;
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Length(12),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);
    let chunks = vertical.split(f.size());
    let board_area = chunks[0];
    let input_area = chunks[1];
    let bottom_layout = Layout::horizontal([Constraint::Min(0), Constraint::Length(20)]);
    let [error_area, move_area] = bottom_layout.areas(chunks[2]);

    let player_string = app.game.player_string();

    let mut all_square_strs = Square::iterator()
        .map(|sq| (sq, app.game.board().get(&sq)))
        .map(|(sq, pc)| {
            let pc_string = if let Some(pc) = pc {
                pc.to_string() + " "
            } else {
                "  ".to_string()
            };
            pc_string
                .bg(if sq.is_light() {
                    Color::White
                } else {
                    Color::LightGreen
                })
                .black()
        })
        .collect::<Vec<_>>()
        .chunks(8)
        .scan(9, |rank, chunk| {
            *rank -= 1;
            Some(Line::from_iter(
                [Span::from(rank.to_string() + " ")]
                    .into_iter()
                    .chain(chunk.to_vec().into_iter())
                    .chain([Span::raw("\n")]),
            ))
        })
        .chain([Line::from("  a b c d e f g h")])
        .collect::<Vec<_>>();
    all_square_strs.push(Line::from(player_string.as_str()));

    let (board, style): (Vec<Line<'_>>, _) = match app.input_mode {
        InputMode::Visual => (all_square_strs, Style::default().gray().on_black()),
        InputMode::Algebraic => (all_square_strs, Style::default()),
    };
    let board = Text::from(board).patch_style(style);
    let board = Paragraph::new(board).block(Block::bordered().style(match app.input_mode {
        InputMode::Visual => ratatui::style::Color::LightCyan,
        InputMode::Algebraic => ratatui::style::Color::default(),
    }));
    f.render_widget(board, board_area);

    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Visual => Style::default(),
            InputMode::Algebraic => Style::default().fg(ratatui::style::Color::Yellow),
        })
        .block(Block::bordered().title("Input"));
    f.render_widget(input, input_area);

    match app.input_mode {
        InputMode::Visual => {
            let y_offset = rank_offset(app);
            let x_offset = file_offset(app);
            f.set_cursor(board_area.x + 3 + x_offset * 2, board_area.y + 8 - y_offset);
        }
        InputMode::Algebraic => {
            f.set_cursor(
                input_area.x + app.character_index as u16 + 1,
                input_area.y + 1,
            );
        }
    }

    let error_messages: Vec<ListItem> = app
        .error_messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::raw(format!("{}: {m}", i + 1)));
            ListItem::new(content)
        })
        .collect();
    let error_messages = List::new(error_messages).block(Block::bordered().title("Error Messages"));
    f.render_widget(error_messages, error_area);

    let game_history: Vec<ListItem> = app
        .game
        .game_hist()
        .chunks(2)
        .scan(ChessGame::default(), |board, turns| {
            let mut turn_string = String::new();

            let turn1 = board.get_minimum_move(&turns[0]);
            board.make_move(&turn1).expect("History is always valid");

            turn_string += &turn1.to_string();

            if let Some(turn2) = turns.get(1) {
                let turn2 = board.get_minimum_move(turn2);
                board.make_move(&turn2).expect("History is always valid");

                turn_string.push(' ');
                turn_string += &turn2.to_string();
            }

            Some(turn_string)
        })
        .enumerate()
        .map(|(i, t)| {
            let content = Line::from(Span::raw(format!("{}: {}\n", i + 1, t)));
            ListItem::new(content)
        })
        .collect();
    let game_history = List::new(game_history).block(Block::bordered().title("Game History"));
    let mut list_state = ListState::default().with_selected(Some(usize::MAX));
    f.render_stateful_widget(game_history, move_area, &mut list_state);
}

fn file_offset(app: &App) -> u16 {
    match app.board_location.file() {
        chess::board::Line::FileA => 0,
        chess::board::Line::FileB => 1,
        chess::board::Line::FileC => 2,
        chess::board::Line::FileD => 3,
        chess::board::Line::FileE => 4,
        chess::board::Line::FileF => 5,
        chess::board::Line::FileG => 6,
        chess::board::Line::FileH => 7,
        _ => unreachable!(),
    }
}

fn rank_offset(app: &App) -> u16 {
    match app.board_location.rank() {
        chess::board::Line::Rank1 => 0,
        chess::board::Line::Rank2 => 1,
        chess::board::Line::Rank3 => 2,
        chess::board::Line::Rank4 => 3,
        chess::board::Line::Rank5 => 4,
        chess::board::Line::Rank6 => 5,
        chess::board::Line::Rank7 => 6,
        chess::board::Line::Rank8 => 7,
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
mod basic {
    use chess::utils::print_all_errors;
    use chess::{board::*, turn::*, *};
    use std::fs;

    pub fn main_play_game() {
        // let mut buf = String::new();
        // println!("Whose perspective? (W, B, R)");
        // io::stdin().read_line(&mut buf).unwrap();
        // let rotate_option = match buf.to_lowercase().trim() {
        //     "w" => RotateBoard::White,
        //     "b" => RotateBoard::Black,
        //     "r" => RotateBoard::Rotate,
        //     _ => RotateBoard::White,
        // };

        let mut game = ChessGame::builder()
            .rotate_board(RotateBoard::White)
            .allow_undo(true)
            // .rotate_board(rotate_option)
            .players(("White".to_owned(), "Black".to_owned()))
            .enforce_flags(true)
            .build();
        let _pgn_string = fs::read_to_string("res/pgn.pgn").expect("Valid file");

        // play_from_pgn(&mut game, _pgn_string);
        // game.reset();
        play_game(&mut game);

        println!("{}\n{}", game.gen_pgn(), game.gen_fen());
    }
    /// plays a full game of chess on a local machine, swapping between players
    pub fn play_game(game: &mut ChessGame) {
        game.display();
        if game.game_state != GameState::Continue {
            game.display_end_message();
            return;
        }

        let mut buf = String::new();
        loop {
            let outcome: Result<GameState, Box<dyn std::error::Error>> = (|| {
                buf.clear();
                std::io::stdin().read_line(&mut buf)?;

                match buf.to_lowercase().trim() {
                    "u" => {
                        if game.undo_move().is_some() {
                            return Ok(GameState::Continue);
                        } else {
                            return Err("Undoing moves is not allowed".into());
                        }
                    }
                    "q" => return Ok(GameState::Stop),
                    "resign" => {
                        return Ok(GameState::Win(Win {
                            is_white: !game.is_white(),
                            kind: WinType::Resign,
                        }))
                    }
                    "draw" => {
                        println!("Accept draw offer? (y/n)");
                        loop {
                            buf.clear();
                            std::io::stdin().read_line(&mut buf)?;
                            if buf.to_lowercase().trim() == "y" {
                                return Ok(GameState::Draw(DrawType::Offer));
                            } else if buf.to_lowercase().trim() == "n" {
                                return Ok(GameState::Continue);
                            } else {
                                println!("Enter `y` or `n`");
                                continue;
                            }
                        }
                    }
                    _ => (),
                }

                let turn = buf.trim().parse::<Turn>()?;
                game.make_move(&turn)?;
                Ok(GameState::Continue)
            })();
            match outcome {
                Ok(state) => match state {
                    end @ (GameState::Win(_) | GameState::Draw(_)) => game.game_state = end,
                    GameState::Stop => {
                        println!("Aborting game");
                        return;
                    }
                    GameState::Continue => (),
                },
                Err(e) => {
                    print_all_errors(e.as_ref());
                    continue;
                }
            }

            game.display();

            if game.game_state == GameState::Continue {
                continue;
            } else {
                game.display_end_message();
                return;
            }
        }
    }
    /// plays a game of chess from a pgn string, progressing when \[Enter\] is pressed
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

        let pgn_read_result = (|| -> Result<GameState, Box<dyn std::error::Error>> {
            for r#move in moves {
                std::io::stdin().read_line(&mut buf)?;
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
            Ok(game_result) => match game.game_state {
                GameState::Continue => match game_result {
                    GameState::Continue => play_game(game),
                    _ => {
                        game.game_state = game_result;
                        game.display_end_message();
                    }
                },
                _ => game.display_end_message(),
            },
            Err(e) => {
                print_all_errors(e.as_ref());
            }
        }
    }
}
