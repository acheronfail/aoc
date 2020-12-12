// See: https://adventofcode.com/2020/day/11
// ## --- Day 11: Seating System ---
//
// Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes
// directly to the tropical island where you can finally start your vacation. As you reach the
// waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!
//
// By modeling the process people use to choose (or abandon) their seat in the waiting area, you're
// pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your
// puzzle input).
//
// The seat layout fits neatly on a grid. Each position is either floor (`.`), an empty seat (`L`),
// or an occupied seat (`#`). For example, the initial seat layout might look like this:
//
// `L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL
// `
//
// Now, you just need to model the people who will be arriving shortly. Fortunately, people are
// entirely predictable and always follow a simple set of rules. All decisions are based on the
// *number of occupied seats* adjacent to a given seat (one of the eight positions immediately up,
// down, left, right, or diagonal from the seat). The following rules are applied to every seat
// simultaneously:
//
// * If a seat is *empty* (`L`) and there are *no* occupied seats adjacent to it, the seat becomes
// *occupied*.
// * If a seat is *occupied* (`#`) and *four or more* seats adjacent to it are also occupied, the
// seat becomes *empty*.
// * Otherwise, the seat's state does not change.
//
// Floor (`.`) never changes; seats don't move, and nobody sits on the floor.
//
// After one round of these rules, every seat in the example layout becomes occupied:
//
// `#.##.##.##
// #######.##
// #.#.#..#..
// ####.##.##
// #.##.##.##
// #.#####.##
// ..#.#.....
// ##########
// #.######.#
// #.#####.##
// `
//
// After a second round, the seats with four or more occupied adjacent seats become empty again:
//
// `#.LL.L#.##
// #LLLLLL.L#
// L.L.L..L..
// #LLL.LL.L#
// #.LL.LL.LL
// #.LLLL#.##
// ..L.L.....
// #LLLLLLLL#
// #.LLLLLL.L
// #.#LLLL.##
// `
//
// This process continues for three more rounds:
//
// `#.##.L#.##
// #L###LL.L#
// L.#.#..#..
// #L##.##.L#
// #.##.LL.LL
// #.###L#.##
// ..#.#.....
// #L######L#
// #.LL###L.L
// #.#L###.##
// `
// `#.#L.L#.##
// #LLL#LL.L#
// L.L.L..#..
// #LLL.##.L#
// #.LL.LL.LL
// #.LL#L#.##
// ..L.L.....
// #L#LLLL#L#
// #.LLLLLL.L
// #.#L#L#.##
// `
// `#.#L.L#.##
// #LLL#LL.L#
// L.#.L..#..
// #L##.##.L#
// #.#L.LL.LL
// #.#L#L#.##
// ..L.L.....
// #L#L##L#L#
// #.LLLLLL.L
// #.#L#L#.##
// `
//
// At this point, something interesting happens: the chaos stabilizes and further applications of
// these rules cause no seats to change state! Once people stop moving around, you count *`37`*
// occupied seats.
//
// Simulate your seating area by applying the seating rules repeatedly until no seats change state.
// *How many seats end up occupied?*
//
//
// ## --- Part Two ---
//
// As soon as people start to arrive, you realize your mistake. People don't just care about
// adjacent seats - they care about *the first seat they can see* in each of those eight directions!
//
// Now, instead of considering just the eight immediately adjacent seats, consider the *first seat*
// in each of those eight directions. For example, the empty seat below would see *eight* occupied
// seats:
//
// `.......#.
// ...#.....
// .#.......
// .........
// ..#L....#
// ....#....
// .........
// #........
// ...#.....
// `
//
// The leftmost empty seat below would only see *one* empty seat, but cannot see any of the occupied
// ones:
//
// `.............
// .L.L.#.#.#.#.
// .............
// `
//
// The empty seat below would see *no* occupied seats:
//
// `.##.##.
// #.#.#.#
// ##...##
// ...L...
// ##...##
// #.#.#.#
// .##.##.
// `
//
// Also, people seem to be more tolerant than you expected: it now takes *five or more* visible
// occupied seats for an occupied seat to become empty (rather than *four or more* from the previous
// rules). The other rules still apply: empty seats that see no occupied seats become occupied,
// seats matching no rule don't change, and floor never changes.
//
// Given the same starting layout as above, these new rules cause the seating area to shift around
// as follows:
//
// `L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL
// `
// `#.##.##.##
// #######.##
// #.#.#..#..
// ####.##.##
// #.##.##.##
// #.#####.##
// ..#.#.....
// ##########
// #.######.#
// #.#####.##
// `
// `#.LL.LL.L#
// #LLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLL#
// #.LLLLLL.L
// #.LLLLL.L#
// `
// `#.L#.##.L#
// #L#####.LL
// L.#.#..#..
// ##L#.##.##
// #.##.#L.##
// #.#####.#L
// ..#.#.....
// LLL####LL#
// #.L#####.L
// #.L####.L#
// `
// `#.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##LL.LL.L#
// L.LL.LL.L#
// #.LLLLL.LL
// ..L.L.....
// LLLLLLLLL#
// #.LLLLL#.L
// #.L#LL#.L#
// `
// `#.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##L#.#L.L#
// L.L#.#L.L#
// #.L####.LL
// ..#.#.....
// LLL###LLL#
// #.LLLLL#.L
// #.L#LL#.L#
// `
// `#.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##L#.#L.L#
// L.L#.LL.L#
// #.LLLL#.LL
// ..#.L.....
// LLL###LLL#
// #.LLLLL#.L
// #.L#LL#.L#
// `
//
// Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once
// this occurs, you count *`26`* occupied seats.
//
// Given the new visibility method and the rule change for occupied seats becoming empty, once
// equilibrium is reached, *how many seats end up occupied?*

use anyhow::Result;

fn step1(lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    lines
        .iter()
        .enumerate()
        .map(|(i, chars)| {
            chars
                .iter()
                .enumerate()
                .map(|(j, seat)| {
                    let occupied_seats = || -> usize {
                        let mut n = 0;

                        let k_start = if i == 0 { 0 } else { i - 1 };
                        let k_end = if i == lines.len() - 1 { i } else { i + 1 };
                        for k in k_start..=k_end {
                            let l_start = if j == 0 { 0 } else { j - 1 };
                            let l_end = if j == chars.len() - 1 { j } else { j + 1 };
                            for l in l_start..=l_end {
                                if k == i && l == j {
                                    continue;
                                }

                                if lines[k][l] == '#' {
                                    n += 1;
                                }
                            }
                        }

                        n
                    };

                    match seat {
                        'L' if occupied_seats() == 0 => '#',
                        '#' if occupied_seats() >= 4 => 'L',
                        _ => *seat,
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),  // l
    (-1, -1), // lu
    (0, -1),  // u
    (1, -1),  // ru
    (1, 0),   // r
    (1, 1),   // rd
    (0, 1),   // d
    (-1, 1),  // dl
];

fn step2(lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    lines
        .iter()
        .enumerate()
        .map(|(i, chars)| {
            chars
                .iter()
                .enumerate()
                .map(|(j, seat)| {
                    let occupied_seats = || -> usize {
                        let mut n = 0;
                        let line_range = 0..(lines.len() as isize);
                        let char_range = 0..(chars.len() as isize);
                        for (dx, dy) in &DIRECTIONS {
                            let (mut x, mut y) = (i as isize, j as isize);
                            loop {
                                x += dx;
                                y += dy;
                                if !line_range.contains(&x) || !char_range.contains(&y) {
                                    break;
                                }

                                match lines[x as usize][y as usize] {
                                    '.' => continue,
                                    c => {
                                        if c == '#' {
                                            n += 1;
                                        }

                                        break;
                                    }
                                }
                            }
                        }

                        n
                    };

                    match seat {
                        'L' if occupied_seats() == 0 => '#',
                        '#' if occupied_seats() >= 5 => 'L',
                        _ => *seat,
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

fn n_seats_once_stable(
    mut lines: Vec<Vec<char>>,
    f: fn(Vec<Vec<char>>) -> Vec<Vec<char>>,
) -> usize {
    let mut prev = None;
    while prev.is_none() || &lines != prev.as_ref().unwrap() {
        prev = Some(lines.clone());
        lines = f(lines);
    }

    lines.iter().fold(0, |count, chars| {
        chars.iter().filter(|ch| **ch == '#').count() + count
    })
}

fn main() -> Result<()> {
    let input = include_str!("./2020-11.txt").trim();
    let lines = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    aoc_lib::set_part_1!(n_seats_once_stable(lines.clone(), step1));
    aoc_lib::set_part_2!(n_seats_once_stable(lines.clone(), step2));

    Ok(())
}
