// See: https://adventofcode.com/2021/day/5
// ## --- Day 5: Hydrothermal Venture ---
//
// You come across a field of [hydrothermal vents][1] on the ocean floor! These vents constantly
// produce large, opaque clouds, so it would be best to avoid them if possible.
//
// They tend to form in *lines*; the submarine helpfully produces a list of nearby lines of vents
// (your puzzle input) for you to review. For example:
//
// `0,9 -> 5,9
// 8,0 -> 0,8
// 9,4 -> 3,4
// 2,2 -> 2,1
// 7,0 -> 7,4
// 6,4 -> 2,0
// 0,9 -> 2,9
// 3,4 -> 1,4
// 0,0 -> 8,8
// 5,5 -> 8,2
// `
//
// Each line of vents is given as a line segment in the format `x1,y1 -> x2,y2` where `x1`,`y1` are
// the coordinates of one end the line segment and `x2`,`y2` are the coordinates of the other end.
// These line segments include the points at both ends. In other words:
//
// * An entry like `1,1 -> 1,3` covers points `1,1`, `1,2`, and `1,3`.
// * An entry like `9,7 -> 7,7` covers points `9,7`, `8,7`, and `7,7`.
//
// For now, *only consider horizontal and vertical lines*: lines where either `x1 = x2` or `y1 =
// y2`.
//
// So, the horizontal and vertical lines from the above list would produce the following diagram:
//
// `.......1..
// ..1....1..
// ..1....1..
// .......1..
// .112111211
// ..........
// ..........
// ..........
// ..........
// 222111....
// `
//
// In this diagram, the top left corner is `0,0` and the bottom right corner is `9,9`. Each position
// is shown as *the number of lines which cover that point* or `.` if no line covers that point. The
// top-left pair of `1`s, for example, comes from `2,2 -> 2,1`; the very bottom row is formed by the
// overlapping lines `0,9 -> 5,9` and `0,9 -> 2,9`.
//
// To avoid the most dangerous areas, you need to determine *the number of points where at least two
// lines overlap*. In the above example, this is anywhere in the diagram with a `2` or larger - a
// total of `*5*` points.
//
// Consider only horizontal and vertical lines. *At how many points do at least two lines overlap?*
//
// [1] https://en.wikipedia.org/wiki/Hydrothermal_vent
//
//
// ## --- Part Two ---
//
// Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture;
// you need to also consider *diagonal lines*.
//
// Because of the limits of the hydrothermal vent mapping system, the lines in your list will only
// ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
//
// * An entry like `1,1 -> 3,3` covers points `1,1`, `2,2`, and `3,3`.
// * An entry like `9,7 -> 7,9` covers points `9,7`, `8,8`, and `7,9`.
//
// Considering all lines from the above example would now produce the following diagram:
//
// `1.1....11.
// .111...2..
// ..2.1.111.
// ...1.2.2..
// .112313211
// ...1.2....
// ..1...1...
// .1.....1..
// 1.......1.
// 222111....
// `
//
// You still need to determine *the number of points where at least two lines overlap*. In the above
// example, this is still anywhere in the diagram with a `2` or larger - now a total of `*12*`
// points.
//
// Consider all of the lines. *At how many points do at least two lines overlap?*

use anyhow::Result;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(i64, i64);

impl Point {
    pub fn line_between(&self, other: &Point) -> Vec<Point> {
        let mut points = vec![self.clone()];
        let mut cur = self.clone();
        loop {
            if cur.0 != other.0 {
                cur.0 = if cur.0 > other.0 {
                    cur.0 - 1
                } else {
                    cur.0 + 1
                };
            }
            if cur.1 != other.1 {
                cur.1 = if cur.1 > other.1 {
                    cur.1 - 1
                } else {
                    cur.1 + 1
                };
            }

            points.push(cur.clone());

            if cur.0 == other.0 && cur.1 == other.1 {
                break;
            }
        }

        points
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.0, self.1))
    }
}

fn main() -> Result<()> {
    let input = include_str!("./input/2021-05.txt")
        .trim()
        .lines()
        .map(|s| {
            s.split("->")
                .map(|s| {
                    s.trim()
                        .split(",")
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|v| Point(v[0], v[1]))
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .collect::<Vec<_>>();

    // only considering horizontal and vertical
    let h_and_v_lines = input
        .iter()
        .filter(|(a, b)| a.0 == b.0 || a.1 == b.1)
        .map(|(a, b)| a.line_between(b))
        .collect::<Vec<_>>();
    let mut frequency_map = HashMap::new();
    for points in h_and_v_lines {
        for point in points {
            *frequency_map.entry(point).or_insert(0) += 1;
        }
    }
    let two_or_more = frequency_map.iter().filter(|(_, n)| **n >= 2).count();
    aoc_lib::set_part_1!(two_or_more);

    // all lines
    let all_lines = input
        .iter()
        .map(|(a, b)| a.line_between(b))
        .collect::<Vec<_>>();

    let mut frequency_map = HashMap::new();
    for points in all_lines {
        for point in points {
            *frequency_map.entry(point).or_insert(0) += 1;
        }
    }
    let two_or_more = frequency_map.iter().filter(|(_, n)| **n >= 2).count();
    aoc_lib::set_part_2!(two_or_more);

    Ok(())
}
