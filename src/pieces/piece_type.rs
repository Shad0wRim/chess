use std::str::FromStr;

use crate::parser::ConversionError;
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum PieceType {
   King,
   Queen,
   Rook,
   Bishop,
   Knight,
   Pawn,
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
         _ => Err(ConversionError { input: s.to_string(), target: "Piece".to_string() }),
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
         _ => Err(ConversionError { input: value.to_string(), target: "Piece".to_string() }),
      }
   }
}
