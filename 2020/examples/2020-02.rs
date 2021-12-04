// See: https://adventofcode.com/2020/day/2
// ## --- Day 2: Password Philosophy ---
//
// Your flight departs in a few days from the coastal airport; the easiest way down to the coast from
// here is via [toboggan][1].
//
// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with
// our computers; we can't log in!" You ask if you can take a look.
//
// Their password database seems to be a little corrupted: some of the passwords wouldn't have been
// allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
//
// To try to debug the problem, they have created a list (your puzzle input) of *passwords* (according
// to the corrupted database) and *the corporate policy when that password was set*.
//
// For example, suppose you have the following list:
//
// `1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
// `
//
// Each line gives the password policy and then the password. The password policy indicates the lowest
// and highest number of times a given letter must appear for the password to be valid. For example,
// `1-3 a` means that the password must contain `a` at least `1` time and at most `3` times.
//
// In the above example, `*2*` passwords are valid. The middle password, `cdefg`, is not; it contains
// no instances of `b`, but needs at least `1`. The first and third passwords are valid: they contain
// one `a` or nine `c`, both within the limits of their respective policies.
//
// *How many passwords are valid* according to their policies?
//
// [1] https://en.wikipedia.org/wiki/Toboggan
//
//
// ## --- Part Two ---
//
// While it appears you validated the passwords correctly, they don't seem to be what the Official
// Toboggan Corporate Authentication System is expecting.
//
// The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from
// his old job at the sled rental place down the street! The Official Toboggan Corporate Policy
// actually works a little differently.
//
// Each policy actually describes two *positions in the password*, where `1` means the first character,
// `2` means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept
// of "index zero"!) *Exactly one of these positions* must contain the given letter. Other occurrences
// of the letter are irrelevant for the purposes of policy enforcement.
//
// Given the same example list from above:
//
// * `1-3 a: *a*b*c*de` is *valid*: position `1` contains `a` and position `3` does not.
// * `1-3 b: *c*d*e*fg` is *invalid*: neither position `1` nor position `3` contains `b`.
// * `2-9 c: c*c*cccccc*c*` is *invalid*: both position `2` and position `9` contain `c`.
//
// *How many passwords are valid* according to the new interpretation of the policies?
use std::ops::RangeInclusive;

struct PasswordWithPolicy {
    range: RangeInclusive<usize>,
    character: char,
    password: String,
}

impl PasswordWithPolicy {
    pub fn is_valid_part_1(&self) -> bool {
        self.range.contains(
            &self
                .password
                .chars()
                .filter(|ch| *ch == self.character)
                .count(),
        )
    }

    pub fn is_valid_part_2(&self) -> bool {
        // convert to zero indexed
        let idx_1 = self.range.start() - 1;
        let idx_2 = self.range.end() - 1;
        let chars = self.password.chars().collect::<Vec<char>>();

        if idx_1 >= chars.len() {
            return false;
        }

        let match_1 = chars[idx_1] == self.character;
        if idx_2 >= chars.len() {
            return match_1;
        }

        let match_2 = chars[idx_2] == self.character;
        match_1 != match_2
    }
}

impl From<&str> for PasswordWithPolicy {
    fn from(s: &str) -> Self {
        let policy_and_password = s.split(':').collect::<Vec<&str>>();
        let policy_parts = policy_and_password[0].split(' ').collect::<Vec<&str>>();
        let policy_range = policy_parts[0].split('-').collect::<Vec<&str>>();
        let start = policy_range[0].parse::<usize>().unwrap();
        let end = policy_range[1].parse::<usize>().unwrap();
        PasswordWithPolicy {
            password: policy_and_password[1].trim().to_string(),
            character: policy_parts[1].chars().last().unwrap(),
            range: start..=end,
        }
    }
}

fn main() {
    let mut part_1_valid_count = 0;
    let mut part_2_valid_count = 0;
    for line in include_str!("./input/2020-02.txt").lines() {
        let pwp = PasswordWithPolicy::from(line);
        if pwp.is_valid_part_1() {
            part_1_valid_count += 1;
        }
        if pwp.is_valid_part_2() {
            part_2_valid_count += 1;
        }
    }

    aoc_lib::set_part_1!(part_1_valid_count);
    aoc_lib::set_part_2!(part_2_valid_count);
}
