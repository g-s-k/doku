#![warn(clippy::pedantic)]

use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

mod cell;
mod math;
mod val;

use self::cell::List;
use self::math::*;
pub use self::val::Val;

pub const MAX_ITER: usize = 9;

#[derive(Debug)]
pub struct Puzzle(List);

impl Default for Puzzle {
    fn default() -> Self {
        Self(cell::new_cell_list())
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
    pub fn set(&mut self, (row, col): (usize, usize), val: Option<Val>) {
        self.0[row * 9 + col].borrow_mut().val = val.into();
    }

    pub fn solved_count(&self) -> usize {
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

    pub fn try_solve(&mut self) -> Result<usize, usize> {
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
