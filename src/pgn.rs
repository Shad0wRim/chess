use crate::Turn;
use std::collections::HashMap;

/// Takes in a pgn string and returns the game data
pub fn read_pgn(pgn_string: &str) -> (HashMap<String, String>, Vec<Turn>) {
    let (info, moves) = split_pgn_string(pgn_string);
    println!("{info}");
    println!("{moves}");
    (parse_pgn_info(&info), parse_pgn_moves(&moves))
}

/// Takes in a list of pgns separated by empty lines and splits them into their respective data
pub fn read_pgn_list(pgn_list_string: &str) -> Vec<(HashMap<String, String>, Vec<Turn>)> {
    split_pgn_list(pgn_list_string)
        .into_iter()
        .map(|pgn_string| read_pgn(&pgn_string))
        .collect()
}

/// Takes in a pgn string and returns the game result that is appended to the end of the move
/// sequence
pub fn get_game_result(pgn_string: &str) -> Option<&str> {
    pgn_string.split_whitespace().last()
}

fn split_pgn_list(pgn_list_string: &str) -> Vec<String> {
    pgn_list_string
        .split("\n\n")
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|items| items.join("\n"))
        .collect()
}

fn split_pgn_string(pgn_string: &str) -> (String, String) {
    let info = pgn_string
        .lines()
        .take_while(|&line| line.starts_with('['))
        .fold(String::new(), |s, l| s + "\n" + l);
    let moves = pgn_string
        .lines()
        .skip_while(|&line| line.starts_with('[') || line.is_empty())
        .fold(String::new(), |s, l| s + "\n" + l);
    (info, moves)
}

fn parse_pgn_info(info_string: &str) -> HashMap<String, String> {
    let mut info = HashMap::new();
    info_string
        .lines()
        .map(|line| {
            let field_name: String = line
                .chars()
                .skip_while(|&char| char != '[')
                .skip(1)
                .take_while(|&char| char != ' ')
                .collect();
            let field_value: String = line
                .chars()
                .skip_while(|&char| char != '"')
                .skip(1)
                .take_while(|&char| char != '"')
                .collect();
            (field_name, field_value)
        })
        .for_each(|(name, value)| {
            if name != "" {
                info.insert(name, value);
            }
        });
    info
}

fn parse_pgn_moves(moves_string: &str) -> Vec<Turn> {
    moves_string
        .lines()
        .skip_while(|line| line.starts_with('[') || line.is_empty())
        .fold(String::new(), |s, l| s + " " + l)
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
        .collect()
}

fn scan_between(
    open_delimiter: char,
    close_delimiter: char,
    keep: bool,
) -> impl Fn(&mut bool, char) -> Option<char> {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        board::{DrawType, Win, WinType},
        ChessGame, GameState,
    };

    #[test]
    fn pgn_list_read() -> Result<(), Box<dyn std::error::Error>> {
        let pgn_string_list = std::fs::read_to_string("res/pgn_list.pgn")?;

        for pgn_string in split_pgn_list(&pgn_string_list) {
            play_game(read_pgn(&pgn_string), get_game_result(&pgn_string))?
        }

        Ok(())
    }
    #[test]
    fn pgn_single_read() -> Result<(), Box<dyn std::error::Error>> {
        let pgn_string = std::fs::read_to_string("res/test.pgn")?;

        play_game(read_pgn(&pgn_string), get_game_result(&pgn_string))
    }

    fn play_game(
        (game_info, moves): (HashMap<String, String>, Vec<Turn>),
        pgn_last: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut game = ChessGame::default();
        let game_result = match game_info.get("Result") {
            Some(result) => result.to_string(),
            None => match pgn_last {
                Some(result) => result.to_string(),
                None => String::new(),
            },
        };
        //if let Some(white) = game_info.get("White") {
        //    game.players.0 = white.clone();
        //}
        //if let Some(black) = game_info.get("Black") {
        //    game.players.1 = black.clone();
        //}
        let move_read_result = (|| -> Result<GameState, Box<dyn std::error::Error>> {
            for r#move in moves {
                println!("{}", game.board_string());
                println!("{move}");
                game.make_move(&r#move)?;
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
                    _ => {
                        game.game_state = game_result;
                    }
                },
                _ => (),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
