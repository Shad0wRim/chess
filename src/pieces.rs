mod piece_type;
pub use piece_type::PieceType;

use std::fmt;

#[derive(Clone, Debug, PartialEq, Copy)]
/// A chess piece
pub struct Piece {
    /// The type of piece
    pub piece: PieceType,
    /// Whether the piece is owned by the white player or not
    pub is_white: bool,
}

impl Piece {
    /// Creates a new chess piece from its type and its owner
    pub fn new(piece: PieceType, is_white: bool) -> Piece {
        Piece { piece, is_white }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chess_sym = if !f.alternate() {
            match self {
                Self {
                    piece: PieceType::King,
                    is_white: true,
                    ..
                } => '\u{2654}',
                Self {
                    piece: PieceType::Queen,
                    is_white: true,
                    ..
                } => '\u{2655}',
                Self {
                    piece: PieceType::Rook,
                    is_white: true,
                    ..
                } => '\u{2656}',
                Self {
                    piece: PieceType::Bishop,
                    is_white: true,
                    ..
                } => '\u{2657}',
                Self {
                    piece: PieceType::Knight,
                    is_white: true,
                    ..
                } => '\u{2658}',
                Self {
                    piece: PieceType::Pawn,
                    is_white: true,
                    ..
                } => '\u{2659}',
                Self {
                    piece: PieceType::King,
                    is_white: false,
                    ..
                } => '\u{265A}',
                Self {
                    piece: PieceType::Queen,
                    is_white: false,
                    ..
                } => '\u{265B}',
                Self {
                    piece: PieceType::Rook,
                    is_white: false,
                    ..
                } => '\u{265C}',
                Self {
                    piece: PieceType::Bishop,
                    is_white: false,
                    ..
                } => '\u{265D}',
                Self {
                    piece: PieceType::Knight,
                    is_white: false,
                    ..
                } => '\u{265E}',
                Self {
                    piece: PieceType::Pawn,
                    is_white: false,
                    ..
                } => '\u{265F}',
            }
        } else if self.is_white {
            self.piece.to_string().chars().next().unwrap()
        } else {
            self.piece
                .to_string()
                .chars()
                .find_map(|c| c.to_lowercase().next())
                .unwrap()
        };
        write!(f, "{}", chess_sym)
    }
}
