use std::collections::HashMap;

use super::square::Square;
use super::{CastlingChecks, ChessBoard};
use crate::pieces::{Piece, PieceType};

impl Default for ChessBoard {
    fn default() -> Self {
        let board_array = [
            (Square::A1, Piece::new(PieceType::Rook, true)),
            (Square::B1, Piece::new(PieceType::Knight, true)),
            (Square::C1, Piece::new(PieceType::Bishop, true)),
            (Square::D1, Piece::new(PieceType::Queen, true)),
            (Square::E1, Piece::new(PieceType::King, true)),
            (Square::F1, Piece::new(PieceType::Bishop, true)),
            (Square::G1, Piece::new(PieceType::Knight, true)),
            (Square::H1, Piece::new(PieceType::Rook, true)),
            (Square::A8, Piece::new(PieceType::Rook, false)),
            (Square::B8, Piece::new(PieceType::Knight, false)),
            (Square::C8, Piece::new(PieceType::Bishop, false)),
            (Square::D8, Piece::new(PieceType::Queen, false)),
            (Square::E8, Piece::new(PieceType::King, false)),
            (Square::F8, Piece::new(PieceType::Bishop, false)),
            (Square::G8, Piece::new(PieceType::Knight, false)),
            (Square::H8, Piece::new(PieceType::Rook, false)),
            (Square::A2, Piece::new(PieceType::Pawn, true)),
            (Square::B2, Piece::new(PieceType::Pawn, true)),
            (Square::C2, Piece::new(PieceType::Pawn, true)),
            (Square::D2, Piece::new(PieceType::Pawn, true)),
            (Square::E2, Piece::new(PieceType::Pawn, true)),
            (Square::F2, Piece::new(PieceType::Pawn, true)),
            (Square::G2, Piece::new(PieceType::Pawn, true)),
            (Square::H2, Piece::new(PieceType::Pawn, true)),
            (Square::A7, Piece::new(PieceType::Pawn, false)),
            (Square::B7, Piece::new(PieceType::Pawn, false)),
            (Square::C7, Piece::new(PieceType::Pawn, false)),
            (Square::D7, Piece::new(PieceType::Pawn, false)),
            (Square::E7, Piece::new(PieceType::Pawn, false)),
            (Square::F7, Piece::new(PieceType::Pawn, false)),
            (Square::G7, Piece::new(PieceType::Pawn, false)),
            (Square::H7, Piece::new(PieceType::Pawn, false)),
        ];
        let board = HashMap::from(board_array);
        ChessBoard {
            board,
            is_white: true,
            castling: CastlingChecks::default(),
            en_passant: None,
            fifty_move: 50,
        }
    }
}
