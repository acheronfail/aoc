// See: https://adventofcode.com/2020/day/17
// ## --- Day 17: Conway Cubes ---
//
// As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the
// North Pole contact you. They'd like some help debugging a malfunctioning experimental energy
// source aboard one of their super-secret imaging satellites.
//
// The experimental energy source is based on cutting-edge technology: a set of Conway Cubes
// contained in a pocket dimension! When you hear it's having problems, you can't help but agree to
// take a look.
//
// The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional
// coordinate (`x,y,z`), there exists a single cube which is either *active* or *inactive*.
//
// In the initial state of the pocket dimension, almost all cubes start *inactive*. The only
// exception to this is a small flat region of cubes (your puzzle input); the cubes in this region
// start in the specified *active* (`#`) or *inactive* (`.`) state.
//
// The energy source then proceeds to boot up by executing six *cycles*.
//
// Each cube only ever considers its *neighbors*: any of the 26 other cubes where any of their
// coordinates differ by at most `1`. For example, given the cube at `x=1,y=2,z=3`, its neighbors
// include the cube at `x=2,y=2,z=2`, the cube at `x=0,y=2,z=3`, and so on.
//
// During a cycle, *all* cubes *simultaneously* change their state according to the following rules:
//
// * If a cube is *active* and *exactly `2` or `3`* of its neighbors are also active, the cube
// remains *active*. Otherwise, the cube becomes *inactive*.
// * If a cube is *inactive* but *exactly `3`* of its neighbors are active, the cube becomes
// *active*. Otherwise, the cube remains *inactive*.
//
// The engineers responsible for this experimental energy source would like you to simulate the
// pocket dimension and determine what the configuration of cubes should be at the end of the
// six-cycle boot process.
//
// For example, consider the following initial state:
//
// `.#.
// ..#
// ###
// `
//
// Even though the pocket dimension is 3-dimensional, this initial state represents a small
// 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the
// 3-dimensional space.)
//
// Simulating a few cycles from this initial state produces the following configurations, where the
// result of each cycle is shown layer-by-layer at each given `z` coordinate (and the frame of view
// follows the active cells in each cycle):
//
// `Before any cycles:
// z=0
// .#.
// ..#
// ###
// After 1 cycle:
// z=-1
// #..
// ..#
// .#.
// z=0
// #.#
// .##
// .#.
// z=1
// #..
// ..#
// .#.
// After 2 cycles:
// z=-2
// .....
// .....
// ..#..
// .....
// .....
// z=-1
// ..#..
// .#..#
// ....#
// .#...
// .....
// z=0
// ##...
// ##...
// #....
// ....#
// .###.
// z=1
// ..#..
// .#..#
// ....#
// .#...
// .....
// z=2
// .....
// .....
// ..#..
// .....
// .....
// After 3 cycles:
// z=-2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......
// z=-1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...
// z=0
// ...#...
// .......
// #......
// .......
// .....##
// .##.#..
// ...#...
// z=1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...
// z=2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......
// `
//
// After the full six-cycle boot process completes, *`112`* cubes are left in the *active* state.
//
// Starting with your given initial configuration, simulate six cycles. *How many cubes are left in
// the active state after the sixth cycle?*
//
//
// ## --- Part Two ---
//
// For some reason, your simulated results don't match what the experimental energy source engineers
// expected. Apparently, the pocket dimension actually has *four spatial dimensions*, not three.
//
// The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional
// coordinate (`x,y,z,w`), there exists a single cube (really, a *hypercube*) which is still either
// *active* or *inactive*.
//
// Each cube only ever considers its *neighbors*: any of the 80 other cubes where any of their
// coordinates differ by at most `1`. For example, given the cube at `x=1,y=2,z=3,w=4`, its
// neighbors include the cube at `x=2,y=2,z=3,w=3`, the cube at `x=0,y=2,z=3,w=4`, and so on.
//
// The initial state of the pocket dimension still consists of a small flat region of cubes.
// Furthermore, the same rules for cycle updating still apply: during each cycle, consider the
// *number of active neighbors* of each cube.
//
// For example, consider the same initial state as in the example above. Even though the pocket
// dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In
// particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)
//
// Simulating a few cycles from this initial state produces the following configurations, where the
// result of each cycle is shown layer-by-layer at each given `z` and `w` coordinate:
//
// `Before any cycles:
// z=0, w=0
// .#.
// ..#
// ###
// After 1 cycle:
// z=-1, w=-1
// #..
// ..#
// .#.
// z=0, w=-1
// #..
// ..#
// .#.
// z=1, w=-1
// #..
// ..#
// .#.
// z=-1, w=0
// #..
// ..#
// .#.
// z=0, w=0
// #.#
// .##
// .#.
// z=1, w=0
// #..
// ..#
// .#.
// z=-1, w=1
// #..
// ..#
// .#.
// z=0, w=1
// #..
// ..#
// .#.
// z=1, w=1
// #..
// ..#
// .#.
// After 2 cycles:
// z=-2, w=-2
// .....
// .....
// ..#..
// .....
// .....
// z=-1, w=-2
// .....
// .....
// .....
// .....
// .....
// z=0, w=-2
// ###..
// ##.##
// #...#
// .#..#
// .###.
// z=1, w=-2
// .....
// .....
// .....
// .....
// .....
// z=2, w=-2
// .....
// .....
// ..#..
// .....
// .....
// z=-2, w=-1
// .....
// .....
// .....
// .....
// .....
// z=-1, w=-1
// .....
// .....
// .....
// .....
// .....
// z=0, w=-1
// .....
// .....
// .....
// .....
// .....
// z=1, w=-1
// .....
// .....
// .....
// .....
// .....
// z=2, w=-1
// .....
// .....
// .....
// .....
// .....
// z=-2, w=0
// ###..
// ##.##
// #...#
// .#..#
// .###.
// z=-1, w=0
// .....
// .....
// .....
// .....
// .....
// z=0, w=0
// .....
// .....
// .....
// .....
// .....
// z=1, w=0
// .....
// .....
// .....
// .....
// .....
// z=2, w=0
// ###..
// ##.##
// #...#
// .#..#
// .###.
// z=-2, w=1
// .....
// .....
// .....
// .....
// .....
// z=-1, w=1
// .....
// .....
// .....
// .....
// .....
// z=0, w=1
// .....
// .....
// .....
// .....
// .....
// z=1, w=1
// .....
// .....
// .....
// .....
// .....
// z=2, w=1
// .....
// .....
// .....
// .....
// .....
// z=-2, w=2
// .....
// .....
// ..#..
// .....
// .....
// z=-1, w=2
// .....
// .....
// .....
// .....
// .....
// z=0, w=2
// ###..
// ##.##
// #...#
// .#..#
// .###.
// z=1, w=2
// .....
// .....
// .....
// .....
// .....
// z=2, w=2
// .....
// .....
// ..#..
// .....
// .....
// `
//
// After the full six-cycle boot process completes, *`848`* cubes are left in the *active* state.
//
// Starting with your given initial configuration, simulate six cycles in a 4-dimensional space.
// *How many cubes are left in the active state after the sixth cycle?*

use anyhow::Result;

fn transform3(state: Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    // extend the space so we can safely access all neighbours
    let z_len = state.len() + 2;
    let y_len = state[0].len() + 2;
    let x_len = state[0][0].len() + 2;
    let mut result = vec![vec![vec!['.'; x_len]; y_len]; z_len];

    // copy the existing state into the new space
    for (z, plane) in state.iter().enumerate() {
        let new_plane = &mut result[z + 1];
        for (y, line) in plane.iter().enumerate() {
            let new_line = &mut new_plane[y + 1];
            for (x, cube) in line.iter().enumerate() {
                new_line[x + 1] = *cube;
            }
        }
    }

    let transform_cube3 = |(x, y, z), cube| -> char {
        // check 26 surrounding cubes
        let mut active_neighbours = 0;
        {
            let (z, y, x) = (z as isize, y as isize, x as isize);
            for z2 in (z - 1)..=(z + 1) {
                for y2 in (y - 1)..=(y + 1) {
                    for x2 in (x - 1)..=(x + 1) {
                        match (z2, y2, x2) {
                            (z2, y2, x2) if z2 < 0 || y2 < 0 || x2 < 0 => continue,
                            (z2, y2, x2)
                                if z2 >= z_len as isize
                                    || y2 >= y_len as isize
                                    || x2 >= x_len as isize =>
                            {
                                continue
                            }
                            (z2, y2, x2) if z2 == z && y2 == y && x2 == x => continue,
                            _ => {
                                let (z, y, x) = (z2 as usize, y2 as usize, x2 as usize);
                                if result[z][y][x] == '#' {
                                    active_neighbours += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        match cube {
            '#' if active_neighbours != 2 && active_neighbours != 3 => '.',
            '.' if active_neighbours == 3 => '#',
            _ => cube,
        }
    };

    result
        .iter()
        .enumerate()
        .map(|(z, plane)| {
            plane
                .iter()
                .enumerate()
                .map(|(y, line)| {
                    line.iter()
                        .enumerate()
                        .map(|(x, cube)| transform_cube3((x, y, z), *cube))
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn cycle3(mut state: Vec<Vec<Vec<char>>>, count: usize) -> Vec<Vec<Vec<char>>> {
    for _ in 0..count {
        state = transform3(state);
    }

    state
}

fn transform4(state: Vec<Vec<Vec<Vec<char>>>>) -> Vec<Vec<Vec<Vec<char>>>> {
    // extend the space so we can safely access all neighbours
    let w_len = state.len() + 2;
    let z_len = state[0].len() + 2;
    let y_len = state[0][0].len() + 2;
    let x_len = state[0][0][0].len() + 2;
    let mut result = vec![vec![vec![vec!['.'; x_len]; y_len]; z_len]; w_len];

    // copy the existing state into the new space
    for (w, dimension) in state.iter().enumerate() {
        let new_dimension = &mut result[w + 1];
        for (z, plane) in dimension.iter().enumerate() {
            let new_plane = &mut new_dimension[z + 1];
            for (y, line) in plane.iter().enumerate() {
                let new_line = &mut new_plane[y + 1];
                for (x, cube) in line.iter().enumerate() {
                    new_line[x + 1] = *cube;
                }
            }
        }
    }

    let transform_cube4 = |(x, y, z, w), cube| -> char {
        // check 80 surrounding cubes
        let mut active_neighbours = 0;
        {
            let (w, z, y, x) = (w as isize, z as isize, y as isize, x as isize);
            for w2 in (w - 1)..=(w + 1) {
                for z2 in (z - 1)..=(z + 1) {
                    for y2 in (y - 1)..=(y + 1) {
                        for x2 in (x - 1)..=(x + 1) {
                            match (w2, z2, y2, x2) {
                                (w2, z2, y2, x2) if w2 < 0 || z2 < 0 || y2 < 0 || x2 < 0 => {
                                    continue
                                }
                                (w2, z2, y2, x2)
                                    if w2 >= w_len as isize
                                        || z2 >= z_len as isize
                                        || y2 >= y_len as isize
                                        || x2 >= x_len as isize =>
                                {
                                    continue
                                }
                                (w2, z2, y2, x2) if w2 == w && z2 == z && y2 == y && x2 == x => {
                                    continue
                                }
                                _ => {
                                    let (w, z, y, x) =
                                        (w2 as usize, z2 as usize, y2 as usize, x2 as usize);
                                    if result[w][z][y][x] == '#' {
                                        active_neighbours += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        match cube {
            '#' if active_neighbours != 2 && active_neighbours != 3 => '.',
            '.' if active_neighbours == 3 => '#',
            _ => cube,
        }
    };

    result
        .iter()
        .enumerate()
        .map(|(w, dimension)| {
            dimension
                .iter()
                .enumerate()
                .map(|(z, plane)| {
                    plane
                        .iter()
                        .enumerate()
                        .map(|(y, line)| {
                            line.iter()
                                .enumerate()
                                .map(|(x, cube)| transform_cube4((x, y, z, w), *cube))
                                .collect()
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn cycle4(mut state: Vec<Vec<Vec<Vec<char>>>>, count: usize) -> Vec<Vec<Vec<Vec<char>>>> {
    for _ in 0..count {
        state = transform4(state);
    }

    state
}

fn main() -> Result<()> {
    let input = include_str!("./input/2020-17.txt").trim();
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cubes_count = cycle3(vec![lines.clone()], 6)
        .into_iter()
        .map(|lines| {
            lines
                .into_iter()
                .map(|chars| chars.into_iter().filter(|ch| *ch == '#').count())
                .sum::<usize>()
        })
        .sum::<usize>();
    aoc_lib::set_part_1!(cubes_count);

    let cubes_count = cycle4(vec![vec![lines]], 6)
        .into_iter()
        .map(|planes| {
            planes
                .into_iter()
                .map(|lines| {
                    lines
                        .into_iter()
                        .map(|chars| chars.into_iter().filter(|ch| *ch == '#').count())
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    aoc_lib::set_part_2!(cubes_count);

    Ok(())
}
