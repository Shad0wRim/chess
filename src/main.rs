//! Local chess game in the terminal
mod tui;

use chess::{
    board::{DrawType, GameState, Square, Win, WinType},
    pieces::Piece,
    turn::Turn,
    utils::all_errors_string,
    ChessGame,
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseButton};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use std::io;
use tui::Tui;

fn main() -> std::io::Result<()> {
    //tui()
    basic::main_play_game();
    Ok(())
}

fn tui() -> std::io::Result<()> {
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

#[derive(PartialEq)]
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
    messages: Vec<String>,
    last_input_was_keyboard: bool,
    saved_location: Square,
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
            messages: Vec::new(),
            last_input_was_keyboard: true,
            saved_location: Square::A1,
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
        self.messages.clear();
        self.handle_input();
        self.input.clear();
        self.reset_cursor();
    }
    fn handle_gamestate(&mut self) {
        match self.game.game_state {
            GameState::Continue => return,
            GameState::Win(win) => {
                let win_message = String::from(if !win.is_white {
                    self.game
                        .game_info
                        .get("White")
                        .map_or("White", |x| x.as_ref())
                } else {
                    self.game
                        .game_info
                        .get("Black")
                        .map_or("Black", |x| x.as_ref())
                }) + " wins by "
                    + match win.kind {
                        WinType::Checkmate => "checkmate",
                        WinType::Resign => "resignation",
                        WinType::Timeout => "timout",
                    };
                self.messages.clear();
                self.messages.push(win_message);
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
                self.messages.clear();
                self.messages.push(draw_message);
            }
            GameState::Stop => {
                self.messages.clear();
                self.messages.push(String::from("The game was aborted"));
            }
        }
        self.messages.push(String::from("Press any key to quit"));
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
        self.messages.clear();
        let Some(selected_piece) = self.selected_piece else {
            self.messages
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
            Ok(_) => self.selected_piece = None,
            Err(err) => {
                let split_errors = all_errors_string(&err)
                    .lines()
                    .map(|str| str.to_string())
                    .collect::<Vec<_>>();
                self.messages.extend_from_slice(&split_errors);
            }
        }
        self.handle_gamestate();
    }
    fn handle_input(&mut self) {
        match self.input.trim() {
            "undo" => match self.game.undo_move() {
                Some(_) => return,
                None => {
                    self.messages
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
                self.messages.extend_from_slice(&split_errors);
                return;
            }
        };
        self.handle_turn(turn);
    }
    fn handle_mouse(&mut self, row: u16, col: u16) {
        self.board_location = Square::iterator()
            .map(|sq| (sq, square_to_location(sq)))
            .fold(
                (Square::A1, 1000_i16),
                |(closest_sq, dist), (next_sq, (x, y))| {
                    let next_dist = (x as i16 - col as i16).pow(2) + (y as i16 - row as i16).pow(2);
                    if next_dist < dist {
                        (next_sq, next_dist)
                    } else {
                        (closest_sq, dist)
                    }
                },
            )
            .0;
    }
    fn select_or_move(&mut self) {
        let potential_piece = self.game.board().get(&self.board_location);
        match potential_piece {
            Some(Piece { is_white, .. }) if *is_white == self.game.is_white() => {
                self.select_piece()
            }
            Some(_) | None => self.move_piece(),
        }
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

        match event::read()? {
            Event::Key(key) => {
                app.last_input_was_keyboard = true;
                match app.input_mode {
                    InputMode::Visual => match key.code {
                        KeyCode::Char('a') => {
                            app.selected_piece = None;
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
            Event::Mouse(mouse) if app.input_mode == InputMode::Visual => match mouse.kind {
                event::MouseEventKind::Down(MouseButton::Left) => {
                    app.last_input_was_keyboard = false;
                    app.handle_mouse(mouse.row, mouse.column);
                    app.saved_location = app.board_location;
                    app.select_or_move();
                }
                event::MouseEventKind::Up(MouseButton::Left)
                    if app.saved_location != app.board_location =>
                {
                    app.move_piece();
                }
                event::MouseEventKind::Drag(MouseButton::Left) => {
                    app.handle_mouse(mouse.row, mouse.column)
                }
                _ => {}
            },
            _ => {}
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

    let top_layout = Layout::horizontal([Constraint::Length(20), Constraint::Min(0)]);

    let [board_area, info_area] = top_layout.areas(chunks[0]);

    let input_area = chunks[1];

    let bottom_layout = Layout::horizontal([Constraint::Min(0), Constraint::Length(20)]);
    let [error_area, move_area] = bottom_layout.areas(chunks[2]);

    render_board(app, f, board_area);

    render_info(app, f, info_area);

    render_input(app, f, input_area);

    render_messages(app, f, error_area);

    render_history(app, f, move_area);
}

fn render_history(app: &App, f: &mut Frame, move_area: ratatui::prelude::Rect) {
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
    let game_history = List::new(game_history).block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .title_bottom("Game History"),
    );
    let mut list_state = ListState::default().with_selected(Some(usize::MAX));
    f.render_stateful_widget(game_history, move_area, &mut list_state);
}

fn render_messages(app: &App, f: &mut Frame, error_area: ratatui::prelude::Rect) {
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::raw(format!("{}: {m}", i + 1)));
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages).block(
        Block::default()
            .borders(Borders::LEFT | Borders::BOTTOM)
            .title_bottom("Messages"),
    );
    f.render_widget(messages, error_area);
}

fn render_info(app: &App, f: &mut Frame, info_area: ratatui::prelude::Rect) {
    let info = match app.input_mode {
        InputMode::Visual => Paragraph::new(vec![
            "Press `a` to enter command mode".into(),
            "Move cursor with hjkl or arrows".into(),
            "Press `space` to select and `enter` to move".into(),
            "Use the mouse to select and move pieces".into(),
        ]),
        InputMode::Algebraic => Paragraph::new(vec![
            "Press `esc` to enter visual mode".into(),
            "`quit` `resign` `draw` to end the game".into(),
            "Enter a move in algebraic chess notation to make a move with commands".into(),
        ]),
    }
    .block(
        Block::default()
            .borders(Borders::TOP | Borders::RIGHT)
            .title("Info"),
    )
    .wrap(Wrap { trim: true });

    f.render_widget(info, info_area);
}

fn render_input(app: &App, f: &mut Frame, input_area: ratatui::prelude::Rect) {
    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Visual => Style::default(),
            InputMode::Algebraic => Style::default().fg(ratatui::style::Color::Yellow),
        })
        .block(Block::bordered().title("Input"));

    if let InputMode::Algebraic = app.input_mode {
        f.set_cursor(
            input_area.x + app.character_index as u16 + 1,
            input_area.y + 1,
        );
    }

    f.render_widget(input, input_area);
}

fn render_board(app: &App, f: &mut Frame, board_area: ratatui::prelude::Rect) {
    let player_string = app.game.player_string();

    let mut all_square_strs = Square::iterator()
        .map(|sq| (sq, app.game.board().get(&sq)))
        .map(|(sq, pc)| {
            let pc_string = if let Some(pc) = pc {
                pc.to_string() + " "
            } else {
                "  ".to_string()
            };
            if let Some((selected_square, _)) = app.selected_piece {
                if selected_square == sq {
                    pc_string.bg(Color::LightYellow)
                } else {
                    pc_string.bg(if sq.is_light() {
                        Color::White
                    } else {
                        Color::LightGreen
                    })
                }
            } else {
                pc_string.bg(if sq.is_light() {
                    Color::White
                } else {
                    Color::LightGreen
                })
            }
            .black()
        })
        .collect::<Vec<_>>()
        .chunks(8)
        .scan(9, |rank, chunk| {
            *rank -= 1;
            Some(Line::from_iter(
                [Span::from(rank.to_string() + " ")]
                    .into_iter()
                    .chain(chunk.to_vec())
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

    if let InputMode::Visual = app.input_mode {
        if app.last_input_was_keyboard {
            let (x, y) = square_to_location(app.board_location);
            f.set_cursor(board_area.x + x, board_area.y + y);
        }
    }

    f.render_widget(board, board_area);
}

fn square_to_location(sq: Square) -> (u16, u16) {
    fn file_offset(sq: Square) -> u16 {
        match sq.file() {
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
    fn rank_offset(sq: Square) -> u16 {
        match sq.rank() {
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

    let x = 3 + file_offset(sq) * 2;
    let y = 8 - rank_offset(sq);

    (x, y)
}

#[allow(dead_code)]
mod basic {
    use chess::utils::print_all_errors;
    use chess::{board::*, turn::*, *};
    use itertools::Itertools;
    use pgn::read_pgn;
    use std::io::BufRead;
    use std::{fs, io};

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

        //let mut game = ChessGame::builder()
        //    .rotate_board(RotateBoard::White)
        //    .allow_undo(true)
        //    // .rotate_board(rotate_option)
        //    .players(("White".to_owned(), "Black".to_owned()))
        //    .enforce_flags(true)
        //    .build();
        //let _pgn_string = fs::read_to_string("res/pgn.pgn").expect("Valid file");
        let pgn_iter = io::BufReader::new(
            fs::File::open("/mnt/c/Users/jungo/Downloads/sicilian.pgn").expect("Valid file"),
        )
        .lines()
        .map_while(Result::ok)
        .chunk_by(|line| line.starts_with('['));
        let pgn_iter = pgn_iter
            .into_iter()
            .map(|(_, mut lines)| lines.join("\n"))
            .chunks(2);
        let pgn_iter = pgn_iter.into_iter().map(|mut lines| lines.join("\n"));

        for pgn_string in pgn_iter {
            play_from_pgn(&mut ChessGame::default(), pgn_string);
        }

        //play_from_pgn(&mut game, _pgn_string);
        //game.reset();
        //play_game(&mut game);

        //println!("{}\n{}", game.gen_pgn(), game.gen_fen());
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
    /// plays a game of chess from a pgn string, progressing when <Enter> is pressed
    pub fn play_from_pgn(game: &mut ChessGame, pgn_string: String) {
        let (game_info, moves) = pgn::read_pgn(&pgn_string);
        let game_result = match game_info.get("Result") {
            Some(result) => result.to_string(),
            None => match pgn::get_game_result(&pgn_string) {
                Some(result) => result.to_string(),
                None => String::new(),
            },
        };
        if let Some(white) = game_info.get("White") {
            game.game_info.insert(String::from("White"), white.clone());
        }
        if let Some(black) = game_info.get("Black") {
            game.game_info.insert(String::from("Black"), black.clone());
        }

        game.display();

        let move_read_result = (|| -> Result<GameState, Box<dyn std::error::Error>> {
            for r#move in moves {
                //std::io::stdin().read_line(&mut String::new())?;
                game.make_move(&r#move)?;
                game.display();
            }

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

        match move_read_result {
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
