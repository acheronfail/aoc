fn num_trees_encountered(map: &str, slope: (usize, usize)) -> usize {
    let max = map.lines().count();
    let mut trees_encountered = 0;
    let mut pos = (0, 0);
    while pos.1 < max {
        let current_line = map
            .lines()
            .nth(pos.1)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();
        if current_line[pos.0 % current_line.len()] == '#' {
            trees_encountered += 1;
        }

        pos.0 += slope.0;
        pos.1 += slope.1;
    }

    trees_encountered
}

fn main() {
    let map = include_str!("./3.txt");

    aoc_lib::set_part_1!(num_trees_encountered(map, (3, 1)));
    aoc_lib::set_part_2!(
        num_trees_encountered(map, (1, 1))
            * num_trees_encountered(map, (3, 1))
            * num_trees_encountered(map, (5, 1))
            * num_trees_encountered(map, (7, 1))
            * num_trees_encountered(map, (1, 2))
    );
}
