use std::collections::BTreeSet;
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum Slot {
    Solved(Val),
    Unsolved(BTreeSet<Val>),
}

impl Default for Slot {
    fn default() -> Self {
        let mut b = BTreeSet::new();
        b.insert(Val::One);
        b.insert(Val::Two);
        b.insert(Val::Three);
        b.insert(Val::Four);
        b.insert(Val::Five);
        b.insert(Val::Six);
        b.insert(Val::Seven);
        b.insert(Val::Eight);
        b.insert(Val::Nine);
        Slot::Unsolved(b)
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Slot::Solved(v) => write!(f, "{}", v),
            _ => write!(f, "."),
        }
    }
}

impl Slot {
    fn is_solved(&self) -> bool {
        if let Slot::Solved(_) = self {
            true
        } else {
            false
        }
    }

    fn remove(&mut self, val: &Val) {
        if let Slot::Unsolved(vals) = self {
            vals.remove(val);
        }
    }

    fn simplify(&mut self) {
        if let Slot::Unsolved(vals) = self {
            if vals.len() == 1 {
                *self = Slot::Solved(*vals.iter().next().unwrap());
            }
        }
    }

    fn reduce_with_peer(&mut self, peer: &Slot) {
        if let Slot::Solved(v) = peer {
            self.remove(&v);
        }
        self.simplify();
    }
}

#[derive(Debug, Default)]
struct Puzzle([[Slot; 9]; 9]);

impl FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Puzzle::default();
        for (idx, c) in s.split_whitespace().enumerate() {
            if idx > 80 {
                return Err("Too many entries provided.");
            }

            if let Ok(v) = c.parse() {
                p.0[idx / 9][idx % 9] = Slot::Solved(v);
            }
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

                write!(f, "{} ", col)?;
            }

            writeln!(f, "|")?;
        }

        write!(f, "+-------+-------+-------+")
    }
}

impl Puzzle {
    fn solve(&mut self) {
        for row in 0..9 {
            self.reduce_row(row);

            for col in 0..9 {
                self.reduce_units(row, col);
            }
        }
    }

    fn reduce_row(&mut self, row: usize) {
        let mut to_remove = BTreeSet::new();

        for outer in 0..9 {
            if let Slot::Unsolved(vals) = &self.0[row][outer] {
                'inner: for inner in 0..9 {
                    if outer == inner {
                        continue 'inner;
                    }

                    if vals.len() == 2 && self.0[row][outer] == self.0[row][inner] {
                        to_remove.extend(vals.iter().cloned().map(|v| (v, outer, inner)));
                    }
                }
            }
        }

        for (entry, skip0, skip1) in to_remove {
            'cols: for col in 0..9 {
                if col == skip0 || col == skip1 {
                    continue 'cols;
                }

                self.0[row][col].remove(&entry);
                self.0[row][col].simplify();
            }
        }
    }

    fn reduce_units(&mut self, row: usize, col: usize) {
        if !self.0[row][col].is_solved() {
            let mut vals = self.0[row][col].clone();
            // reduce row unit
            for u_col in 0..9 {
                if u_col == col {
                    continue;
                }

                vals.reduce_with_peer(&self.0[row][u_col]);
            }

            // reduce column unit
            for u_row in 0..9 {
                if u_row == row {
                    continue;
                }

                vals.reduce_with_peer(&self.0[u_row][col]);
            }

            // reduce box unit
            let (last_r, last_c) = ((row / 3) * 3, (col / 3) * 3);
            for u_row in 0..3 {
                for u_col in 0..3 {
                    let (c_row, c_col) = (last_r + u_row, last_c + u_col);
                    if c_row == row && c_col == col {
                        continue;
                    }

                    vals.reduce_with_peer(&self.0[c_row][c_col]);
                }
            }

            self.0[row][col] = vals;
        }
    }
}

fn main() -> io::Result<()> {
    let mut bufr = String::with_capacity(512);
    io::stdin().read_to_string(&mut bufr)?;
    let mut puzzl = bufr.parse::<Puzzle>().unwrap();
    println!("Start:\n{}", puzzl);
    puzzl.solve();
    println!("\nEnd:\n{}", puzzl);
    Ok(())
}
