// See: https://adventofcode.com/2020/day/24
// ## --- Day 24: Lobby Layout ---
//
// Your raft makes it to the tropical island; it turns out that the small crab was an excellent
// navigator. You make your way to the resort.
//
// As you enter the lobby, you discover a small problem: the floor is being renovated. You can't
// even reach the check-in desk until they've finished installing the *new tile floor*.
//
// The tiles are all *hexagonal*; they need to be arranged in a [hex grid][1] with a very specific
// color pattern. Not in the mood to wait, you offer to help figure out the pattern.
//
// The tiles are all *white* on one side and *black* on the other. They start with the white side
// facing up. The lobby is large enough to fit whatever pattern might need to appear there.
//
// A member of the renovation crew gives you a *list of the tiles that need to be flipped over*
// (your puzzle input). Each line in the list identifies a single tile that needs to be flipped by
// giving a series of steps starting from a *reference tile* in the very center of the room. (Every
// line starts from the same reference tile.)
//
// Because the tiles are hexagonal, every tile has *six neighbors*: east, southeast, southwest,
// west, northwest, and northeast. These directions are given in your list, respectively, as `e`,
// `se`, `sw`, `w`, `nw`, and `ne`. A tile is identified by a series of these directions with *no
// delimiters*; for example, `esenee` identifies the tile you land on if you start at the reference
// tile and then move one tile east, one tile southeast, one tile northeast, and one tile east.
//
// Each time a tile is identified, it flips from white to black or from black to white. Tiles might
// be flipped more than once. For example, a line like `esew` flips a tile immediately adjacent to
// the reference tile, and a line like `nwwswee` flips the reference tile itself.
//
// Here is a larger example:
//
// `sesenwnenenewseeswwswswwnenewsewsw
// neeenesenwnwwswnenewnwwsewnenwseswesw
// seswneswswsenwwnwse
// nwnwneseeswswnenewneswwnewseswneseene
// swweswneswnenwsewnwneneseenw
// eesenwseswswnenwswnwnwsewwnwsene
// sewnenenenesenwsewnenwwwse
// wenwwweseeeweswwwnwwe
// wsweesenenewnwwnwsenewsenwwsesesenwne
// neeswseenwwswnwswswnw
// nenwswwsewswnenenewsenwsenwnesesenew
// enewnwewneswsewnwswenweswnenwsenwsw
// sweneswneswneneenwnewenewwneswswnese
// swwesenesewenwneswnwwneseswwne
// enesenwswwswneneswsenwnewswseenwsese
// wnwnesenesenenwwnenwsewesewsesesew
// nenewswnwewswnenesenwnesewesw
// eneswnwswnwsenenwnwnwwseeswneewsenese
// neswnwewnwnwseenwseesewsenwsweewe
// wseweeenwnesenwwwswnew
// `
//
// In the above example, 10 tiles are flipped once (to black), and 5 more are flipped twice (to
// black, then back to white). After all of these instructions have been followed, a total of *`10`*
// tiles are *black*.
//
// Go through the renovation crew's list and determine which tiles they need to flip. After all of
// the instructions have been followed, *how many tiles are left with the black side up?*
//
// [1] https://en.wikipedia.org/wiki/Hexagonal_tiling
//
//
// ## --- Part Two ---
//
// The tile floor in the lobby is meant to be a living art exhibit. Every day, the tiles are all
// flipped according to the following rules:
//
// * Any *black* tile with *zero* or *more than 2* black tiles immediately adjacent to it is flipped
// to *white*.
// * Any *white* tile with *exactly 2* black tiles immediately adjacent to it is flipped to *black*.
//
// Here, *tiles immediately adjacent* means the six tiles directly touching the tile in question.
//
// The rules are applied *simultaneously* to every tile; put another way, it is first determined
// which tiles need to be flipped, then they are all flipped at the same time.
//
// In the above example, the number of black tiles that are facing up after the given number of days
// has passed is as follows:
//
// `Day 1: 15
// Day 2: 12
// Day 3: 25
// Day 4: 14
// Day 5: 23
// Day 6: 28
// Day 7: 41
// Day 8: 37
// Day 9: 49
// Day 10: 37
// Day 20: 132
// Day 30: 259
// Day 40: 406
// Day 50: 566
// Day 60: 788
// Day 70: 1106
// Day 80: 1373
// Day 90: 1844
// Day 100: 2208
// `
//
// After executing this process a total of 100 times, there would be *`2208`* black tiles facing up.
//
// *How many tiles will be black after 100 days?*

use anyhow::Result;
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::ops::{Add, AddAssign};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0, z: 0 }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:>2}, {:>2}, {:>2})", self.x, self.y, self.z)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl From<(isize, isize, isize)> for Point {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Point { x, y, z }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum HexDir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthEast,
    NorthWest,
}

impl HexDir {
    pub const ALL: [HexDir; 6] = [
        HexDir::East,
        HexDir::West,
        HexDir::SouthEast,
        HexDir::SouthWest,
        HexDir::NorthEast,
        HexDir::NorthWest,
    ];

    // https://www.redblobgames.com/grids/hexagons/#coordinates-cube
    pub fn as_point(self) -> Point {
        match self {
            HexDir::East => (1, -1, 0).into(),
            HexDir::West => (-1, 1, 0).into(),
            HexDir::SouthEast => (0, -1, 1).into(),
            HexDir::SouthWest => (-1, 0, 1).into(),
            HexDir::NorthEast => (1, 0, -1).into(),
            HexDir::NorthWest => (0, 1, -1).into(),
        }
    }

    pub fn vec_from_str(s: &str) -> Vec<HexDir> {
        let mut tiles = vec![];
        let mut pos = 0;
        while pos < s.len() {
            let tile = HexDir::from(&s[pos..]);
            pos += match tile {
                HexDir::East => 1,
                HexDir::West => 1,
                HexDir::SouthEast => 2,
                HexDir::SouthWest => 2,
                HexDir::NorthWest => 2,
                HexDir::NorthEast => 2,
            };
            tiles.push(tile);
        }

        tiles
    }
}

impl From<&str> for HexDir {
    fn from(s: &str) -> Self {
        match s {
            s if s.starts_with("e") => HexDir::East,
            s if s.starts_with("w") => HexDir::West,
            s if s.starts_with("se") => HexDir::SouthEast,
            s if s.starts_with("sw") => HexDir::SouthWest,
            s if s.starts_with("ne") => HexDir::NorthEast,
            s if s.starts_with("nw") => HexDir::NorthWest,
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<()> {
    let input = include_str!("./input/2020-24.txt").trim();
    let tile_paths = input.lines().map(HexDir::vec_from_str).collect::<Vec<_>>();

    let mut black_tiles = HashSet::new();
    for path in &tile_paths {
        let pos = path
            .iter()
            .fold(Point::new(), |pnt, dir| pnt + dir.as_point());

        if black_tiles.contains(&pos) {
            black_tiles.remove(&pos);
        } else {
            black_tiles.insert(pos);
        }
    }

    aoc_lib::set_part_1!(black_tiles.len());

    for _ in 0..100 {
        let mut tmp = HashSet::new();
        let count = |p| {
            HexDir::ALL
                .iter()
                .filter(|t| black_tiles.contains(&(t.as_point() + p)))
                .count()
        };

        let mut min_point = Point::new();
        let mut max_point = Point::new();
        for point in &black_tiles {
            min_point.x = std::cmp::min(min_point.x, point.x);
            min_point.y = std::cmp::min(min_point.y, point.y);
            min_point.z = std::cmp::min(min_point.z, point.z);
            max_point.x = std::cmp::max(max_point.x, point.x);
            max_point.y = std::cmp::max(max_point.y, point.y);
            max_point.z = std::cmp::max(max_point.z, point.z);

            let black_neighbours = count(*point);
            if black_neighbours == 1 || black_neighbours == 2 {
                tmp.insert(*point);
            }
        }

        // NOTE: not the best, but +/- 5 points seems to cover the required area
        for x in (min_point.x - 5)..=(max_point.x + 5) {
            for y in (min_point.y - 5)..=(max_point.y + 5) {
                for z in (min_point.z - 5)..=(max_point.z + 5) {
                    // not a valid hex position
                    if x + y + z != 0 {
                        continue;
                    }

                    let point = Point { x, y, z };

                    // not a white tile
                    if black_tiles.contains(&point) {
                        continue;
                    }

                    // println!("{}", point);
                    if count(point) == 2 {
                        tmp.insert(point);
                    }
                }
            }
        }

        black_tiles = tmp;
    }

    aoc_lib::set_part_2!(black_tiles.len());

    Ok(())
}
