use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fmt;
use std::iter;
use std::rc::Rc;

use super::{math::*, Val};

pub type CellList = Vec<Rc<RefCell<Cell>>>;

pub fn new_cell_list() -> CellList {
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

    v
}

#[derive(Default)]
pub struct Cell {
    pub val: Option<Val>,
    pub not: BTreeSet<Val>,
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
    pub fn is_solved(&self) -> bool {
        self.val.is_some()
    }

    pub fn possible_values(&self) -> BTreeSet<Val> {
        if let Some(v) = self.val {
            return iter::once(v).collect();
        }

        let mut s = Val::all().collect::<BTreeSet<_>>();

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
