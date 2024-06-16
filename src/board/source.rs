use std::fmt::Display;

use super::line::Line;
use super::square::Square;

#[derive(Debug, Clone, PartialEq, Copy)]
/// The type of source square used for disambiguation
pub enum Source {
    /// Line source, which can be a file or rank
    Line(Line),
    /// Square source
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
