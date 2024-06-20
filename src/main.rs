//! Local chess game in the terminal
use chess::utils::print_all_errors;
use chess::*;
use std::fs;

fn main() {
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
                    if let Ok(_) = game.undo_move() {
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

            let turn = parse_move(buf.trim())?;
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
