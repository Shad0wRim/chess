use std::error::Error;
use std::fmt::Display;

use crate::board::{Line, Source, Square};
use crate::pieces::PieceType;
use crate::turn::{CastlingType, Move, Turn};

#[derive(Debug)]
pub struct ChessParseError {
    pub character: char,
    pub kind: ParseErrorKind,
}
impl Display for ChessParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid input string due to: {}", self.character)
    }
}
impl Error for ChessParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

#[derive(Debug)]
pub enum ParseErrorKind {
    InvalidChars,
    ExcessPieces,
    ExcessSquares,
    NeedSquare,
    PromotionError(PromotionError),
    ConversionError(ConversionError),
}

impl Error for ParseErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidChars => None,
            Self::ExcessPieces => None,
            Self::ExcessSquares => None,
            Self::NeedSquare => None,
            Self::ConversionError(e) => Some(e),
            Self::PromotionError(e) => Some(e),
        }
    }
}
impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChars => write!(f, "Invalid input character"),
            Self::ExcessPieces => write!(f, "Can only have one piece (uppercase letter) per move"),
            Self::ExcessSquares => write!(f, "Can only have a one destination and one source"),
            Self::NeedSquare => write!(f, "Need to specify a destination square such as 'e4'"),
            Self::PromotionError(_) => write!(f, "Invalid promotion specified"),
            Self::ConversionError(_) => write!(f, "Couldn't convert the string into a valid move"),
        }
    }
}

#[derive(Debug)]
pub enum PromotionError {
    Must,
    Cant,
    Invalid(PieceType),
}
impl Error for PromotionError {}
impl Display for PromotionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Must => write!(f, "Must promote when reaching the final rank"),
            Self::Cant => write!(f, "Can't promote until reaching the final rank"),
            Self::Invalid(pc) => write!(f, "Cannot promote a pawn into a {}", pc),
        }
    }
}

#[derive(Debug)]
pub struct ConversionError {
    pub input: String,
    pub target: String,
}
impl Error for ConversionError {}
impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot convert {0} into {1}", self.input, self.target)
    }
}

pub fn parse_move(input: &str) -> Result<Turn, ChessParseError> {
    if !input
        .chars()
        .all(|c| "abcdefgh12345678+#x=-O0KQRBN".contains(c))
    {
        return Err(ChessParseError {
            character: input
                .chars()
                .find(|&c| !"abcdefgh12345678+#x=-O0KQRBN".contains(c))
                .expect("Invalid character from previous check"),
            kind: ParseErrorKind::InvalidChars,
        });
    }

    if let Some(castling) = parse_castling(input) {
        return Ok(Turn::Castling(castling, get_flags(input)));
    }

    let mut piece = get_piece(input)?;
    let (dst, src) = get_squares(input)?;
    let flags = get_flags(input);
    let promotion;
    if input.contains('=') {
        promotion = Some(piece);
        piece = PieceType::Pawn;
    } else {
        promotion = None;
    };
    let r#move = Move {
        piece,
        dst,
        flags,
        src,
        promotion,
    };
    verify_move(r#move).map_err(|e| ChessParseError {
        character: '=',
        kind: ParseErrorKind::PromotionError(e),
    })
}

fn verify_move(r#move: Move) -> Result<Turn, PromotionError> {
    if let Some(piece) = r#move.promotion {
        if let PieceType::King | PieceType::Pawn = piece {
            return Err(PromotionError::Invalid(piece));
        }

        match r#move.dst.rank() {
            Line::Rank1 | Line::Rank8 => Ok(Turn::Move(r#move)),
            _ => Err(PromotionError::Cant),
        }
    } else {
        match (r#move.piece, r#move.dst.rank()) {
            (PieceType::Pawn, Line::Rank1 | Line::Rank8) => Err(PromotionError::Must),
            _ => Ok(Turn::Move(r#move)),
        }
    }
}

fn get_piece(turn: &str) -> Result<PieceType, ChessParseError> {
    let piece_options: Vec<char> = turn.chars().filter(|&c| c.is_uppercase()).collect();
    if piece_options.len() > 1 {
        Err(ChessParseError {
            character: piece_options[1],
            kind: ParseErrorKind::ExcessPieces,
        })
    } else {
        (*piece_options.first().unwrap_or(&'P'))
            .try_into()
            .map_err(|e| ChessParseError {
                character: *piece_options.first().unwrap_or(&'P'),
                kind: ParseErrorKind::ConversionError(e),
            })
    }
}

fn get_squares(turn: &str) -> Result<(Square, Option<Source>), ChessParseError> {
    let square_chars = turn
        .chars()
        .filter(|&x| "abcdefgh12345678".contains(x))
        .collect::<String>();
    match square_chars.chars().count() {
        0 | 1 => Err(ChessParseError {
            character: ' ',
            kind: ParseErrorKind::NeedSquare,
        }),
        2 => Ok((
            (square_chars[..]).parse().map_err(|e| ChessParseError {
                character: square_chars[0..=0].chars().next().unwrap(),
                kind: ParseErrorKind::ConversionError(e),
            })?,
            None,
        )),
        3 => Ok((
            square_chars[1..3].parse().map_err(|e| ChessParseError {
                character: square_chars[1..=1].chars().next().unwrap(),
                kind: ParseErrorKind::ConversionError(e),
            })?,
            Some(Source::Line(
                Line::new(square_chars.chars().next().expect("string has characters"))
                    .expect("should be valid rank or file character"),
            )),
        )),
        4 => Ok((
            square_chars[2..4].parse().map_err(|e| ChessParseError {
                character: square_chars[2..=2].chars().next().unwrap(),
                kind: ParseErrorKind::ConversionError(e),
            })?,
            Some(Source::Square(square_chars[0..2].parse().map_err(|e| {
                ChessParseError {
                    character: square_chars[0..=0].chars().next().unwrap(),
                    kind: ParseErrorKind::ConversionError(e),
                }
            })?)),
        )),
        _ => Err(ChessParseError {
            character: square_chars[4..=4].chars().next().unwrap(),
            kind: ParseErrorKind::ExcessSquares,
        }),
    }
}

fn get_flags(turn: &str) -> u8 {
    let mut flag = Flag::NONE;
    if turn.contains('x') {
        flag |= Flag::CAPTURE;
    }
    if turn.contains('#') {
        flag |= Flag::CHECKMATE;
    } else if turn.contains('+') {
        flag |= Flag::CHECK;
    }
    flag
}

fn parse_castling(turn: &str) -> Option<CastlingType> {
    if turn.contains("0-0-0") || turn.contains("O-O-O") {
        Some(CastlingType::Long)
    } else if turn.contains("0-0") || turn.contains("O-O") {
        Some(CastlingType::Short)
    } else {
        None
    }
}

#[allow(non_snake_case)]
pub mod Flag {
    pub const NONE: u8 = 0;
    pub const CHECK: u8 = 1 << 0;
    pub const CHECKMATE: u8 = 1 << 1;
    pub const CAPTURE: u8 = 1 << 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_pawn_move() {
        assert!(if let Ok(Turn::Move(Move {
            piece: PieceType::Pawn,
            dst: Square::E4,
            flags: Flag::NONE,
            src: None,
            promotion: None,
        })) = parse_move("e4")
        {
            true
        } else {
            false
        });
    }
    #[test]
    fn castling() {
        assert!(
            if let Ok(Turn::Castling(CastlingType::Short, Flag::NONE)) = parse_move("O-O") {
                true
            } else {
                false
            }
        );
        assert!(
            if let Ok(Turn::Castling(CastlingType::Long, Flag::NONE)) = parse_move("O-O-O") {
                true
            } else {
                false
            }
        );
    }
    #[test]
    fn simple_piece_move() {
        assert!(if let Ok(Turn::Move(Move {
            piece: PieceType::Queen,
            dst: Square::F3,
            flags: Flag::NONE,
            src: None,
            promotion: None,
        })) = parse_move("Qf3")
        {
            true
        } else {
            false
        })
    }
    #[test]
    fn capture() {
        assert!(if let Ok(Turn::Move(Move {
            piece: PieceType::Pawn,
            dst: Square::F5,
            flags: Flag::CAPTURE,
            src: Some(Source::Line(Line::FileE)),
            promotion: None,
        })) = parse_move("exf5")
        {
            true
        } else {
            false
        })
    }
    #[test]
    fn promotion() {
        assert!(if let Ok(Turn::Move(Move {
            piece: PieceType::Pawn,
            dst: Square::E8,
            flags: Flag::NONE,
            src: None,
            promotion: Some(PieceType::Queen),
        })) = parse_move("e8=Q")
        {
            true
        } else {
            false
        })
    }

    #[test]
    fn capture_into_promotion_checkmate() {
        assert!(if let Ok(Turn::Move(Move {
            piece: PieceType::Pawn,
            dst: Square::F8,
            flags: 6,
            src: Some(Source::Line(Line::FileE)),
            promotion: Some(PieceType::Rook),
        })) = parse_move("exf8=R#")
        {
            true
        } else {
            false
        })
    }

    #[test]
    fn castling_check() {
        assert!(
            if let Ok(Turn::Castling(CastlingType::Long, Flag::CHECK)) = parse_move("0-0-0+") {
                true
            } else {
                false
            }
        )
    }

    #[test]
    fn doesnt_promote() {
        assert!(parse_move("f1").is_err());
        assert!(parse_move("a8").is_err());
    }

    #[test]
    fn invalid_input() {
        assert!(parse_move("sljfelk0932").is_err());
        assert!(parse_move("Qxe1=R").is_err());
        assert!(parse_move("*").is_err());
        assert!(parse_move("abcde12312").is_err());
    }
}
