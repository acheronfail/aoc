// AoC 2020 5

fn boarding_pass_to_rowcol(pass: &str) -> (usize, usize) {
    let mut row = 0..128_usize;
    let mut col = 0..8_usize;

    for ch in pass.chars() {
        match ch {
            'F' => row = row.start..((row.start + row.end) / 2),
            'B' => row = ((row.start + row.end) / 2)..row.end,
            'L' => col = col.start..((col.start + col.end) / 2),
            'R' => col = ((col.start + col.end) / 2)..col.end,
            _ => {}
        }
    }

    (row.start, col.start)
}

fn main() {
    let input = include_str!("./5.txt");

    use std::collections::HashSet;
    let mut set: HashSet<usize> = HashSet::new();

    let mut max_id = 0;
    for line in input.lines() {
        let (row, col) = boarding_pass_to_rowcol(line);
        let id = row * 8 + col;
        if id > max_id {
            max_id = id;
        }

        set.insert(id);
    }

    aoc_lib::set_part_1!(max_id);

    let mut ids = set.drain().collect::<Vec<usize>>();
    ids.sort();

    for (i, n) in ids[0..ids.len() - 1].iter().enumerate() {
        let m = ids[i + 1];
        if m - n > 1 {
            aoc_lib::set_part_2!(n + 1);
        }
    }
}
