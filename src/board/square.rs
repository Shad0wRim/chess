use std::{fmt::Display, str::FromStr};

use super::line::Line;
use crate::parser::ConversionError;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
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
   #[rustfmt::skip]
    pub fn is_light(&self) -> bool {
      matches!(
         self,
           Self::A8 | Self::A6 | Self::A4 | Self::A2 | Self::B7 | Self::B5 | Self::B3 | Self::B1
         | Self::C8 | Self::C6 | Self::C4 | Self::C2 | Self::D7 | Self::D5 | Self::D3 | Self::D1
         | Self::E8 | Self::E6 | Self::E4 | Self::E2 | Self::F7 | Self::F5 | Self::F3 | Self::F1
         | Self::G8 | Self::G6 | Self::G4 | Self::G2 | Self::H7 | Self::H5 | Self::H3 | Self::H1
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
}
impl FromStr for Square {
    type Err = ConversionError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "a1" => Ok(Self::A1),
            "a2" => Ok(Self::A2),
            "a3" => Ok(Self::A3),
            "a4" => Ok(Self::A4),
            "a5" => Ok(Self::A5),
            "a6" => Ok(Self::A6),
            "a7" => Ok(Self::A7),
            "a8" => Ok(Self::A8),
            "b1" => Ok(Self::B1),
            "b2" => Ok(Self::B2),
            "b3" => Ok(Self::B3),
            "b4" => Ok(Self::B4),
            "b5" => Ok(Self::B5),
            "b6" => Ok(Self::B6),
            "b7" => Ok(Self::B7),
            "b8" => Ok(Self::B8),
            "c1" => Ok(Self::C1),
            "c2" => Ok(Self::C2),
            "c3" => Ok(Self::C3),
            "c4" => Ok(Self::C4),
            "c5" => Ok(Self::C5),
            "c6" => Ok(Self::C6),
            "c7" => Ok(Self::C7),
            "c8" => Ok(Self::C8),
            "d1" => Ok(Self::D1),
            "d2" => Ok(Self::D2),
            "d3" => Ok(Self::D3),
            "d4" => Ok(Self::D4),
            "d5" => Ok(Self::D5),
            "d6" => Ok(Self::D6),
            "d7" => Ok(Self::D7),
            "d8" => Ok(Self::D8),
            "e1" => Ok(Self::E1),
            "e2" => Ok(Self::E2),
            "e3" => Ok(Self::E3),
            "e4" => Ok(Self::E4),
            "e5" => Ok(Self::E5),
            "e6" => Ok(Self::E6),
            "e7" => Ok(Self::E7),
            "e8" => Ok(Self::E8),
            "f1" => Ok(Self::F1),
            "f2" => Ok(Self::F2),
            "f3" => Ok(Self::F3),
            "f4" => Ok(Self::F4),
            "f5" => Ok(Self::F5),
            "f6" => Ok(Self::F6),
            "f7" => Ok(Self::F7),
            "f8" => Ok(Self::F8),
            "g1" => Ok(Self::G1),
            "g2" => Ok(Self::G2),
            "g3" => Ok(Self::G3),
            "g4" => Ok(Self::G4),
            "g5" => Ok(Self::G5),
            "g6" => Ok(Self::G6),
            "g7" => Ok(Self::G7),
            "g8" => Ok(Self::G8),
            "h1" => Ok(Self::H1),
            "h2" => Ok(Self::H2),
            "h3" => Ok(Self::H3),
            "h4" => Ok(Self::H4),
            "h5" => Ok(Self::H5),
            "h6" => Ok(Self::H6),
            "h7" => Ok(Self::H7),
            "h8" => Ok(Self::H8),
            _ => Err(ConversionError {
                input: value.to_string(),
                target: "Square".to_string(),
            }),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A1 => write!(f, "a1"),
            Self::A2 => write!(f, "a2"),
            Self::A3 => write!(f, "a3"),
            Self::A4 => write!(f, "a4"),
            Self::A5 => write!(f, "a5"),
            Self::A6 => write!(f, "a6"),
            Self::A7 => write!(f, "a7"),
            Self::A8 => write!(f, "a8"),
            Self::B1 => write!(f, "b1"),
            Self::B2 => write!(f, "b2"),
            Self::B3 => write!(f, "b3"),
            Self::B4 => write!(f, "b4"),
            Self::B5 => write!(f, "b5"),
            Self::B6 => write!(f, "b6"),
            Self::B7 => write!(f, "b7"),
            Self::B8 => write!(f, "b8"),
            Self::C1 => write!(f, "c1"),
            Self::C2 => write!(f, "c2"),
            Self::C3 => write!(f, "c3"),
            Self::C4 => write!(f, "c4"),
            Self::C5 => write!(f, "c5"),
            Self::C6 => write!(f, "c6"),
            Self::C7 => write!(f, "c7"),
            Self::C8 => write!(f, "c8"),
            Self::D1 => write!(f, "d1"),
            Self::D2 => write!(f, "d2"),
            Self::D3 => write!(f, "d3"),
            Self::D4 => write!(f, "d4"),
            Self::D5 => write!(f, "d5"),
            Self::D6 => write!(f, "d6"),
            Self::D7 => write!(f, "d7"),
            Self::D8 => write!(f, "d8"),
            Self::E1 => write!(f, "e1"),
            Self::E2 => write!(f, "e2"),
            Self::E3 => write!(f, "e3"),
            Self::E4 => write!(f, "e4"),
            Self::E5 => write!(f, "e5"),
            Self::E6 => write!(f, "e6"),
            Self::E7 => write!(f, "e7"),
            Self::E8 => write!(f, "e8"),
            Self::F1 => write!(f, "f1"),
            Self::F2 => write!(f, "f2"),
            Self::F3 => write!(f, "f3"),
            Self::F4 => write!(f, "f4"),
            Self::F5 => write!(f, "f5"),
            Self::F6 => write!(f, "f6"),
            Self::F7 => write!(f, "f7"),
            Self::F8 => write!(f, "f8"),
            Self::G1 => write!(f, "g1"),
            Self::G2 => write!(f, "g2"),
            Self::G3 => write!(f, "g3"),
            Self::G4 => write!(f, "g4"),
            Self::G5 => write!(f, "g5"),
            Self::G6 => write!(f, "g6"),
            Self::G7 => write!(f, "g7"),
            Self::G8 => write!(f, "g8"),
            Self::H1 => write!(f, "h1"),
            Self::H2 => write!(f, "h2"),
            Self::H3 => write!(f, "h3"),
            Self::H4 => write!(f, "h4"),
            Self::H5 => write!(f, "h5"),
            Self::H6 => write!(f, "h6"),
            Self::H7 => write!(f, "h7"),
            Self::H8 => write!(f, "h8"),
        }
    }
}
