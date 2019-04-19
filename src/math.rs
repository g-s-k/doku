pub fn get_row_num(idx: usize) -> usize {
    idx / 9
}

pub fn get_col_num(idx: usize) -> usize {
    idx % 9
}

pub fn get_box_num(idx: usize) -> usize {
    let box_row = idx / 27 * 3;
    let box_col = idx % 9 / 3;
    box_row + box_col
}

pub fn idx_to_row(idx: usize) -> impl Iterator<Item = usize> {
    let col_num = get_col_num(idx);
    (0..col_num)
        .chain(col_num + 1..9)
        .map(move |i| get_row_num(idx) * 9 + i)
}

pub fn idx_to_col(idx: usize) -> impl Iterator<Item = usize> {
    let row_num = get_row_num(idx);
    (0..row_num)
        .chain(row_num + 1..9)
        .map(move |i| i * 9 + get_col_num(idx))
}

pub fn idx_to_box(idx: usize) -> impl Iterator<Item = usize> {
    box_num(get_box_num(idx)).filter(move |i| *i != idx)
}

pub fn row_num(num: usize) -> impl Iterator<Item = usize> {
    (num * 9..).take(9)
}

pub fn col_num(num: usize) -> impl Iterator<Item = usize> {
    (0..81).skip(num).step_by(9)
}

pub fn box_num(num: usize) -> impl Iterator<Item = usize> {
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
