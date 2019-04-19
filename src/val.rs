use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Val {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl FromStr for Val {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "1" | "a" | "one" => Ok(Val::One),
            "2" | "b" | "two" => Ok(Val::Two),
            "3" | "c" | "three" => Ok(Val::Three),
            "4" | "d" | "four" => Ok(Val::Four),
            "5" | "e" | "five" => Ok(Val::Five),
            "6" | "f" | "six" => Ok(Val::Six),
            "7" | "g" | "seven" => Ok(Val::Seven),
            "8" | "h" | "eight" => Ok(Val::Eight),
            "9" | "i" | "nine" => Ok(Val::Nine),
            _ => Err("Could not parse value from string"),
        }
    }
}

impl TryFrom<u32> for Val {
    type Error = &'static str;

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Val::One),
            2 => Ok(Val::Two),
            3 => Ok(Val::Three),
            4 => Ok(Val::Four),
            5 => Ok(Val::Five),
            6 => Ok(Val::Six),
            7 => Ok(Val::Seven),
            8 => Ok(Val::Eight),
            9 => Ok(Val::Nine),
            _ => Err("Not a valid Sudoku number."),
        }
    }
}

impl Val {
    pub(crate) fn all() -> impl Iterator<Item = Self> {
        (1..=9).map(Self::try_from).filter_map(Result::ok)
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u32)
    }
}
