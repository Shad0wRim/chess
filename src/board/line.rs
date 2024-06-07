use std::{fmt::Display, str::FromStr};

use super::square::Square;
use lines::*;

#[derive(Debug, Clone, PartialEq, Copy)]
#[rustfmt::skip]
pub enum Line {
   Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8,
   FileA, FileB, FileC, FileD, FileE, FileF, FileG, FileH,
}

#[allow(dead_code)]
impl Line {
    pub fn new(rank_or_file: char) -> Result<Line, &'static str> {
        match rank_or_file {
            '1' => Ok(Self::Rank1),
            '2' => Ok(Self::Rank2),
            '3' => Ok(Self::Rank3),
            '4' => Ok(Self::Rank4),
            '5' => Ok(Self::Rank5),
            '6' => Ok(Self::Rank6),
            '7' => Ok(Self::Rank7),
            '8' => Ok(Self::Rank8),
            'a' => Ok(Self::FileA),
            'b' => Ok(Self::FileB),
            'c' => Ok(Self::FileC),
            'd' => Ok(Self::FileD),
            'e' => Ok(Self::FileE),
            'f' => Ok(Self::FileF),
            'g' => Ok(Self::FileG),
            'h' => Ok(Self::FileH),
            _ => Err("cannot create line from that character"),
        }
    }
    pub fn to_vec(self) -> Vec<Square> {
        match self {
            Line::Rank1 => RANK_1.to_owned(),
            Line::Rank2 => RANK_2.to_owned(),
            Line::Rank3 => RANK_3.to_owned(),
            Line::Rank4 => RANK_4.to_owned(),
            Line::Rank5 => RANK_5.to_owned(),
            Line::Rank6 => RANK_6.to_owned(),
            Line::Rank7 => RANK_7.to_owned(),
            Line::Rank8 => RANK_8.to_owned(),
            Line::FileA => FILE_A.to_owned(),
            Line::FileB => FILE_B.to_owned(),
            Line::FileC => FILE_C.to_owned(),
            Line::FileD => FILE_D.to_owned(),
            Line::FileE => FILE_E.to_owned(),
            Line::FileF => FILE_F.to_owned(),
            Line::FileG => FILE_G.to_owned(),
            Line::FileH => FILE_H.to_owned(),
        }
    }
    pub fn intersection(&self, line: &Line) -> Option<Square> {
        match self {
            Line::Rank1 => match line {
                Line::FileA => Some(Square::A1),
                Line::FileB => Some(Square::B1),
                Line::FileC => Some(Square::C1),
                Line::FileD => Some(Square::D1),
                Line::FileE => Some(Square::E1),
                Line::FileF => Some(Square::F1),
                Line::FileG => Some(Square::G1),
                Line::FileH => Some(Square::H1),
                _ => None,
            },
            Line::Rank2 => match line {
                Line::FileA => Some(Square::A2),
                Line::FileB => Some(Square::B2),
                Line::FileC => Some(Square::C2),
                Line::FileD => Some(Square::D2),
                Line::FileE => Some(Square::E2),
                Line::FileF => Some(Square::F2),
                Line::FileG => Some(Square::G2),
                Line::FileH => Some(Square::H2),
                _ => None,
            },
            Line::Rank3 => match line {
                Line::FileA => Some(Square::A3),
                Line::FileB => Some(Square::B3),
                Line::FileC => Some(Square::C3),
                Line::FileD => Some(Square::D3),
                Line::FileE => Some(Square::E3),
                Line::FileF => Some(Square::F3),
                Line::FileG => Some(Square::G3),
                Line::FileH => Some(Square::H3),
                _ => None,
            },
            Line::Rank4 => match line {
                Line::FileA => Some(Square::A4),
                Line::FileB => Some(Square::B4),
                Line::FileC => Some(Square::C4),
                Line::FileD => Some(Square::D4),
                Line::FileE => Some(Square::E4),
                Line::FileF => Some(Square::F4),
                Line::FileG => Some(Square::G4),
                Line::FileH => Some(Square::H4),
                _ => None,
            },
            Line::Rank5 => match line {
                Line::FileA => Some(Square::A5),
                Line::FileB => Some(Square::B5),
                Line::FileC => Some(Square::C5),
                Line::FileD => Some(Square::D5),
                Line::FileE => Some(Square::E5),
                Line::FileF => Some(Square::F5),
                Line::FileG => Some(Square::G5),
                Line::FileH => Some(Square::H5),
                _ => None,
            },
            Line::Rank6 => match line {
                Line::FileA => Some(Square::A6),
                Line::FileB => Some(Square::B6),
                Line::FileC => Some(Square::C6),
                Line::FileD => Some(Square::D6),
                Line::FileE => Some(Square::E6),
                Line::FileF => Some(Square::F6),
                Line::FileG => Some(Square::G6),
                Line::FileH => Some(Square::H6),
                _ => None,
            },
            Line::Rank7 => match line {
                Line::FileA => Some(Square::A7),
                Line::FileB => Some(Square::B7),
                Line::FileC => Some(Square::C7),
                Line::FileD => Some(Square::D7),
                Line::FileE => Some(Square::E7),
                Line::FileF => Some(Square::F7),
                Line::FileG => Some(Square::G7),
                Line::FileH => Some(Square::H7),
                _ => None,
            },
            Line::Rank8 => match line {
                Line::FileA => Some(Square::A8),
                Line::FileB => Some(Square::B8),
                Line::FileC => Some(Square::C8),
                Line::FileD => Some(Square::D8),
                Line::FileE => Some(Square::E8),
                Line::FileF => Some(Square::F8),
                Line::FileG => Some(Square::G8),
                Line::FileH => Some(Square::H8),
                _ => None,
            },
            Line::FileA => match line {
                Line::Rank1 => Some(Square::A1),
                Line::Rank2 => Some(Square::A2),
                Line::Rank3 => Some(Square::A3),
                Line::Rank4 => Some(Square::A4),
                Line::Rank5 => Some(Square::A5),
                Line::Rank6 => Some(Square::A6),
                Line::Rank7 => Some(Square::A7),
                Line::Rank8 => Some(Square::A8),
                _ => None,
            },
            Line::FileB => match line {
                Line::Rank1 => Some(Square::B1),
                Line::Rank2 => Some(Square::B2),
                Line::Rank3 => Some(Square::B3),
                Line::Rank4 => Some(Square::B4),
                Line::Rank5 => Some(Square::B5),
                Line::Rank6 => Some(Square::B6),
                Line::Rank7 => Some(Square::B7),
                Line::Rank8 => Some(Square::B8),
                _ => None,
            },
            Line::FileC => match line {
                Line::Rank1 => Some(Square::C1),
                Line::Rank2 => Some(Square::C2),
                Line::Rank3 => Some(Square::C3),
                Line::Rank4 => Some(Square::C4),
                Line::Rank5 => Some(Square::C5),
                Line::Rank6 => Some(Square::C6),
                Line::Rank7 => Some(Square::C7),
                Line::Rank8 => Some(Square::C8),
                _ => None,
            },
            Line::FileD => match line {
                Line::Rank1 => Some(Square::D1),
                Line::Rank2 => Some(Square::D2),
                Line::Rank3 => Some(Square::D3),
                Line::Rank4 => Some(Square::D4),
                Line::Rank5 => Some(Square::D5),
                Line::Rank6 => Some(Square::D6),
                Line::Rank7 => Some(Square::D7),
                Line::Rank8 => Some(Square::D8),
                _ => None,
            },
            Line::FileE => match line {
                Line::Rank1 => Some(Square::E1),
                Line::Rank2 => Some(Square::E2),
                Line::Rank3 => Some(Square::E3),
                Line::Rank4 => Some(Square::E4),
                Line::Rank5 => Some(Square::E5),
                Line::Rank6 => Some(Square::E6),
                Line::Rank7 => Some(Square::E7),
                Line::Rank8 => Some(Square::E8),
                _ => None,
            },
            Line::FileF => match line {
                Line::Rank1 => Some(Square::F1),
                Line::Rank2 => Some(Square::F2),
                Line::Rank3 => Some(Square::F3),
                Line::Rank4 => Some(Square::F4),
                Line::Rank5 => Some(Square::F5),
                Line::Rank6 => Some(Square::F6),
                Line::Rank7 => Some(Square::F7),
                Line::Rank8 => Some(Square::F8),
                _ => None,
            },
            Line::FileG => match line {
                Line::Rank1 => Some(Square::G1),
                Line::Rank2 => Some(Square::G2),
                Line::Rank3 => Some(Square::G3),
                Line::Rank4 => Some(Square::G4),
                Line::Rank5 => Some(Square::G5),
                Line::Rank6 => Some(Square::G6),
                Line::Rank7 => Some(Square::G7),
                Line::Rank8 => Some(Square::G8),
                _ => None,
            },
            Line::FileH => match line {
                Line::Rank1 => Some(Square::H1),
                Line::Rank2 => Some(Square::H2),
                Line::Rank3 => Some(Square::H3),
                Line::Rank4 => Some(Square::H4),
                Line::Rank5 => Some(Square::H5),
                Line::Rank6 => Some(Square::H6),
                Line::Rank7 => Some(Square::H7),
                Line::Rank8 => Some(Square::H8),
                _ => None,
            },
        }
    }
    pub fn is_file(&self) -> bool {
        matches!(
            self,
            Line::FileA
                | Line::FileB
                | Line::FileC
                | Line::FileD
                | Line::FileE
                | Line::FileF
                | Line::FileG
                | Line::FileH
        )
    }
    pub fn is_rank(&self) -> bool {
        matches!(
            self,
            Line::Rank1
                | Line::Rank2
                | Line::Rank3
                | Line::Rank4
                | Line::Rank5
                | Line::Rank6
                | Line::Rank7
                | Line::Rank8
        )
    }
}
impl IntoIterator for Line {
    type Item = Square;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}
impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            Self::new(s.chars().next().unwrap())
        } else {
            Err("input too long")
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Line::Rank1 => write!(f, "1"),
            Line::Rank2 => write!(f, "2"),
            Line::Rank3 => write!(f, "3"),
            Line::Rank4 => write!(f, "4"),
            Line::Rank5 => write!(f, "5"),
            Line::Rank6 => write!(f, "6"),
            Line::Rank7 => write!(f, "7"),
            Line::Rank8 => write!(f, "8"),
            Line::FileA => write!(f, "a"),
            Line::FileB => write!(f, "b"),
            Line::FileC => write!(f, "c"),
            Line::FileD => write!(f, "d"),
            Line::FileE => write!(f, "e"),
            Line::FileF => write!(f, "f"),
            Line::FileG => write!(f, "g"),
            Line::FileH => write!(f, "h"),
        }
    }
}

pub mod lines {
    use super::Square;
    pub const RANK_1: &[Square] = &[
        Square::A1,
        Square::B1,
        Square::C1,
        Square::D1,
        Square::E1,
        Square::F1,
        Square::G1,
        Square::H1,
    ];
    pub const RANK_2: &[Square] = &[
        Square::A2,
        Square::B2,
        Square::C2,
        Square::D2,
        Square::E2,
        Square::F2,
        Square::G2,
        Square::H2,
    ];
    pub const RANK_3: &[Square] = &[
        Square::A3,
        Square::B3,
        Square::C3,
        Square::D3,
        Square::E3,
        Square::F3,
        Square::G3,
        Square::H3,
    ];
    pub const RANK_4: &[Square] = &[
        Square::A4,
        Square::B4,
        Square::C4,
        Square::D4,
        Square::E4,
        Square::F4,
        Square::G4,
        Square::H4,
    ];
    pub const RANK_5: &[Square] = &[
        Square::A5,
        Square::B5,
        Square::C5,
        Square::D5,
        Square::E5,
        Square::F5,
        Square::G5,
        Square::H5,
    ];
    pub const RANK_6: &[Square] = &[
        Square::A6,
        Square::B6,
        Square::C6,
        Square::D6,
        Square::E6,
        Square::F6,
        Square::G6,
        Square::H6,
    ];
    pub const RANK_7: &[Square] = &[
        Square::A7,
        Square::B7,
        Square::C7,
        Square::D7,
        Square::E7,
        Square::F7,
        Square::G7,
        Square::H7,
    ];
    pub const RANK_8: &[Square] = &[
        Square::A8,
        Square::B8,
        Square::C8,
        Square::D8,
        Square::E8,
        Square::F8,
        Square::G8,
        Square::H8,
    ];
    pub const FILE_A: &[Square] = &[
        Square::A1,
        Square::A2,
        Square::A3,
        Square::A4,
        Square::A5,
        Square::A6,
        Square::A7,
        Square::A8,
    ];
    pub const FILE_B: &[Square] = &[
        Square::B1,
        Square::B2,
        Square::B3,
        Square::B4,
        Square::B5,
        Square::B6,
        Square::B7,
        Square::B8,
    ];
    pub const FILE_C: &[Square] = &[
        Square::C1,
        Square::C2,
        Square::C3,
        Square::C4,
        Square::C5,
        Square::C6,
        Square::C7,
        Square::C8,
    ];
    pub const FILE_D: &[Square] = &[
        Square::D1,
        Square::D2,
        Square::D3,
        Square::D4,
        Square::D5,
        Square::D6,
        Square::D7,
        Square::D8,
    ];
    pub const FILE_E: &[Square] = &[
        Square::E1,
        Square::E2,
        Square::E3,
        Square::E4,
        Square::E5,
        Square::E6,
        Square::E7,
        Square::E8,
    ];
    pub const FILE_F: &[Square] = &[
        Square::F1,
        Square::F2,
        Square::F3,
        Square::F4,
        Square::F5,
        Square::F6,
        Square::F7,
        Square::F8,
    ];
    pub const FILE_G: &[Square] = &[
        Square::G1,
        Square::G2,
        Square::G3,
        Square::G4,
        Square::G5,
        Square::G6,
        Square::G7,
        Square::G8,
    ];
    pub const FILE_H: &[Square] = &[
        Square::H1,
        Square::H2,
        Square::H3,
        Square::H4,
        Square::H5,
        Square::H6,
        Square::H7,
        Square::H8,
    ];
}
