use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fmt;
use std::io::{self, Read};
use std::rc::Rc;
use std::str::FromStr;

const MAX_ITER: usize = 9;

fn get_row_num(idx: usize) -> usize {
    idx / 9
}

fn get_col_num(idx: usize) -> usize {
    idx % 9
}

fn get_box_num(idx: usize) -> usize {
    let box_row = idx / 27 * 3;
    let box_col = idx % 9 / 3;
    box_row + box_col
}

fn idx_to_row(idx: usize) -> impl Iterator<Item = usize> {
    let col_num = get_col_num(idx);
    (0..col_num)
        .chain(col_num + 1..9)
        .map(move |i| get_row_num(idx) * 9 + i)
}

fn idx_to_col(idx: usize) -> impl Iterator<Item = usize> {
    let row_num = get_row_num(idx);
    (0..row_num)
        .chain(row_num + 1..9)
        .map(move |i| i * 9 + get_col_num(idx))
}

fn idx_to_box(idx: usize) -> impl Iterator<Item = usize> {
    box_num(get_box_num(idx)).filter(move |i| *i != idx)
}

fn row_num(num: usize) -> impl Iterator<Item = usize> {
    (num * 9..).take(9)
}

fn col_num(num: usize) -> impl Iterator<Item = usize> {
    (0..81).skip(num).step_by(9)
}

fn box_num(num: usize) -> impl Iterator<Item = usize> {
    let row_start = num / 3 * 3;
    let col_start = num * 3 % 9;
    let idx_start = row_start * 9 + col_start;
    (idx_start..)
        .take(3)
        .chain(idx_start + 9..)
        .take(6)
        .chain(idx_start + 18..)
        .take(9)
}

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

impl Val {
    fn all() -> impl Iterator<Item = Self> {
        vec![
            Val::One,
            Val::Two,
            Val::Three,
            Val::Four,
            Val::Five,
            Val::Six,
            Val::Seven,
            Val::Eight,
            Val::Nine,
        ]
        .into_iter()
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

#[derive(Default)]
struct Cell {
    val: Option<Val>,
    not: BTreeSet<Val>,
    row: CellList,
    col: CellList,
    r#box: CellList,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cell({:?})", self.val)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(v) = self.val {
            write!(f, "{}", v)
        } else {
            write!(f, ".")
        }
    }
}

impl Cell {
    fn is_solved(&self) -> bool {
        self.val.is_some()
    }

    fn possible_values(&self) -> BTreeSet<Val> {
        let mut s = BTreeSet::new();

        if let Some(v) = self.val {
            s.insert(v);
            return s;
        }

        s.insert(Val::One);
        s.insert(Val::Two);
        s.insert(Val::Three);
        s.insert(Val::Four);
        s.insert(Val::Five);
        s.insert(Val::Six);
        s.insert(Val::Seven);
        s.insert(Val::Eight);
        s.insert(Val::Nine);

        for val in &self.not {
            s.remove(val);
        }

        for peer in self
            .row
            .iter()
            .chain(self.col.iter())
            .chain(self.r#box.iter())
        {
            if let Some(v) = peer.borrow().val {
                s.remove(&v);
            }

            if s.is_empty() {
                panic!("Every cell should have at least one possible value.");
            }
        }

        s
    }
}

type CellList = Vec<Rc<RefCell<Cell>>>;

#[derive(Debug)]
struct Puzzle(CellList);

impl Default for Puzzle {
    fn default() -> Self {
        let mut v = Vec::new();
        for _ in 0..81 {
            v.push(Rc::new(RefCell::new(Cell::default())));
        }

        for (idx, cell) in v.iter().enumerate() {
            let mut c = cell.borrow_mut();

            for row_idx in idx_to_row(idx) {
                c.row.push(v[row_idx].clone());
            }

            for col_idx in idx_to_col(idx) {
                c.col.push(v[col_idx].clone());
            }

            for box_idx in idx_to_box(idx) {
                c.r#box.push(v[box_idx].clone());
            }
        }

        Self(v)
    }
}

impl FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = Self::default();

        for (idx, tok) in s.split_whitespace().enumerate() {
            if idx == 81 {
                return Err("More than 81 tokens provided.");
            }

            out.0[idx].borrow_mut().val = tok.parse().ok();
        }

        Ok(out)
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..9 {
            if row % 3 == 0 {
                writeln!(f, "+-------+-------+-------+")?;
            }

            for offset in 0..3 {
                let sl = &self.0[row * 9 + offset * 3..];
                write!(
                    f,
                    "| {} {} {} ",
                    sl[0].borrow(),
                    sl[1].borrow(),
                    sl[2].borrow()
                )?;
            }

            writeln!(f, "|")?;
        }

        write!(f, "+-------+-------+-------+")
    }
}

impl Puzzle {
    fn solved_count(&self) -> usize {
        self.0
            .iter()
            .filter(|cell| cell.borrow().is_solved())
            .count()
    }

    fn promote(&mut self) {
        for cell in &self.0 {
            let mut cell = cell.borrow_mut();
            if !cell.is_solved() {
                let p = cell.possible_values();
                if p.len() == 1 {
                    cell.val = p.into_iter().next();
                }
            }
        }
    }

    fn candidates<I: Iterator<Item = usize>>(&self, indices: I, val: Val) -> Vec<usize> {
        indices
            .filter(|&idx| {
                let c = self.0[idx].borrow();
                !c.is_solved() && c.possible_values().contains(&val)
            })
            .collect()
    }

    fn try_solve(&mut self) -> Result<usize, usize> {
        let mut num_solved = self.solved_count();

        for iter_num in 0..MAX_ITER {
            self.promote();

            for val in Val::all() {
                for unit in 0..9 {
                    let row_p = self.candidates(row_num(unit), val);
                    match row_p.len() {
                        1 => {
                            self.0[row_p[0]].borrow_mut().val = Some(val);
                        }
                        2 | 3 => {
                            if row_p
                                .iter()
                                .cloned()
                                .map(get_box_num)
                                .collect::<BTreeSet<_>>()
                                .len()
                                == 1
                            {
                                for inner_idx in idx_to_box(row_p[0]).filter(|i| !row_p.contains(i))
                                {
                                    self.0[inner_idx].borrow_mut().not.insert(val);
                                }
                            }
                        }
                        _ => (),
                    }

                    let col_p = self.candidates(col_num(unit), val);
                    match col_p.len() {
                        1 => {
                            self.0[col_p[0]].borrow_mut().val = Some(val);
                        }
                        2 | 3 => {
                            if col_p
                                .iter()
                                .cloned()
                                .map(get_box_num)
                                .collect::<BTreeSet<_>>()
                                .len()
                                == 1
                            {
                                for inner_idx in idx_to_box(col_p[0]).filter(|i| !col_p.contains(i))
                                {
                                    self.0[inner_idx].borrow_mut().not.insert(val);
                                }
                            }
                        }
                        _ => (),
                    }

                    let box_p = self.candidates(box_num(unit), val);
                    match box_p.len() {
                        1 => {
                            self.0[box_p[0]].borrow_mut().val = Some(val);
                        }
                        2 | 3 => {
                            if box_p
                                .iter()
                                .cloned()
                                .map(get_row_num)
                                .collect::<BTreeSet<_>>()
                                .len()
                                == 1
                            {
                                for inner_idx in idx_to_row(box_p[0]).filter(|i| !box_p.contains(i))
                                {
                                    self.0[inner_idx].borrow_mut().not.insert(val);
                                }
                            } else if box_p
                                .iter()
                                .cloned()
                                .map(get_col_num)
                                .collect::<BTreeSet<_>>()
                                .len()
                                == 1
                            {
                                for inner_idx in idx_to_col(box_p[0]).filter(|i| !box_p.contains(i))
                                {
                                    self.0[inner_idx].borrow_mut().not.insert(val);
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }

            self.promote();

            num_solved = self.solved_count();

            if num_solved == 81 {
                return Ok(iter_num + 1);
            }
        }

        Err(num_solved)
    }
}

fn main() {
    let mut buffer = String::with_capacity(512);
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read to string.");
    let mut puzzl = buffer.parse::<Puzzle>().unwrap();
    eprintln!(
        "Start: {} cells filled in.\n{}",
        puzzl.solved_count(),
        puzzl
    );
    match puzzl.try_solve() {
        Ok(iters) => eprintln!("Solved it in {} iterations.", iters),
        Err(count) => eprintln!(
            "Couldn't solve it in {} iterations. Only {} cells are solved.",
            MAX_ITER, count
        ),
    }
    eprintln!("{}", puzzl);
}
