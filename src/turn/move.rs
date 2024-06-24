use crate::board::{Source, Square};
use crate::pieces::PieceType;

#[derive(Debug, Clone, PartialEq, Copy)]
/// The information specifiying a piece move
pub struct Move {
    /// The type of piece that is moving
    pub piece: PieceType,
    /// The destination square of the move
    pub dst: Square,
    /// The capture/check/checkmate flags for the move
    pub flags: u8,
    /// The source of the piece for disambiguation
    pub src: Option<Source>,
    /// The piece that a pawn promotes to
    pub promotion: Option<PieceType>,
}
