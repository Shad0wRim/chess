use std::{fmt::Display, str::FromStr};

use super::line::Line;
use crate::parser::ConversionError;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Square {
   A8, B8, C8, D8, E8, F8, G8, H8,
   A7, B7, C7, D7, E7, F7, G7, H7,
   A6, B6, C6, D6, E6, F6, G6, H6,
   A5, B5, C5, D5, E5, F5, G5, H5,
   A4, B4, C4, D4, E4, F4, G4, H4,
   A3, B3, C3, D3, E3, F3, G3, H3,
   A2, B2, C2, D2, E2, F2, G2, H2,
   A1, B1, C1, D1, E1, F1, G1, H1,
}

impl Square {
    pub fn rank(&self) -> Line {
        match self {
            Self::A1
            | Self::B1
            | Self::C1
            | Self::D1
            | Self::E1
            | Self::F1
            | Self::G1
            | Self::H1 => Line::Rank1,
            Self::A2
            | Self::B2
            | Self::C2
            | Self::D2
            | Self::E2
            | Self::F2
            | Self::G2
            | Self::H2 => Line::Rank2,
            Self::A3
            | Self::B3
            | Self::C3
            | Self::D3
            | Self::E3
            | Self::F3
            | Self::G3
            | Self::H3 => Line::Rank3,
            Self::A4
            | Self::B4
            | Self::C4
            | Self::D4
            | Self::E4
            | Self::F4
            | Self::G4
            | Self::H4 => Line::Rank4,
            Self::A5
            | Self::B5
            | Self::C5
            | Self::D5
            | Self::E5
            | Self::F5
            | Self::G5
            | Self::H5 => Line::Rank5,
            Self::A6
            | Self::B6
            | Self::C6
            | Self::D6
            | Self::E6
            | Self::F6
            | Self::G6
            | Self::H6 => Line::Rank6,
            Self::A7
            | Self::B7
            | Self::C7
            | Self::D7
            | Self::E7
            | Self::F7
            | Self::G7
            | Self::H7 => Line::Rank7,
            Self::A8
            | Self::B8
            | Self::C8
            | Self::D8
            | Self::E8
            | Self::F8
            | Self::G8
            | Self::H8 => Line::Rank8,
        }
    }
    pub fn file(&self) -> Line {
        match self {
            Self::A1
            | Self::A2
            | Self::A3
            | Self::A4
            | Self::A5
            | Self::A6
            | Self::A7
            | Self::A8 => Line::FileA,
            Self::B1
            | Self::B2
            | Self::B3
            | Self::B4
            | Self::B5
            | Self::B6
            | Self::B7
            | Self::B8 => Line::FileB,
            Self::C1
            | Self::C2
            | Self::C3
            | Self::C4
            | Self::C5
            | Self::C6
            | Self::C7
            | Self::C8 => Line::FileC,
            Self::D1
            | Self::D2
            | Self::D3
            | Self::D4
            | Self::D5
            | Self::D6
            | Self::D7
            | Self::D8 => Line::FileD,
            Self::E1
            | Self::E2
            | Self::E3
            | Self::E4
            | Self::E5
            | Self::E6
            | Self::E7
            | Self::E8 => Line::FileE,
            Self::F1
            | Self::F2
            | Self::F3
            | Self::F4
            | Self::F5
            | Self::F6
            | Self::F7
            | Self::F8 => Line::FileF,
            Self::G1
            | Self::G2
            | Self::G3
            | Self::G4
            | Self::G5
            | Self::G6
            | Self::G7
            | Self::G8 => Line::FileG,
            Self::H1
            | Self::H2
            | Self::H3
            | Self::H4
            | Self::H5
            | Self::H6
            | Self::H7
            | Self::H8 => Line::FileH,
        }
    }
    pub fn is_light(&self) -> bool {
        matches!(
            self,
            Self::A8
                | Self::A6
                | Self::A4
                | Self::A2
                | Self::B7
                | Self::B5
                | Self::B3
                | Self::B1
                | Self::C8
                | Self::C6
                | Self::C4
                | Self::C2
                | Self::D7
                | Self::D5
                | Self::D3
                | Self::D1
                | Self::E8
                | Self::E6
                | Self::E4
                | Self::E2
                | Self::F7
                | Self::F5
                | Self::F3
                | Self::F1
                | Self::G8
                | Self::G6
                | Self::G4
                | Self::G2
                | Self::H7
                | Self::H5
                | Self::H3
                | Self::H1
        )
    }
    pub fn up(&self) -> Option<Square> {
        match self.rank() {
            Line::Rank1 => Line::Rank2.intersection(&self.file()),
            Line::Rank2 => Line::Rank3.intersection(&self.file()),
            Line::Rank3 => Line::Rank4.intersection(&self.file()),
            Line::Rank4 => Line::Rank5.intersection(&self.file()),
            Line::Rank5 => Line::Rank6.intersection(&self.file()),
            Line::Rank6 => Line::Rank7.intersection(&self.file()),
            Line::Rank7 => Line::Rank8.intersection(&self.file()),
            Line::Rank8 => None,
            _ => unreachable!(),
        }
    }
    pub fn down(&self) -> Option<Square> {
        match self.rank() {
            Line::Rank1 => None,
            Line::Rank2 => Line::Rank1.intersection(&self.file()),
            Line::Rank3 => Line::Rank2.intersection(&self.file()),
            Line::Rank4 => Line::Rank3.intersection(&self.file()),
            Line::Rank5 => Line::Rank4.intersection(&self.file()),
            Line::Rank6 => Line::Rank5.intersection(&self.file()),
            Line::Rank7 => Line::Rank6.intersection(&self.file()),
            Line::Rank8 => Line::Rank7.intersection(&self.file()),
            _ => unreachable!(),
        }
    }
    pub fn right(&self) -> Option<Square> {
        match self.file() {
            Line::FileA => Line::FileB.intersection(&self.rank()),
            Line::FileB => Line::FileC.intersection(&self.rank()),
            Line::FileC => Line::FileD.intersection(&self.rank()),
            Line::FileD => Line::FileE.intersection(&self.rank()),
            Line::FileE => Line::FileF.intersection(&self.rank()),
            Line::FileF => Line::FileG.intersection(&self.rank()),
            Line::FileG => Line::FileH.intersection(&self.rank()),
            Line::FileH => None,
            _ => unreachable!(),
        }
    }
    pub fn left(&self) -> Option<Square> {
        match self.file() {
            Line::FileA => None,
            Line::FileB => Line::FileA.intersection(&self.rank()),
            Line::FileC => Line::FileB.intersection(&self.rank()),
            Line::FileD => Line::FileC.intersection(&self.rank()),
            Line::FileE => Line::FileD.intersection(&self.rank()),
            Line::FileF => Line::FileE.intersection(&self.rank()),
            Line::FileG => Line::FileF.intersection(&self.rank()),
            Line::FileH => Line::FileG.intersection(&self.rank()),
            _ => unreachable!(),
        }
    }
    pub fn up_right(&self) -> Option<Square> {
        self.up()?.right()
    }
    pub fn up_left(&self) -> Option<Square> {
        self.up()?.left()
    }
    pub fn down_right(&self) -> Option<Square> {
        self.down()?.right()
    }
    pub fn down_left(&self) -> Option<Square> {
        self.down()?.left()
    }
    pub fn iterator() -> impl Iterator<Item = Square> {
        unsafe { (Self::A8 as u8..=Self::H1 as u8).map(|num| std::mem::transmute(num)) }
    }
}
impl FromStr for Square {
    type Err = ConversionError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 2 {
            return Err(ConversionError {
                input: value.to_string(),
                target: String::from("Square"),
            });
        }
        let file = match value.chars().next().unwrap() {
            file @ 'a'..='h' => Line::new(file).unwrap(),
            _ => {
                return Err(ConversionError {
                    input: value.to_string(),
                    target: String::from("File"),
                })
            }
        };
        let rank = match value.chars().last().unwrap() {
            rank @ '1'..='8' => Line::new(rank).unwrap(),
            _ => {
                return Err(ConversionError {
                    input: value.to_string(),
                    target: String::from("Rank"),
                })
            }
        };
        Ok(file.intersection(&rank).unwrap())
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = match self.file() {
            Line::FileA => 'a',
            Line::FileB => 'b',
            Line::FileC => 'c',
            Line::FileD => 'd',
            Line::FileE => 'e',
            Line::FileF => 'f',
            Line::FileG => 'g',
            Line::FileH => 'h',
            _ => unreachable!(),
        };
        let rank = match self.rank() {
            Line::Rank1 => '1',
            Line::Rank2 => '2',
            Line::Rank3 => '3',
            Line::Rank4 => '4',
            Line::Rank5 => '5',
            Line::Rank6 => '6',
            Line::Rank7 => '7',
            Line::Rank8 => '8',
            _ => unreachable!(),
        };
        write!(f, "{}{}", file, rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn square_iterator() {
        let mut test = Square::iterator().take(5);
        assert_eq!(test.next(), Some(Square::A8));
        assert_eq!(test.next(), Some(Square::B8));
        assert_eq!(test.next(), Some(Square::C8));
        assert_eq!(test.next(), Some(Square::D8));
        assert_eq!(test.next(), Some(Square::E8));
        assert_eq!(test.next(), None);
        let test = Square::iterator().last();
        assert_eq!(test, Some(Square::H1));
    }
    #[test]
    fn display() {
        assert_eq!(Square::H1.to_string(), String::from("h1"));
    }
}
