use crate::board::{Source, Square};
use crate::pieces::PieceType;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Move {
    pub piece: PieceType,
    pub dst: Square,
    pub flags: u8,
    pub src: Option<Source>,
    pub promotion: Option<PieceType>,
}
