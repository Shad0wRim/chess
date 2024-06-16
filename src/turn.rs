mod castling;
mod r#move;

use std::{fmt::Display, str::FromStr};

pub use castling::CastlingType;
pub use r#move::Move;

use crate::{
    board::{Source, Square},
    parser::{parse_move, Flag},
    pieces::{Piece, PieceType},
};

#[derive(Debug, Clone, PartialEq, Copy)]
/// The type of turn
pub enum Turn {
    /// A castling turn
    Castling(CastlingType, u8),
    /// A piece move turn
    Move(Move),
}
impl Turn {
    /// Creates a new turn from a piece, its location, and the destination square.
    pub fn new((loc, piece): (Square, Piece), dst: Square) -> Turn {
        Turn::Move(Move {
            piece: piece.piece,
            dst,
            flags: 0,
            src: Some(Source::Square(loc)),
            promotion: None,
        })
    }
}

impl FromStr for Turn {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_move(s).map_err(|_| "Failed to parse string into chess move")
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        match self {
            Turn::Castling(castling_type, flags) => {
                let base = match castling_type {
                    CastlingType::Long => "0-0-0",
                    CastlingType::Short => "0-0",
                };
                let check_or_checkmate = if flags & Flag::CHECKMATE != 0 {
                    "#"
                } else if flags & Flag::CHECK != 0 {
                    "+"
                } else {
                    ""
                };
                output.push_str(base);
                output.push_str(check_or_checkmate);
            }
            Turn::Move(Move {
                piece,
                dst,
                flags,
                src,
                promotion,
            }) => {
                let piece = match piece {
                    PieceType::Pawn => "".to_string(),
                    _ => piece.to_string(),
                };
                let source = match src {
                    Some(src) => src.to_string(),
                    None => "".to_string(),
                };
                let capture = flags & Flag::CAPTURE != 0;
                let promotion = match promotion {
                    Some(pc) => "=".to_string() + &pc.to_string(),
                    None => "".to_string(),
                };
                let check_or_checkmate = if flags & Flag::CHECKMATE != 0 {
                    "#"
                } else if flags & Flag::CHECK != 0 {
                    "+"
                } else {
                    ""
                };

                output.push_str(&piece);
                output.push_str(&source);
                if capture {
                    output.push('x');
                }
                output.push_str(&dst.to_string());
                output.push_str(&promotion);
                output.push_str(check_or_checkmate);
            }
        }

        write!(f, "{}", output)
    }
}
