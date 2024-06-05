use std::fmt::Display;

use super::line::Line;
use super::square::Square;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Source {
   Line(Line),
   Square(Square),
}

impl Display for Source {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         Source::Line(line) => write!(f, "{}", line),
         Source::Square(square) => write!(f, "{}", square),
      }
   }
}
