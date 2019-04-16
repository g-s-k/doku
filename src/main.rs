use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Val {
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
            "3" | "c" | "Three" => Ok(Val::Three),
            "4" | "d" | "Four" => Ok(Val::Four),
            "5" | "e" | "Five" => Ok(Val::Five),
            "6" | "f" | "Six" => Ok(Val::Six),
            "7" | "g" | "Seven" => Ok(Val::Seven),
            "8" | "h" | "Eight" => Ok(Val::Eight),
            "9" | "i" | "Nine" => Ok(Val::Nine),
            _ => Err("Could not parse value from string"),
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

#[derive(Debug)]
struct Puzzle([[Option<Val>; 9]; 9]);

impl FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Puzzle([[None; 9]; 9]);
        for (idx, c) in s.split_whitespace().enumerate() {
            if idx > 80 {
                return Err("Too many entries provided.")
            }

            p.0[idx / 9][idx % 9] = c.parse().ok();
        }
        Ok(p)
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (ir, row) in self.0.iter().enumerate() {
            if ir % 3 == 0 {
                writeln!(f, "+-------+-------+-------+")?;
            }

            for (ic, col) in row.iter().enumerate() {
                if ic % 3 == 0 {
                    write!(f, "| ")?;
                }

                if let Some(v) = col {
                    write!(f, "{} ", v)?;
                } else {
                    write!(f, ". ")?;
                }
            }

            writeln!(f, "|")?;
        }

        write!(f, "+-------+-------+-------+")
    }
}

fn main() -> io::Result<()> {
    let mut bufr = String::with_capacity(256);
    io::stdin().read_to_string(&mut bufr)?;
    let puzzl = bufr.parse::<Puzzle>().unwrap();
    println!("Start:\n{}", puzzl);
    Ok(())
}
