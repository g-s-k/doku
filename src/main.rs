use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

const MAX_ITER: usize = 25;

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

impl Val {
    fn all() -> BTreeSet<Self> {
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
        b
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Slot {
    Solved(Val),
    Unsolved(BTreeSet<Val>),
}

impl Default for Slot {
    fn default() -> Self {
        Slot::Unsolved(Val::all())
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
    fn len(&self) -> usize {
        if let Slot::Unsolved(vals) = self {
            vals.len()
        } else {
            1
        }
    }

    fn difference(&self, other: &Self) -> BTreeSet<Val> {
        match (self, other) {
            (Slot::Unsolved(s_vals), Slot::Unsolved(o_vals)) => {
                s_vals.difference(o_vals).cloned().collect()
            }
            (Slot::Unsolved(vals), Slot::Solved(val)) => {
                let mut out = vals.clone();
                out.remove(val);
                out
            }
            _ => BTreeSet::new(),
        }
    }

    fn union(&self, other: &Self) -> BTreeSet<Val> {
        match (self, other) {
            (Slot::Unsolved(s_vals), Slot::Unsolved(o_vals)) => {
                s_vals.union(o_vals).cloned().collect()
            }
            (Slot::Unsolved(vals), _) | (_, Slot::Unsolved(vals)) => vals.clone(),
            _ => BTreeSet::new(),
        }
    }

    fn remove(&mut self, val: &Val) {
        if let Slot::Unsolved(vals) = self {
            vals.remove(val);
        }
    }

    fn remove_all(&mut self, peer: &Self) {
        if let Slot::Unsolved(vals) = peer {
            for val in vals {
                self.remove(val);
            }
        }
    }

    fn simplify(&mut self) {
        if let Slot::Unsolved(vals) = self {
            if vals.len() == 1 {
                *self = Slot::Solved(*vals.iter().next().unwrap());
            }
        }
    }

    fn replace_with(&mut self, val: Val) {
        *self = Slot::Solved(val);
    }

    fn reduce_with_peer(&mut self, peer: &Self) {
        if let Slot::Solved(v) = peer {
            self.remove(&v);
        }
    }
}

fn reduce_unit(unit: &[RefCell<&mut Slot>]) {
    let scrub_unit = |union: BTreeSet<Val>, skips: &[usize]| {
        let to_remove = Slot::Unsolved(union);
        for idx in (0..unit.len()).filter(|i| !skips.contains(i)) {
            let mut current = unit[idx].borrow_mut();
            current.remove_all(&to_remove);
            current.simplify();
        }
    };

    for first in 0..unit.len() {
        // find values which uniquely belong to this space
        let mut uniqs = Val::all();
        {
            let o = unit[first].borrow();
            for second in (0..unit.len()).filter(|i| *i != first) {
                let i = unit[second].borrow();
                let diff = o.difference(&i);
                uniqs = uniqs.intersection(&diff).cloned().collect();
            }
        }
        if uniqs.len() == 1 {
            unit[first]
                .borrow_mut()
                .replace_with(uniqs.into_iter().next().unwrap());
        }

        for second in (0..unit.len()).filter(|i| *i != first) {
            // remove definitive peers and try to simplify
            let i = unit[second].borrow();
            {
                let mut o = unit[first].borrow_mut();
                o.reduce_with_peer(&i);
                o.simplify();
            }

            // check for uniqueness of a given possibility

            // // check for naked pairs
            // let o = unit[first].borrow();
            // if o.len() == 2 && *o == *i {
            //     scrub_unit(o.union(&i), &[first, second]);
            // }

            // // check for naked triples
            // for third in (0..unit.len()).filter(|i| ![first, second].contains(i)) {
            //     let t = unit[third].borrow();
            //     let u = Slot::Unsolved(o.union(&i)).union(&t);
            //     if u.len() == 3 {
            //         scrub_unit(u, &[first, second, third]);
            //     }

            //     // and naked quads
            //     for fourth in (0..unit.len()).filter(|i| ![first, second, third].contains(i)) {
            //         let f = unit[fourth].borrow();
            //         let u = Slot::Unsolved(Slot::Unsolved(o.union(&i)).union(&t)).union(&f);
            //         if u.len() == 4 {
            //             scrub_unit(u, &[first, second, third, fourth]);
            //         }
            //     }
            // }
        }
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
    fn is_solved(&self) -> bool {
        self.0
            .iter()
            .flat_map(|row| row.iter())
            .all(|c| c.len() == 1)
    }

    fn solve(&mut self) {
        for cell in self.0.iter_mut().flat_map(|row| row.iter_mut()) {
            cell.simplify();
        }

        for iter_num in 0..MAX_ITER {
            for idx in 0..9 {
                // reduce row
                let row_v = self.0[idx].iter_mut().map(RefCell::new).collect::<Vec<_>>();
                reduce_unit(&row_v);

                // reduce column
                let col_v = self
                    .0
                    .iter_mut()
                    .flat_map(|r| r.iter_mut().skip(idx).take(1))
                    .map(RefCell::new)
                    .collect::<Vec<_>>();
                reduce_unit(&col_v);

                // reduce box
                let (r_start, c_start) = ((idx / 3) * 3, (idx % 3) * 3);
                let box_v = self
                    .0
                    .iter_mut()
                    .skip(r_start)
                    .take(3)
                    .flat_map(|r| r.iter_mut().skip(c_start).take(3))
                    .map(RefCell::new)
                    .collect::<Vec<_>>();
                reduce_unit(&box_v);
            }

            if self.is_solved() {
                eprintln!("Solved in {} iterations!", iter_num);
                return;
            }
        }

        eprintln!("Gave up after {} iterations.", MAX_ITER);
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
