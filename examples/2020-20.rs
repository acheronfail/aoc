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
use easy_collections::{map, set, EasyMap, EasySet};
use std::fmt::{self, Display};

#[derive(Debug, Clone, Default)]
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

    // NOTE: all orientations. Flipping horizontally and rotating twice is the same as flipping vertically.
    // Therefore, all possible rotations and all possible rotations after flipping once covers every orientation.
    pub fn orientations(&self) -> impl Iterator<Item = Tile> + '_ {
        vec![
            self.data.clone(),
            rotate(&self.data),
            rotate(&rotate(&self.data)),
            rotate(&rotate(&rotate(&self.data))),
            flip_v(&self.data),
            rotate(&flip_v(&self.data)),
            rotate(&rotate(&flip_v(&self.data))),
            rotate(&rotate(&rotate(&flip_v(&self.data)))),
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

fn sides_can_match<S: AsRef<str>>(a: S, b: S) -> bool {
    let (a, b) = (a.as_ref(), b.as_ref());
    a == b || a.chars().rev().collect::<String>() == b
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

// up, right, down, left
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const SEA_MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

fn main() -> Result<()> {
    let input = include_str!("./input/2020-20.txt").trim();
    let tiles = input.split("\n\n").map(Tile::from).collect::<Vec<_>>();

    let mut adjacency_list = map! {};
    for (i, tile) in tiles.iter().enumerate() {
        for (j, other) in tiles.iter().enumerate() {
            if i == j {
                continue;
            }

            for a in tile.sides().iter() {
                for b in other.sides().iter() {
                    if sides_can_match(a, b) {
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
        .collect::<EasySet<usize>>();
    aoc_lib::set_part_1!(corner_ids.iter().product::<usize>());

    let n_tiles = (tiles.len() as f64).sqrt() as usize;
    let mut pieces: Vec<Vec<Option<usize>>> = vec![vec![None; n_tiles]; n_tiles];

    // put in the top-left corner pieces
    let top_left = *corner_ids.iter().next().unwrap();
    let adjacent = adjacency_list.get(&top_left).unwrap();
    pieces[0][0] = Some(top_left);
    pieces[0][1] = Some(adjacent[0]);
    pieces[1][0] = Some(adjacent[1]);
    let mut used = set! {top_left, adjacent[0], adjacent[1]};
    let mut tiles: EasyMap<usize, Tile> = tiles.into_iter().map(|t| (t.id, t)).collect();

    // place all the tiles in the right spots
    loop {
        if used.len() == tiles.len() {
            break;
        }

        for x in 0..n_tiles {
            for y in 0..n_tiles {
                if pieces[y][x].is_some() {
                    continue;
                }

                let mut available = adjacency_list
                    .keys()
                    .filter(|k| !used.contains(k))
                    .copied()
                    .collect::<EasySet<usize>>();

                for (dx, dy) in DIRECTIONS.iter() {
                    let (x, y) = ((x as isize) + dx, (y as isize) + dy);
                    let (x, y) = (x as usize, y as usize);
                    if let Some(Some(Some(id))) = pieces.get(y).map(|row| row.get(x)) {
                        available &= adjacency_list.get(&id).unwrap();
                    }
                }

                if available.len() == 1 {
                    let id = *available.iter().next().unwrap();
                    pieces[y][x] = Some(id);
                    used.insert(id);
                }
            }
        }
    }

    // rotate each piece correctly
    for y in 0..n_tiles {
        for x in 0..n_tiles {
            let id = pieces[y][x].unwrap();
            let tile = tiles.get(&id).unwrap();
            let mut orientations = tile.orientations().collect::<Vec<_>>();

            // NOTE: this assumes that there's only one valid orientation for a piece that fits with its neighbours
            for (i, (dx, dy)) in DIRECTIONS.iter().enumerate() {
                let (x, y) = ((x as isize) + dx, (y as isize) + dy);
                let (x, y) = (x as usize, y as usize);
                if let Some(Some(Some(other_id))) = pieces.get(y).map(|row| row.get(x)) {
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

    // remove borders and build final picture
    let picture = {
        let mut data: Vec<Vec<char>> = vec![];
        for y in 0..n_tiles {
            for i in 1..9 {
                let mut line = vec![];
                for x in 0..n_tiles {
                    let tile = tiles.get(&pieces[y][x].unwrap()).unwrap();
                    let row = &tile.data[i];
                    line.extend(&row[1..9].to_vec());
                }
                data.push(line);
            }
        }

        Tile::new(0, data)
    };

    // map sea monster to a list of positions
    let sea_monster_positions = SEA_MONSTER
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
    let line_width = n_tiles * 8;
    let max_y = sea_monster_positions.iter().map(|(_, y)| y).max().unwrap() + 1;
    let max_x = sea_monster_positions.iter().map(|(x, _)| x).max().unwrap() + 1;
    for tile in picture.orientations() {
        let mut sea_monsters = 0;
        for window in tile.data.windows(max_y) {
            for i in 0..line_width - max_x - 1 {
                let has_sea_monster = sea_monster_positions
                    .iter()
                    .all(|(x, y)| window[*y][*x + i] == '#');
                if has_sea_monster {
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
