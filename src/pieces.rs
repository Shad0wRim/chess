mod piece_type;
pub use piece_type::PieceType;

use std::fmt;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Piece {
    pub piece: PieceType,
    pub is_white: bool,
}

impl Piece {
    pub fn new(piece: PieceType, is_white: bool) -> Piece {
        Piece { piece, is_white }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chess_sym = match self {
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
        };
        write!(f, "{}", chess_sym)
    }
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
