mod default;
mod line;
mod source;
mod square;
pub use line::Line;
pub use source::Source;
pub use square::Square;

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use crate::parser::Flag;
use crate::pieces::{Piece, PieceType};
use crate::turn::{CastlingType, Move, Turn};
use crate::utils::Counter;

#[derive(Debug)]
/// Ways that a turn can be incorrect
pub enum TurnError {
    /// No piece can move to the specified destination
    NoTarget,
    /// Need a line disambiguation to determine which piece moves
    NeedLine,
    /// Need a file disambiguation to determine which piece moves
    NeedFile,
    /// Need a square disambiguation to determine which piece moves
    NeedSquare,
    /// A disambiguation was provided that was overspecific and unneeded
    OverSpecification,
    /// There is no eligible piece at the specified source square
    MissingAtSquare,
    /// There is no eligible piece at the specified source line
    MissingInLine,
    /// There are multiple eligible pieces in the specified source line
    BothInLine,
    /// The move specified leaves the king in check
    KingInCheck,
    /// Cannot castle due to losing the rights
    CastleLostRights,
    /// Cannot castle due to pieces being in the way
    CastlePathBlocked,
    /// Cannot castle due to opposing pieces targetting the castling squares
    CastleThroughCheck,
    /// Need to add the `+` flag for a check
    NeedCheckSpecifier,
    /// Need to add the `#` flag for a checkmate
    NeedCheckmateSpecifier,
    /// Need to add the `x` flag for a capture
    NeedCaptureSpecifier,
    /// Need to remove the `#` flag for a non-checkmate
    RemoveCheckmateSpecifier,
    /// Need to remove the `+` flag for a non-check
    RemoveCheckSpecifier,
    /// Need to remove the `x` flag for a non-capture
    RemoveCaptureSpecifier,
}

impl Error for TurnError {}
impl Display for TurnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TurnError::NoTarget => write!(f, "That piece can't move to that square"),
            TurnError::NeedLine => write!(f, "Need to specify the line that the piece comes from"),
            TurnError::NeedFile => write!(f, "Need to specify the file the pawn captures from"),
            TurnError::NeedSquare => write!(f, "Need to specify the square the piece comes from"),
            TurnError::OverSpecification => write!(f, "Provided unneeded line information"),
            TurnError::MissingAtSquare => write!(
                f,
                "No piece that can move to the destination found at that square"
            ),
            TurnError::MissingInLine => write!(
                f,
                "No piece that can move to the destination found in that line"
            ),
            TurnError::BothInLine => write!(f, "Both potential pieces found in the line specified"),
            TurnError::KingInCheck => write!(f, "That move causes the king to be in check"),
            TurnError::CastleLostRights => write!(f, "Lost the right to castle"),
            TurnError::CastlePathBlocked => write!(f, "Can't castle becuase the path is blocked"),
            TurnError::CastleThroughCheck => {
                write!(f, "Can't castle because the king would move through check")
            }
            TurnError::NeedCheckSpecifier => write!(f, "Need to add a `+` when giving a check"),
            TurnError::NeedCheckmateSpecifier => {
                write!(f, "Need to add a `#` when giving checkmate")
            }
            TurnError::NeedCaptureSpecifier => {
                write!(f, "Need to add a `x` when capturing a piece")
            }
            TurnError::RemoveCheckmateSpecifier => {
                write!(f, "Remove `#` when not giving checkmate")
            }
            TurnError::RemoveCheckSpecifier => write!(f, "Remove `+` when capturing a piece"),
            TurnError::RemoveCaptureSpecifier => write!(f, "Remove `x` when not capturing a piece"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Stores the current board state
///
/// Includes the piece locations, current turn, castling rights, en passant rights, half move
/// clock, and the full move number
pub struct ChessBoard {
    piece_locs: HashMap<Square, Piece>,
    is_white: bool,
    castling: CastlingRights,
    en_passant: Option<Square>,
    half_move_clock: u8,
    full_move_number: u16,
}

impl ChessBoard {
    /// Returns the fully qualified turn by determining the source of the piece
    ///
    /// # Errors
    ///
    /// Returns an error if the move is illegal or if the source cannot be determined
    pub fn validate_and_complete_turn(&self, turn: Turn) -> Result<Turn, TurnError> {
        match turn {
            Turn::Move(r#move) => {
                let src = Some(self.validate_move(&r#move)?);
                Ok(Turn::Move(Move { src, ..r#move }))
            }
            Turn::Castling(castling, flags) => {
                self.validate_castling(&castling, &flags)?;
                Ok(turn)
            }
        }
    }
    /// Updates the piece locations given a fully qualified turn with the source square specified
    ///
    /// # Side effects
    ///
    /// Updates the piece locations, current player, en passant square, half move clock, and full
    /// move number
    ///
    /// # Panics
    ///
    /// Panics if the turn is a move and does not have [Source::Square] as the source
    pub fn update_board(&mut self, turn: &Turn) {
        match turn {
            Turn::Castling(castling_type, _) => {
                let new_king;
                let new_rook;
                let old_king_loc: Square;
                let old_rook_loc: Square;
                match (castling_type, self.is_white) {
                    (CastlingType::Long, true) => {
                        new_king = (
                            Square::C1,
                            Piece {
                                piece: PieceType::King,
                                is_white: true,
                            },
                        );
                        new_rook = (
                            Square::D1,
                            Piece {
                                piece: PieceType::Rook,
                                is_white: true,
                            },
                        );
                        old_king_loc = Square::E1;
                        old_rook_loc = Square::A1;
                        self.castling.white_kingside = false;
                        self.castling.white_queenside = false;
                    }
                    (CastlingType::Long, false) => {
                        new_king = (
                            Square::C8,
                            Piece {
                                piece: PieceType::King,
                                is_white: false,
                            },
                        );
                        new_rook = (
                            Square::D8,
                            Piece {
                                piece: PieceType::Rook,
                                is_white: false,
                            },
                        );
                        old_king_loc = Square::E8;
                        old_rook_loc = Square::A8;
                        self.castling.black_kingside = false;
                        self.castling.black_queenside = false;
                    }
                    (CastlingType::Short, true) => {
                        new_king = (
                            Square::G1,
                            Piece {
                                piece: PieceType::King,
                                is_white: true,
                            },
                        );
                        new_rook = (
                            Square::F1,
                            Piece {
                                piece: PieceType::Rook,
                                is_white: true,
                            },
                        );
                        old_king_loc = Square::E1;
                        old_rook_loc = Square::H1;
                        self.castling.white_kingside = false;
                        self.castling.white_queenside = false;
                    }
                    (CastlingType::Short, false) => {
                        new_king = (
                            Square::G8,
                            Piece {
                                piece: PieceType::King,
                                is_white: false,
                            },
                        );
                        new_rook = (
                            Square::F8,
                            Piece {
                                piece: PieceType::Rook,
                                is_white: false,
                            },
                        );
                        old_king_loc = Square::E8;
                        old_rook_loc = Square::H8;
                        self.castling.black_kingside = false;
                        self.castling.black_queenside = false;
                    }
                }
                self.remove(&old_king_loc);
                self.remove(&old_rook_loc);
                self.insert(new_king);
                self.insert(new_rook);
            }
            Turn::Move(r#move) => {
                let Some(Source::Square(src)) = r#move.src else {
                    panic!("No specified source");
                };
                match src {
                    Square::A1 => self.castling.white_queenside = false,
                    Square::E1 => {
                        self.castling.white_kingside = false;
                        self.castling.white_queenside = false;
                    }
                    Square::H1 => self.castling.white_kingside = false,
                    Square::A8 => self.castling.black_queenside = false,
                    Square::E8 => {
                        self.castling.black_kingside = false;
                        self.castling.black_queenside = false;
                    }
                    Square::H8 => self.castling.black_kingside = false,
                    _ => (),
                };
                match r#move.dst {
                    Square::A1 => self.castling.white_queenside = false,
                    Square::H1 => self.castling.white_kingside = false,
                    Square::A8 => self.castling.black_queenside = false,
                    Square::H8 => self.castling.black_kingside = false,
                    _ => (),
                };
                let piece = (
                    r#move.dst,
                    Piece {
                        piece: if let Some(prom) = r#move.promotion {
                            prom
                        } else {
                            r#move.piece
                        },
                        is_white: self.is_white,
                    },
                );

                // update en passant
                if r#move.piece == PieceType::Pawn
                    && if self.is_white {
                        Line::Rank4.to_vec().contains(&r#move.dst)
                            && Line::Rank2.to_vec().contains(&src)
                    } else {
                        Line::Rank5.to_vec().contains(&r#move.dst)
                            && Line::Rank7.to_vec().contains(&src)
                    }
                {
                    self.en_passant = if self.is_white { src.up() } else { src.down() };
                } else {
                    self.en_passant = None;
                }

                // update the board
                self.remove(&src);
                self.insert(piece);
                if self.en_passant.is_some_and(|sq| sq == r#move.dst) {
                    if self.is_white {
                        self.remove(&r#move.dst.down().expect("is valid square"));
                    } else {
                        self.remove(&r#move.dst.up().expect("is valid square"));
                    }
                }
            }
        }
        // update fifty move rule
        if let Turn::Move(r#move) = turn {
            if r#move.piece == PieceType::Pawn || self.get(&r#move.dst).is_some() {
                self.half_move_clock = 0;
            } else {
                self.half_move_clock += 1;
            }
        }
        if !self.is_white {
            self.full_move_number += 1;
        }
        self.is_white = !self.is_white;
    }
    /// Returns what the gamestate is based on the board state and the position history
    ///
    /// The current player must be the player who will play next, rather than the player who just
    /// made the move, so this function must be run after [ChessBoard::update_board]
    pub fn check_gamestate(&self, position_hist: &Counter<String>) -> GameState {
        let mut moves: Vec<Turn> = Vec::new();
        for pc in self.get_player_pieces(self.is_white) {
            let this_piece_moves = self.gen_moves(pc);
            for dst in this_piece_moves {
                let new_turn = Turn::new((*pc.0, *pc.1), dst);
                moves.push(new_turn);
            }
        }
        let no_moves_left = moves
            .iter()
            .all(|turn| self.causes_check(turn, self.is_white));

        // checkmate and stalemate
        if no_moves_left && self.is_in_check(self.is_white) {
            if !self.is_white {
                return GameState::Win(Win {
                    is_white: true,
                    kind: WinType::Checkmate,
                });
            } else {
                return GameState::Win(Win {
                    is_white: false,
                    kind: WinType::Checkmate,
                });
            }
        } else if no_moves_left {
            return GameState::Draw(DrawType::Stalemate);
        }

        // other draws
        if self.is_insufficient_material() {
            return GameState::Draw(DrawType::InsufficientMaterial);
        }

        if self.is_threefold_repitition(position_hist) {
            return GameState::Draw(DrawType::ThreefoldRepitition);
        }

        if self.half_move_clock >= 100 {
            return GameState::Draw(DrawType::FiftyMove);
        }

        GameState::Continue
    }
    /// Returns whether the current player is white
    pub fn is_white(&self) -> bool {
        self.is_white
    }
    /// Returns an error if the flags provided in a turn are invalid
    pub fn enforce_flags(&self, turn: &Turn) -> Result<(), TurnError> {
        let flags = match turn {
            Turn::Castling(_, flag) => *flag,
            Turn::Move(r#move) => r#move.flags,
        };
        if let Turn::Move(r#move) = turn {
            match (
                self.get(&r#move.dst).is_some(),
                is_flag_set(flags, Flag::CAPTURE),
            ) {
                (true, true) => (),
                (true, false) => return Err(TurnError::NeedCaptureSpecifier),
                (false, true) => return Err(TurnError::RemoveCaptureSpecifier),
                (false, false) => (),
            }
        }
        match (
            self.causes_checkmate(turn),
            is_flag_set(flags, Flag::CHECKMATE),
        ) {
            (true, true) => return Ok(()),
            (true, false) => return Err(TurnError::NeedCheckmateSpecifier),
            (false, true) => return Err(TurnError::RemoveCheckmateSpecifier),
            (false, false) => (),
        }
        match (
            self.causes_check(turn, !self.is_white),
            is_flag_set(flags, Flag::CHECK),
        ) {
            (true, true) => (),
            (true, false) => return Err(TurnError::NeedCheckSpecifier),
            (false, true) => return Err(TurnError::RemoveCheckSpecifier),
            (false, false) => (),
        }
        if self.causes_check(turn, self.is_white) {
            return Err(TurnError::KingInCheck);
        }
        Ok(())
    }
    /// Returns the inputted turn with the proper flags set
    pub fn gen_flags(&self, turn: Turn) -> Turn {
        let mut flags: u8 = 0;
        if self.causes_checkmate(&turn) {
            flags |= Flag::CHECKMATE;
        } else if self.causes_check(&turn, !self.is_white) {
            flags |= Flag::CHECK;
        }
        if let Turn::Move(Move { dst, .. }) = turn {
            if self.get(&dst).is_some() {
                flags |= Flag::CAPTURE;
            }
        };

        match turn {
            Turn::Castling(castling_type, _) => Turn::Castling(castling_type, flags),
            Turn::Move(r#move) => Turn::Move(Move { flags, ..r#move }),
        }
    }
    /// Returns the fen string for the current board state
    pub fn gen_fen(&self) -> String {
        let mut fen = String::new();

        for rank in ('1'..='8').rev().map(|c| Line::new(c).unwrap()) {
            // let mut line = String::new();
            // for loc in rank.to_vec() {
            //     let piece_char = if let Some(pc) = self.get(&loc) {
            //         format!("{:#}", pc)
            //     } else {
            //         String::from("1")
            //     };
            //     line.push_str(&piece_char);
            // }
            // let line = line.chars().fold(String::new(), |mut full_line, c| {
            //     match (c.is_numeric(), unsafe {
            //         full_line.as_bytes_mut().last_mut()
            //     }) {
            //         (true, Some(last_char)) if (*last_char as char).is_numeric() => *last_char += 1,
            //         _ => full_line.push(c),
            //     }
            //     full_line
            // });
            //
            let mut line: Vec<u8> = Vec::new();
            for loc in rank.to_vec() {
                let piece_char = if let Some(pc) = self.get(&loc) {
                    format!("{:#}", pc).into_bytes().into_iter().nth(0).unwrap()
                } else {
                    b'1'
                };
                line.push(piece_char);
            }
            let line = String::from_utf8(line.into_iter().fold(
                Vec::new(),
                |mut full_line: Vec<u8>, c| {
                    match (c.is_ascii_digit(), full_line.last_mut()) {
                        (true, Some(last_char)) if (last_char.is_ascii_digit()) => *last_char += 1,
                        _ => full_line.push(c),
                    }
                    full_line
                },
            ))
            .expect("is always ascii");

            fen.push_str(&line);
            fen.push('/');
        }
        fen.remove(fen.len() - 1);
        fen.push(' ');

        fen.push(if self.is_white { 'w' } else { 'b' });
        fen.push(' ');

        let mut castling = String::new();
        if self.castling.white_kingside {
            castling.push('K');
        }
        if self.castling.white_queenside {
            castling.push('Q');
        }
        if self.castling.black_kingside {
            castling.push('k');
        }
        if self.castling.black_queenside {
            castling.push('q');
        }
        if castling.is_empty() {
            castling.push('-');
        }
        fen.push_str(&castling);
        fen.push(' ');

        if let Some(en_passant) = self.en_passant {
            fen.push_str(&en_passant.to_string());
        } else {
            fen.push('-');
        }
        fen.push(' ');

        fen.push_str(&self.half_move_clock.to_string());
        fen.push(' ');

        fen.push_str(&self.full_move_number.to_string());

        fen
    }
    fn validate_move(&self, r#move: &Move) -> Result<Source, TurnError> {
        let mut potential_moves: Vec<(Square, Vec<Square>)> = Vec::new();
        for piece in self.find_pieces(Piece {
            piece: r#move.piece,
            is_white: self.is_white,
        }) {
            let mut generated_moves = self.gen_moves(piece);
            if generated_moves.contains(&r#move.dst) {
                generated_moves.retain(|sq| {
                    !self.causes_check(&Turn::new((*piece.0, *piece.1), *sq), self.is_white)
                });
                potential_moves.push((*piece.0, generated_moves));
            }
        }

        let src = match potential_moves.len() {
            0 => Err(TurnError::NoTarget),
            1 => {
                let source = potential_moves[0].0;
                match r#move.src {
                    None => {
                        if r#move.piece != PieceType::Pawn || r#move.dst.file() == source.file() {
                            Ok(Source::Square(source))
                        } else {
                            Err(TurnError::NeedFile)
                        }
                    }
                    Some(Source::Square(sq)) => {
                        if sq == source {
                            Ok(Source::Square(source))
                        } else {
                            Err(TurnError::MissingAtSquare)
                        }
                    }
                    Some(Source::Line(line)) => {
                        if r#move.piece == PieceType::Pawn {
                            if line.is_file() {
                                Ok(Source::Square(source))
                            } else {
                                Err(TurnError::NeedFile)
                            }
                        } else {
                            Err(TurnError::OverSpecification)
                        }
                    }
                }
            }
            2 => {
                if let Some(Source::Line(line)) = r#move.src {
                    let matching_moves = potential_moves
                        .iter()
                        .filter(|(loc, _)| line.to_vec().contains(loc))
                        .collect::<Vec<_>>();
                    if matching_moves.is_empty() {
                        return Err(TurnError::MissingInLine);
                    }
                    if matching_moves.len() == 2 {
                        return Err(TurnError::BothInLine);
                    }
                    let source = matching_moves[0].0;
                    Ok(Source::Square(source))
                } else {
                    Err(TurnError::NeedLine)
                }
            }
            _ => {
                if let Some(Source::Square(square)) = r#move.src {
                    let matching_move = potential_moves.iter().find(|(loc, _)| *loc == square);
                    if matching_move.is_some() {
                        Ok(Source::Square(square))
                    } else {
                        Err(TurnError::MissingAtSquare)
                    }
                } else {
                    Err(TurnError::NeedSquare)
                }
            }
        }?;

        Ok(src)
    }
    fn validate_castling(&self, castling: &CastlingType, _flags: &u8) -> Result<(), TurnError> {
        let is_short = *castling == CastlingType::Short;
        let castling_squares = match (is_short, self.is_white) {
            (true, true) => vec![Square::F1, Square::G1],
            (true, false) => vec![Square::F8, Square::G8],
            (false, true) => vec![Square::D1, Square::C1, Square::B1],
            (false, false) => vec![Square::D8, Square::C8, Square::B8],
        };
        let castling_right = match (is_short, self.is_white) {
            (true, true) => self.castling.white_kingside,
            (true, false) => self.castling.black_kingside,
            (false, true) => self.castling.white_queenside,
            (false, false) => self.castling.black_queenside,
        };

        if self.get_player_pieces(!self.is_white).any(|full_piece| {
            let targets = self.gen_targets(full_piece);
            castling_squares.iter().any(|sq| targets.contains(sq))
        }) || self.is_in_check(self.is_white)
        {
            return Err(TurnError::CastleThroughCheck);
        }
        if castling_squares.iter().any(|sq| self.get(sq).is_some()) {
            return Err(TurnError::CastlePathBlocked);
        }
        if !castling_right {
            return Err(TurnError::CastleLostRights);
        }
        Ok(())
    }
    /// Returns the turn with the least amount of information to fully specify a move, given a
    /// fully qualified move.
    ///
    /// # Panics
    ///
    /// Panics if the input move does not have a [Source::Square] as the source.
    pub fn get_minimum_move(&self, turn: &Turn) -> Turn {
        match turn {
            Turn::Castling(_, _) => *turn,
            Turn::Move(r#move) => {
                let file = match r#move.src {
                    Some(Source::Square(sq)) => sq.file(),
                    _ => unreachable!(),
                };
                let rank = match r#move.src {
                    Some(Source::Square(sq)) => sq.rank(),
                    _ => unreachable!(),
                };
                let turn_copies = [
                    Turn::Move(Move {
                        src: None,
                        ..*r#move
                    }),
                    Turn::Move(Move {
                        src: Some(Source::Line(file)),
                        ..*r#move
                    }),
                    Turn::Move(Move {
                        src: Some(Source::Line(rank)),
                        ..*r#move
                    }),
                ];

                let mut min_copy: Option<Turn> = None;
                for turn_copy in turn_copies {
                    if self.validate_and_complete_turn(turn_copy).is_ok() {
                        min_copy = Some(turn_copy);
                        break;
                    }
                }
                min_copy.unwrap_or(*turn)
            }
        }
    }
    fn find_pieces(&self, piece: Piece) -> impl Iterator<Item = (&Square, &Piece)> {
        self.piece_locs
            .iter()
            .filter(move |&(_, pc)| pc.piece == piece.piece && pc.is_white == piece.is_white)
    }
    fn is_in_check(&self, is_white: bool) -> bool {
        let mut king = self.find_pieces(Piece {
            piece: PieceType::King,
            is_white,
        });
        if let Some(king) = king.next() {
            self.get_player_pieces(!is_white)
                .any(|full_piece| self.gen_targets(full_piece).contains(king.0))
        } else {
            false
        }
    }
    fn get(&self, sq: &Square) -> Option<&Piece> {
        self.piece_locs.get(sq)
    }
    fn insert(&mut self, piece: (Square, Piece)) {
        self.piece_locs.insert(piece.0, piece.1);
    }
    fn remove(&mut self, sq: &Square) {
        self.piece_locs.remove(sq);
    }
    fn gen_moves(&self, full_piece: (&Square, &Piece)) -> Vec<Square> {
        let (loc, piece) = full_piece;
        let mut moves: Vec<_> = self
            .gen_targets(full_piece)
            .into_iter()
            .filter(|sq| {
                self.get(sq).is_none()
                    || self.get(sq).expect("is some from previous check").is_white != self.is_white
            })
            .collect();
        if piece.piece == PieceType::Pawn {
            moves.retain(|sq| {
                self.get(sq).is_some_and(|pc| pc.is_white != self.is_white)
                    || self.en_passant.is_some_and(|a| a == *sq)
            })
        }
        if piece.piece == PieceType::Pawn && piece.is_white {
            let uu = |sq: &Square| sq.up()?.up();
            if let Some(next_sq) = loc.up() {
                if self.get(&next_sq).is_none() {
                    moves.push(next_sq);
                }
            }
            if let Some(next_sq) = uu(loc) {
                if loc.rank() == Line::Rank2
                    && self.get(&next_sq).is_none()
                    && self
                        .get(&loc.up().expect("is some from prev check"))
                        .is_none()
                {
                    moves.push(next_sq);
                }
            }
        } else if piece.piece == PieceType::Pawn && !piece.is_white {
            let dd = |sq: &Square| sq.down()?.down();
            if let Some(next_sq) = loc.down() {
                if self.get(&next_sq).is_none() {
                    moves.push(next_sq);
                }
            }
            if let Some(next_sq) = dd(loc) {
                if loc.rank() == Line::Rank7
                    && self.get(&next_sq).is_none()
                    && self
                        .get(&loc.down().expect("is some from prev check"))
                        .is_none()
                {
                    moves.push(next_sq);
                }
            }
        }
        moves
    }
    fn gen_targets(&self, full_piece: (&Square, &Piece)) -> Vec<Square> {
        let (loc, piece) = full_piece;
        let mut moves = Vec::new();
        let mut stop_going = |curr_sq: &mut Square, next_sq: Square| {
            let next_piece = self.get(&next_sq);
            if next_piece.is_some() {
                moves.push(next_sq);
                true
            } else {
                moves.push(next_sq);
                *curr_sq = next_sq;
                false
            }
        };
        match piece.piece {
            PieceType::King => {
                let directions = vec![
                    Square::up,
                    Square::down,
                    Square::right,
                    Square::left,
                    Square::up_right,
                    Square::up_left,
                    Square::down_right,
                    Square::down_left,
                ];
                for direction in directions {
                    if let Some(next_sq) = direction(loc) {
                        moves.push(next_sq);
                    }
                }
            }
            PieceType::Queen => {
                let directions = vec![
                    Square::up,
                    Square::down,
                    Square::right,
                    Square::left,
                    Square::up_right,
                    Square::up_left,
                    Square::down_right,
                    Square::down_left,
                ];
                for direction in directions {
                    let mut curr_sq = *loc;
                    while let Some(next_sq) = direction(&curr_sq) {
                        if stop_going(&mut curr_sq, next_sq) {
                            break;
                        }
                    }
                }
            }
            PieceType::Rook => {
                let directions = vec![Square::up, Square::down, Square::right, Square::left];
                for direction in directions {
                    let mut curr_sq = *loc;
                    while let Some(next_sq) = direction(&curr_sq) {
                        if stop_going(&mut curr_sq, next_sq) {
                            break;
                        }
                    }
                }
            }
            PieceType::Bishop => {
                let directions = vec![
                    Square::up_right,
                    Square::up_left,
                    Square::down_right,
                    Square::down_left,
                ];
                for direction in directions {
                    let mut curr_sq = *loc;
                    while let Some(next_sq) = direction(&curr_sq) {
                        if stop_going(&mut curr_sq, next_sq) {
                            break;
                        }
                    }
                }
            }
            PieceType::Knight => {
                let uur = |sq: &Square| sq.up()?.up()?.right();
                let uul = |sq: &Square| sq.up()?.up()?.left();
                let rru = |sq: &Square| sq.right()?.right()?.up();
                let rrd = |sq: &Square| sq.right()?.right()?.down();
                let ddr = |sq: &Square| sq.down()?.down()?.right();
                let ddl = |sq: &Square| sq.down()?.down()?.left();
                let llu = |sq: &Square| sq.left()?.left()?.up();
                let lld = |sq: &Square| sq.left()?.left()?.down();
                let directions = vec![uur, uul, rru, rrd, ddr, ddl, llu, lld];
                for direction in directions {
                    if let Some(sq) = direction(loc) {
                        moves.push(sq);
                    }
                }
            }
            PieceType::Pawn => match piece.is_white {
                true => {
                    if let Some(sq) = loc.up_right() {
                        moves.push(sq);
                    }
                    if let Some(sq) = loc.up_left() {
                        moves.push(sq);
                    }
                }
                false => {
                    if let Some(sq) = loc.down_right() {
                        moves.push(sq);
                    }
                    if let Some(sq) = loc.down_left() {
                        moves.push(sq);
                    }
                }
            },
        }
        moves
    }
    fn causes_check(&self, turn: &Turn, is_white: bool) -> bool {
        let mut test_board = self.clone();
        test_board.update_board(turn);
        test_board.is_in_check(is_white)
    }
    fn causes_checkmate(&self, turn: &Turn) -> bool {
        let mut test_board = self.clone();
        test_board.update_board(turn);
        matches!(
            test_board.check_gamestate(&Counter::new()),
            GameState::Win(_)
        )
    }
    fn get_player_pieces(&self, is_white: bool) -> impl Iterator<Item = (&Square, &Piece)> {
        self.piece_locs
            .iter()
            .filter(move |(_, pc)| pc.is_white == is_white)
    }
    fn is_insufficient_material(&self) -> bool {
        let white_pieces: Vec<_> = self.get_player_pieces(true).collect();
        let black_pieces: Vec<_> = self.get_player_pieces(false).collect();
        white_pieces.len() == 1 && black_pieces.len() == 1
            || black_pieces.len() == 1
                && white_pieces.len() == 2
                && white_pieces
                    .iter()
                    .filter(|(_, pc)| [PieceType::Bishop, PieceType::Knight].contains(&pc.piece))
                    .count()
                    == 1
            || white_pieces.len() == 1
                && black_pieces.len() == 2
                && black_pieces
                    .iter()
                    .filter(|(_, pc)| [PieceType::Bishop, PieceType::Knight].contains(&pc.piece))
                    .count()
                    == 1
            || white_pieces.len() == 2
                && black_pieces.len() == 2
                && white_pieces
                    .iter()
                    .find(|(_, pc)| pc.piece == PieceType::Bishop)
                    .is_some_and(|(white_loc, _)| {
                        black_pieces
                            .iter()
                            .find(|(_, pc)| pc.piece == PieceType::Bishop)
                            .is_some_and(|(black_loc, _)| {
                                black_loc.is_light() == white_loc.is_light()
                            })
                    })
    }
    fn is_threefold_repitition(&self, position_hist: &Counter<String>) -> bool {
        position_hist.counts().any(|&count| count >= 3)
    }
}

impl Display for ChessBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        if f.alternate() {
            for (i, rank) in ('1'..='8')
                .map(|c| Line::new(c).expect("hard coded value is safe"))
                .enumerate()
            {
                output.push('\t');
                output.push_str(&(i + 1).to_string());
                output.push(' ');
                let line: String = rank
                    .into_iter()
                    .map(|x| self.get(&x).map_or(" ".to_string(), |sq| sq.to_string()) + " ")
                    .rev()
                    .collect();
                output.push_str(&(line + "\n"));
            }
            output.push_str("\t  h g f e d c b a\n");
        } else {
            for (i, rank) in ('1'..='8')
                .map(|c| Line::new(c).expect("hard coded value is safe"))
                .rev()
                .enumerate()
            {
                output.push('\t');
                output.push_str(&(8 - i).to_string());
                output.push(' ');
                let line: String = rank
                    .into_iter()
                    .map(|x| self.get(&x).map_or(" ".to_string(), |sq| sq.to_string()) + " ")
                    .collect();
                output.push_str(&(line + "\n"));
            }
            output.push_str("\t  a b c d e f g h\n");
        }
        write!(f, "{}", output)
    }
}

impl FromStr for ChessBoard {
    type Err = &'static str;
    /// parses a FEN string into a ChessBoard
    ///
    /// # Errors
    ///
    /// returns an error if the given FEN string is an invalid format
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fen_split = s.split_whitespace();
        let board = fen_split
            .next()
            .ok_or("Couldn't find the board information")?;
        let player = fen_split
            .next()
            .ok_or("Couldn't find the player information")?;
        let castling_rights = fen_split
            .next()
            .ok_or("Couldn't find the castling information")?;
        let en_passant = fen_split
            .next()
            .ok_or("Couldn't find en passant information")?;
        let half_move_clock = fen_split
            .next()
            .ok_or("Couldn't find half move clock information")?;
        let turn_number = fen_split
            .next()
            .ok_or("Couldn't find the turn number information")?;
        let None = fen_split.next() else {
            return Err("Additional fields specified");
        };

        let mut piece_locs: HashMap<Square, Piece> = HashMap::new();
        let mut board_squares = Square::iterator();
        for rank in board.split('/') {
            let mut count = 0;

            for char in rank.chars() {
                if let Some(num_empty) = char.to_digit(10) {
                    count += num_empty;
                    for _ in 0..num_empty {
                        board_squares.next();
                    }
                    continue;
                }
                let sq = board_squares
                    .next()
                    .ok_or("Too many locations on the board")?;
                count += 1;
                let piece = Piece {
                    piece: char
                        .to_ascii_uppercase()
                        .try_into()
                        .map_err(|_| "Invalid character in board")?,
                    is_white: char.is_ascii_uppercase(),
                };
                piece_locs.insert(sq, piece);
            }
            if count != 8 {
                return Err("Invalid number of pieces on a line");
            }
        }

        let is_white = match player {
            "w" => true,
            "b" => false,
            _ => return Err("Invalid player specified"),
        };

        let mut castling = CastlingRights {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        };
        let mut castling_chars = castling_rights.chars();
        let mut full_castling = vec!['K', 'Q', 'k', 'q'].into_iter();
        let mut passed_chars = Vec::new();

        let mut next_input_char = castling_chars.next();
        let mut next_test_char = full_castling.next();
        loop {
            let current_target = match next_test_char {
                Some('K') => &mut castling.white_kingside,
                Some('Q') => &mut castling.white_queenside,
                Some('k') => &mut castling.black_kingside,
                Some('q') => &mut castling.black_queenside,
                None if next_input_char.is_none() => break,
                None if next_input_char.is_some() => {
                    return Err("Additional characters specified after `q` which is the end")
                }
                _ => unreachable!(),
            };

            if next_test_char == next_input_char {
                *current_target = true;
                passed_chars.push(next_test_char);
                next_input_char = castling_chars.next();
                next_test_char = full_castling.next();
            } else if next_input_char.is_none() {
                break;
            } else if next_input_char == Some('-') {
                if castling_chars.next().is_some() {
                    return Err("Additional characters specified after `-`");
                }
                break;
            } else if !matches!(
                next_input_char,
                Some('K') | Some('Q') | Some('k') | Some('q')
            ) {
                return Err("Invalid characters in castling input");
            } else if passed_chars.contains(&next_input_char) {
                return Err("Out of order castling");
            } else {
                passed_chars.push(next_test_char);
                next_test_char = full_castling.next();
            }
        }

        let en_passant = match en_passant {
            "-" => None,
            sq if sq.len() == 2 => Some(
                sq.parse::<Square>()
                    .map_err(|_| "En passant was not a square")?,
            ),
            _ => return Err("En passant was not a square"),
        };

        let half_move_clock = half_move_clock
            .parse::<u8>()
            .map_err(|_| "Half move clock was not a number")?;
        let full_move_number = turn_number
            .parse::<u16>()
            .map_err(|_| "Full move number was not a number")?;

        Ok(ChessBoard {
            piece_locs,
            is_white,
            castling,
            en_passant,
            half_move_clock,
            full_move_number,
        })
    }
}

fn is_flag_set(flags: u8, check_flag: u8) -> bool {
    flags & check_flag != 0
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
/// The current game state
pub enum GameState {
    #[default]
    /// Game is still in play
    Continue,
    /// Game has been won by some player
    Win(Win),
    /// Game has been drawn
    Draw(DrawType),
    /// Game has been aborted
    Stop,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// The information describing the win state
pub struct Win {
    /// The player who won, true if white
    pub is_white: bool,
    /// The type of win
    pub kind: WinType,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// The type of win
pub enum WinType {
    /// Win by checkmate
    Checkmate,
    /// Win by resignation
    Resign,
    /// Win by timeout
    Timeout,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// The type of draw
pub enum DrawType {
    /// Draw by stalemate
    Stalemate,
    /// Draw by the fifty move rule
    FiftyMove,
    /// Draw by threefold repetition
    ThreefoldRepitition,
    /// Draw by insufficient material
    InsufficientMaterial,
    /// Draw by draw offer
    Offer,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}
impl Default for CastlingRights {
    fn default() -> Self {
        CastlingRights {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_fen() {
        let test = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        assert!(test.parse::<ChessBoard>().is_ok());
        let test = "lsefw sefwoe fjwofnwf weefwlkfn wlefkwlkfn sdf";
        assert!(test.parse::<ChessBoard>().is_err());
    }
}
