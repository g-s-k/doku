use std::io::{self, Read};

use doku::{Puzzle, MAX_ITER};

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
