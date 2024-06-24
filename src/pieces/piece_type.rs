use std::fmt;
use std::str::FromStr;

use crate::parser::ConversionError;
#[derive(Clone, Debug, Copy, PartialEq)]
#[allow(missing_docs)]
/// The type of a piece
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_letter = match self {
            Self::King => 'K',
            Self::Queen => 'Q',
            Self::Rook => 'R',
            Self::Bishop => 'B',
            Self::Knight => 'N',
            Self::Pawn => 'P',
        };
        write!(f, "{}", piece_letter)
    }
}
impl FromStr for PieceType {
    type Err = ConversionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "K" => Ok(Self::King),
            "Q" => Ok(Self::Queen),
            "R" => Ok(Self::Rook),
            "B" => Ok(Self::Bishop),
            "N" => Ok(Self::Knight),
            "P" => Ok(Self::Pawn),
            _ => Err(ConversionError {
                input: s.to_string(),
                target: "Piece".to_string(),
            }),
        }
    }
}
impl TryFrom<char> for PieceType {
    type Error = ConversionError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'R' => Ok(Self::Rook),
            'B' => Ok(Self::Bishop),
            'N' => Ok(Self::Knight),
            'P' => Ok(Self::Pawn),
            _ => Err(ConversionError {
                input: value.to_string(),
                target: "Piece".to_string(),
            }),
        }
    }
}
