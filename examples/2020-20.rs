// See: https://adventofcode.com/2020/day/20
// ## --- Day 20: Jurassic Jigsaw ---
//
// The high-speed train leaves the forest and quickly carries you south. You can even see a desert
// in the distance! Since you have some spare time, you might as well see if there was anything
// interesting in the image the Mythical Information Bureau satellite captured.
//
// After decoding the satellite messages, you discover that the data actually contains many small
// images created by the satellite's *camera array*. The camera array consists of many cameras;
// rather than produce a single square image, they produce many smaller square image *tiles* that
// need to be *reassembled back into a single image*.
//
// Each camera in the camera array returns a single monochrome *image tile* with a random unique *ID
// number*. The tiles (your puzzle input) arrived in a random order.
//
// Worse yet, the camera array appears to be malfunctioning: each image tile has been *rotated and
// flipped to a random orientation*. Your first task is to reassemble the original image by
// orienting the tiles so they fit together.
//
// To show how the tiles should be reassembled, each tile's image data includes a border that should
// line up exactly with its adjacent tiles. All tiles have this border, and the border lines up
// exactly when the tiles are both oriented correctly. Tiles at the edge of the image also have this
// border, but the outermost edges won't line up with any other tiles.
//
// For example, suppose you have the following nine tiles:
//
// `Tile 2311:
// ..##.#..#.
// ##..#.....
// #...##..#.
// ####.#...#
// ##.##.###.
// ##...#.###
// .#.#.#..##
// ..#....#..
// ###...#.#.
// ..###..###
// Tile 1951:
// #.##...##.
// #.####...#
// .....#..##
// #...######
// .##.#....#
// .###.#####
// ###.##.##.
// .###....#.
// ..#.#..#.#
// #...##.#..
// Tile 1171:
// ####...##.
// #..##.#..#
// ##.#..#.#.
// .###.####.
// ..###.####
// .##....##.
// .#...####.
// #.##.####.
// ####..#...
// .....##...
// Tile 1427:
// ###.##.#..
// .#..#.##..
// .#.##.#..#
// #.#.#.##.#
// ....#...##
// ...##..##.
// ...#.#####
// .#.####.#.
// ..#..###.#
// ..##.#..#.
// Tile 1489:
// ##.#.#....
// ..##...#..
// .##..##...
// ..#...#...
// #####...#.
// #..#.#.#.#
// ...#.#.#..
// ##.#...##.
// ..##.##.##
// ###.##.#..
// Tile 2473:
// #....####.
// #..#.##...
// #.##..#...
// ######.#.#
// .#...#.#.#
// .#########
// .###.#..#.
// ########.#
// ##...##.#.
// ..###.#.#.
// Tile 2971:
// ..#.#....#
// #...###...
// #.#.###...
// ##.##..#..
// .#####..##
// .#..####.#
// #..#.#..#.
// ..####.###
// ..#.#.###.
// ...#.#.#.#
// Tile 2729:
// ...#.#.#.#
// ####.#....
// ..#.#.....
// ....#..#.#
// .##..##.#.
// .#.####...
// ####.#.#..
// ##.####...
// ##..#.##..
// #.##...##.
// Tile 3079:
// #.#.#####.
// .#..######
// ..#.......
// ######....
// ####.#..#.
// .#...#.##.
// #.#####.##
// ..#.###...
// ..#.......
// ..#.###...
// `
//
// By rotating, flipping, and rearranging them, you can find a square arrangement that causes all
// adjacent borders to line up:
//
// `#...##.#.. ..###..### #.#.#####.
// ..#.#..#.# ###...#.#. .#..######
// .###....#. ..#....#.. ..#.......
// ###.##.##. .#.#.#..## ######....
// .###.##### ##...#.### ####.#..#.
// .##.#....# ##.##.###. .#...#.##.
// #...###### ####.#...# #.#####.##
// .....#..## #...##..#. ..#.###...
// #.####...# ##..#..... ..#.......
// #.##...##. ..##.#..#. ..#.###...
// #.##...##. ..##.#..#. ..#.###...
// ##..#.##.. ..#..###.# ##.##....#
// ##.####... .#.####.#. ..#.###..#
// ####.#.#.. ...#.##### ###.#..###
// .#.####... ...##..##. .######.##
// .##..##.#. ....#...## #.#.#.#...
// ....#..#.# #.#.#.##.# #.###.###.
// ..#.#..... .#.##.#..# #.###.##..
// ####.#.... .#..#.##.. .######...
// ...#.#.#.# ###.##.#.. .##...####
// ...#.#.#.# ###.##.#.. .##...####
// ..#.#.###. ..##.##.## #..#.##..#
// ..####.### ##.#...##. .#.#..#.##
// #..#.#..#. ...#.#.#.. .####.###.
// .#..####.# #..#.#.#.# ####.###..
// .#####..## #####...#. .##....##.
// ##.##..#.. ..#...#... .####...#.
// #.#.###... .##..##... .####.##.#
// #...###... ..##...#.. ...#..####
// ..#.#....# ##.#.#.... ...##.....
// `
//
// For reference, the IDs of the above tiles are:
//
// `*1951*    2311    *3079*
// 2729    1427    2473
// *2971*    1489    *1171*
// `
//
// To check that you've assembled the image correctly, multiply the IDs of the four corner tiles
// together. If you do this with the assembled tiles from the example above, you get `1951 * 3079 *
// 2971 * 1171` = *`20899048083289`*.
//
// Assemble the tiles into an image. *What do you get if you multiply together the IDs of the four
// corner tiles?*
//
//
// ## --- Part Two ---
//
// Now, you're ready to *check the image for sea monsters*.
//
// The borders of each tile are not part of the actual image; start by removing them.
//
// In the example above, the tiles become:
//
// `.#.#..#. ##...#.# #..#####
// ###....# .#....#. .#......
// ##.##.## #.#.#..# #####...
// ###.#### #...#.## ###.#..#
// ##.#.... #.##.### #...#.##
// ...##### ###.#... .#####.#
// ....#..# ...##..# .#.###..
// .####... #..#.... .#......
// #..#.##. .#..###. #.##....
// #.####.. #.####.# .#.###..
// ###.#.#. ..#.#### ##.#..##
// #.####.. ..##..## ######.#
// ##..##.# ...#...# .#.#.#..
// ...#..#. .#.#.##. .###.###
// .#.#.... #.##.#.. .###.##.
// ###.#... #..#.##. ######..
// .#.#.### .##.##.# ..#.##..
// .####.## #.#...## #.#..#.#
// ..#.#..# ..#.#.#. ####.###
// #..####. ..#.#.#. ###.###.
// #####..# ####...# ##....##
// #.##..#. .#...#.. ####...#
// .#.###.. ##..##.. ####.##.
// ...###.. .##...#. ..#..###
// `
//
// Remove the gaps to form the actual image:
//
// `.#.#..#.##...#.##..#####
// ###....#.#....#..#......
// ##.##.###.#.#..######...
// ###.#####...#.#####.#..#
// ##.#....#.##.####...#.##
// ...########.#....#####.#
// ....#..#...##..#.#.###..
// .####...#..#.....#......
// #..#.##..#..###.#.##....
// #.####..#.####.#.#.###..
// ###.#.#...#.######.#..##
// #.####....##..########.#
// ##..##.#...#...#.#.#.#..
// ...#..#..#.#.##..###.###
// .#.#....#.##.#...###.##.
// ###.#...#..#.##.######..
// .#.#.###.##.##.#..#.##..
// .####.###.#...###.#..#.#
// ..#.#..#..#.#.#.####.###
// #..####...#.#.#.###.###.
// #####..#####...###....##
// #.##..#..#...#..####...#
// .#.###..##..##..####.##.
// ...###...##...#...#..###
// `
//
// Now, you're ready to search for sea monsters! Because your image is monochrome, a sea monster
// will look like this:
//
// `                  #
// #    ##    ##    ###
// #  #  #  #  #  #
// `
//
// When looking for this pattern in the image, *the spaces can be anything*; only the `#` need to
// match. Also, you might need to rotate or flip your image before it's oriented correctly to find
// sea monsters. In the above image, *after flipping and rotating it* to the appropriate
// orientation, there are *two* sea monsters (marked with `*O*`):
//
// `.####...#####..#...###..
// #####..#..#.#.####..#.#.
// .#.#...#.###...#.##.*O*#..
// #.*O*.##.*O**O*#.#.*O**O*.##.*O**O**O*##
// ..#*O*.#*O*#.*O*##*O*..*O*.#*O*##.##
// ...#.#..##.##...#..#..##
// #.##.#..#.#..#..##.#.#..
// .###.##.....#...###.#...
// #.####.#.#....##.#..#.#.
// ##...#..#....#..#...####
// ..#.##...###..#.#####..#
// ....#.##.#.#####....#...
// ..##.##.###.....#.##..#.
// #...#...###..####....##.
// .#.##...#.##.#.#.###...#
// #.###.#..####...##..#...
// #.###...#.##...#.##*O*###.
// .*O*##.#*O**O*.###*O**O*##..*O**O**O*##.
// ..*O*#.*O*..*O*..*O*.#*O*##*O*##.###
// #.#..##.########..#..##.
// #.#####..#.#...##..#....
// #....##..#.#########..##
// #...#.....#..##...###.##
// #..###....##.#...##.##.#
// `
//
// Determine how rough the waters are in the sea monsters' habitat by counting the number of `#`
// that are *not* part of a sea monster. In the above example, the habitat's water roughness is
// *`273`*.
//
// *How many `#` are not part of a sea monster?*

use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
// use std::ops::Index;

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    height: usize,
    data: Vec<Vec<char>>,
}

impl Tile {
    pub fn new(id: usize, data: Vec<Vec<char>>) -> Tile {
        let height = data.len();
        Tile { id, height, data }
    }

    // top, right, bottom, left
    pub fn sides(&self) -> [String; 4] {
        [
            self.data[0].iter().collect(),
            self.data.iter().map(|v| v[v.len() - 1]).collect(),
            self.data[self.data.len() - 1].iter().collect(),
            self.data.iter().map(|v| v[0]).collect(),
        ]
    }

    // o   h   v   vh  L1  L2  R1  R2
    // 123 321 789 987 369 987 741 987
    // 456 654 456 654 258 654 852 654
    // 789 987 123 321 147 321 963 321
    //
    // hL1 vL1 hR1 vR1 hL2 vL2 hR2 vR2
    // 963 147 147 963 789 321 789 321
    // 852 258 258 852 456 654 456 654
    // 741 369 369 741 321 987 123 987
    //
    // o   == R4  == L4  == hh  ==  vv
    // v   == vL2 == vR2
    // h   == hL2 == vR2
    // hv  == vh  == L2  == R2
    // L1  == R3
    // R1  == L3
    // hL1 == vR1
    // vL1 == hR1
    pub fn orientations(&self) -> impl Iterator<Item = Tile> + '_ {
        vec![
            self.data.clone(),
            flip_v(&self.data),
            flip_h(&self.data),
            flip_h(&flip_v(&self.data)),
            rotate(&self.data),
            rotate(&rotate(&rotate(&self.data))),
            flip_v(&rotate(&self.data)),
            flip_h(&rotate(&self.data)),
        ]
        .into_iter()
        .map(move |data| Tile::new(self.id, data))
    }
}

impl From<&str> for Tile {
    fn from(s: &str) -> Self {
        let lines = s.trim().lines().collect::<Vec<_>>();
        let id = lines[0][5..9].parse::<usize>().unwrap();
        let data = s
            .lines()
            .skip(1)
            .map(|line| line.chars().collect())
            .collect();

        Tile::new(id, data)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        for line in &self.data {
            writeln!(f, "{}", line.iter().collect::<String>())?;
        }
        writeln!(f)?;
        let sides = self.sides();
        let t = &sides[0];
        let r = &sides[1];
        let b = &sides[2];
        let l = &sides[3];
        writeln!(f, "{}", t)?;
        for i in 1..self.height - 1 {
            writeln!(
                f,
                "{}{: >width$}",
                &l[i..i + 1],
                &r[i..i + 1],
                width = sides[0].len() - 1
            )?;
        }
        write!(f, "{}", b)?;

        Ok(())
    }
}

#[inline]
fn rev_str(s: impl AsRef<str>) -> String {
    s.as_ref().chars().rev().collect()
}

fn check<S: AsRef<str>>(a: S, b: S) -> Option<String> {
    let (a, b) = (a.as_ref(), b.as_ref());
    if a == b {
        Some(a.to_string())
    } else {
        let reversed = rev_str(&a);
        if reversed == b {
            Some(reversed)
        } else {
            None
        }
    }
}

macro_rules! set {
    ($set:ident[$a:expr][$b:expr] exists) => {
        $set.entry($a).or_insert(HashSet::new()).contains(&$b)
    };
    ($set:ident[$a:expr][$b:expr] insert) => {
        $set.entry($a).or_insert(HashSet::new()).insert($b)
    };
}

fn dfs<'a>(
    start: usize,
    graph: &'a HashMap<usize, Vec<usize>>,
    visited: &'a mut HashMap<usize, HashSet<usize>>,
    path: &'a mut Vec<usize>,
) -> &'a mut Vec<usize> {
    path.push(start);
    for next in graph.get(&start).unwrap().iter().cloned() {
        if !set!(visited[start][next] exists) {
            set!(visited[start][next] insert);
            set!(visited[next][start] insert);
            dfs(next, graph, visited, path);
        }
    }

    path
}

fn rotate(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = input.len();
    let mut output = vec![vec![char::default(); len]; len];
    for y in 0..len {
        for x in 0..len {
            output[x][y] = input[len - 1 - y][x];
        }
    }

    output
}

fn flip_v(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    input.iter().rev().cloned().collect()
}

fn flip_h(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = input.clone();
    for i in 0..output.len() {
        output[i] = output[i].iter().copied().rev().collect();
    }

    output
}

const UP: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (1, 0);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);
const DIRECTIONS: [(isize, isize); 4] = [UP, RIGHT, DOWN, LEFT];

fn main() -> Result<()> {
    let input = include_str!("./input/2020-20.txt").trim();
    let tiles = input.split("\n\n").map(Tile::from).collect::<Vec<_>>();

    let mut adjacency_list = HashMap::new();
    for (i, tile) in tiles.iter().enumerate() {
        for (j, other) in tiles.iter().enumerate() {
            if i == j {
                continue;
            }

            for a in tile.sides().iter() {
                for b in other.sides().iter() {
                    if check(a, b).is_some() {
                        adjacency_list
                            .entry(tile.id)
                            .or_insert(vec![])
                            .push(other.id)
                    }
                }
            }
        }
    }

    let corner_ids = adjacency_list
        .iter()
        .filter_map(|(id, v)| if v.len() == 2 { Some(id) } else { None })
        .copied()
        .collect::<HashSet<usize>>();
    aoc_lib::set_part_1!(corner_ids.iter().product::<usize>());

    // Looks like we have 3 Euler paths and 1 Euler cycle
    for id in &corner_ids {
        let mut path = vec![];
        dfs(*id, &adjacency_list, &mut HashMap::new(), &mut path);
        println!(
            "corner {} is cycle: {}",
            id,
            path[0] == path[path.len() - 1]
        );
    }

    let img_width = (tiles.len() as f64).sqrt() as usize;
    let mut image: Vec<Vec<Option<usize>>> = vec![vec![None; img_width]; img_width];

    // Put in the top-left corner pieces
    let top_left = *corner_ids.iter().next().unwrap();
    let adjacent = adjacency_list.get(&top_left).unwrap();
    let right = &adjacent[0];
    let under = &adjacent[1];
    image[0][0] = Some(top_left);
    image[0][1] = Some(*right);
    image[1][0] = Some(*under);
    let mut used = HashSet::new();
    used.insert(top_left);
    used.insert(*right);
    used.insert(*under);

    let mut tiles = tiles
        .into_iter()
        .map(|t| (t.id, t))
        .collect::<HashMap<usize, Tile>>();

    loop {
        if used.len() == tiles.len() {
            break;
        }
        for x in 0..img_width {
            for y in 0..img_width {
                if image[y][x].is_some() {
                    continue;
                }
                let mut available = adjacency_list
                    .keys()
                    .filter(|k| !used.contains(k))
                    .copied()
                    .collect::<HashSet<usize>>();
                for (dx, dy) in DIRECTIONS.iter() {
                    let (x, y) = ((x as isize) + dx, ((y as isize) + dy));
                    if (x >= 0 && x < (img_width as isize)) && (y >= 0 && y < (img_width as isize))
                    {
                        let (x, y) = (x as usize, y as usize);
                        if let Some(id) = image[y][x] {
                            available = available
                                .intersection(
                                    &adjacency_list
                                        .get(&id)
                                        .unwrap()
                                        .iter()
                                        .copied()
                                        .collect::<HashSet<usize>>(),
                                )
                                .copied()
                                .collect::<HashSet<_>>();
                        }
                    }
                }

                if available.len() == 1 {
                    let id = *available.iter().next().unwrap();
                    image[y][x] = Some(id);
                    used.insert(id);
                }
            }
        }
    }

    // rotate each piece correctly
    for y in 0..img_width {
        for x in 0..img_width {
            let id = image[y][x].unwrap();
            let tile = tiles.get(&id).unwrap();
            let mut orientations = tile.orientations().collect::<Vec<_>>();
            // NOTE: this assumes that there's only one valid orientation for a piece that fits with its neighbours
            for (i, (dx, dy)) in DIRECTIONS.iter().enumerate() {
                let (x, y) = ((x as isize) + dx, (y as isize) + dy);
                let (x, y) = (x as usize, y as usize);
                if let Some(Some(Some(other_id))) = image.get(y).map(|row| row.get(x)) {
                    let other = tiles.get(other_id).unwrap();
                    let mut ok_orientations = vec![];
                    for a in &orientations {
                        for b in other.orientations() {
                            if a.sides()[i] == b.sides()[(i + 2) % 4] {
                                ok_orientations.push(a as *const _);
                            }
                        }
                    }
                    orientations.retain(|t| ok_orientations.contains(&(t as *const _)));
                }
            }

            if orientations.len() == 1 {
                let rotated_tile = orientations.remove(0);
                let tile = tiles.get_mut(&id).unwrap();
                *tile = rotated_tile;
            }
        }
    }

    // remove borders and build image
    let picture = {
        let mut data: Vec<Vec<char>> = vec![];
        for y in 0..img_width {
            for i in 1..9 {
                let mut line = vec![];
                for x in 0..img_width {
                    let tile = tiles.get(&image[y][x].unwrap()).unwrap();
                    let row = &tile.data[i];
                    line.extend(&row[1..9].to_vec());
                }
                data.push(line);
            }
        }

        Tile::new(0, data)
    };

    let sea_monster_positions = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]
    .iter()
    .enumerate()
    .flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter_map(|(x, ch)| if ch == '#' { Some((x, y)) } else { None })
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    // check for sea monsters
    let line_width = img_width * 8;
    let max_y = sea_monster_positions.iter().map(|(_, y)| y).max().unwrap() + 1;
    let max_x = sea_monster_positions.iter().map(|(x, _)| x).max().unwrap() + 1;
    for tile in picture.orientations() {
        let mut sea_monsters = 0;
        for window in tile.data.windows(max_y) {
            for i in 0..line_width - max_x - 1 {
                if sea_monster_positions
                    .iter()
                    .all(|(x, y)| window[*y][*x + i] == '#')
                {
                    sea_monsters += 1;
                }
            }
        }

        // there should only be one orientation which has sea monsters in it
        // NOTE: this assumes the sea monsters do not overlap
        if sea_monsters > 0 {
            let total_hashes = tile
                .data
                .iter()
                .fold(0, |r, v| r + v.iter().filter(|ch| **ch == '#').count());
            let water_roughness = total_hashes - (sea_monsters * sea_monster_positions.len());
            aoc_lib::set_part_2!(water_roughness);
            return Ok(());
        }
    }

    anyhow::bail!("Could not solve part 2")
}
