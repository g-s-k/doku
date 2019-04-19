//! Print and solve Sudoku puzzles.

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

/// The maximum number of times to try solving a puzzle before giving up.
pub const MAX_ITER: usize = 9;

fn uniq_by_unit<'a, I>(i: I, f: fn(usize) -> usize) -> bool
where
    I: IntoIterator<Item = &'a usize>,
{
    i.into_iter().cloned().map(f).collect::<BTreeSet<_>>().len() == 1
}

/// A type that can represent an index into a [`Puzzle`](struct.Puzzle.html).
pub trait PuzzleIndex {
    /// Compute a row-major index into the underlying array of cells.
    fn as_puzzle_index(&self) -> usize;
}

impl PuzzleIndex for usize {
    fn as_puzzle_index(&self) -> usize {
        *self
    }
}

impl PuzzleIndex for (usize, usize) {
    fn as_puzzle_index(&self) -> usize {
        self.0 * 9 + self.1
    }
}

impl PuzzleIndex for (Val, Val) {
    fn as_puzzle_index(&self) -> usize {
        self.0 as usize * 9 + self.1 as usize
    }
}

/// An entire Sudoku puzzle.
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
    /// Set the value of a cell at a given `(row, column)` index.
    pub fn set<T: PuzzleIndex>(&mut self, index: T, val: Option<Val>) {
        self.0[index.as_puzzle_index()].borrow_mut().val = val;
    }

    pub fn set_not<T: PuzzleIndex>(&mut self, index: T, val: Val) {
        self.0[index.as_puzzle_index()].borrow_mut().not.insert(val);
    }

    /// Number of cells presently filled in.
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

    /// Attempt to find a solution to the puzzle.
    ///
    /// If a solution is found, `Ok(n_iterations)` is returned, where `n_iterations`
    /// is the number of iterations it took to complete the solution. If the [maximum
    /// number of iterations](constant.MAX_ITER.html) is reached without finding
    /// a solution, `Err(n_solved)` is returned, where `n_solved` is the number
    /// of cells that were filled in during the attempt.
    pub fn try_solve(&mut self) -> Result<usize, usize> {
        let orig_solved = self.solved_count();
        let mut num_solved = orig_solved;

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
                            if uniq_by_unit(&row_p, get_box_num) {
                                for inner_idx in idx_to_box(row_p[0]).filter(|i| !row_p.contains(i))
                                {
                                    self.set_not(inner_idx, val)
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
                            if uniq_by_unit(&col_p, get_box_num) {
                                for inner_idx in idx_to_box(col_p[0]).filter(|i| !col_p.contains(i))
                                {
                                    self.set_not(inner_idx, val)
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
                            if uniq_by_unit(&box_p, get_row_num) {
                                for inner_idx in idx_to_row(box_p[0]).filter(|i| !box_p.contains(i))
                                {
                                    self.set_not(inner_idx, val)
                                }
                            } else if uniq_by_unit(&box_p, get_col_num) {
                                for inner_idx in idx_to_col(box_p[0]).filter(|i| !box_p.contains(i))
                                {
                                    self.set_not(inner_idx, val)
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

        Err(num_solved - orig_solved)
    }
}
