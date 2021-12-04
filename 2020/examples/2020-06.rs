// See: https://adventofcode.com/2020/day/6
// ## --- Day 6: Custom Customs ---
//
// As your flight approaches the regional airport where you'll switch to a much larger plane, [customs
// declaration forms][1] are distributed to the passengers.
//
// The form asks a series of 26 yes-or-no questions marked `a` through `z`. All you need to do is
// identify the questions for which *anyone in your group* answers "yes". Since your group is just you,
// this doesn't take very long.
//
// However, the person sitting next to you seems to be experiencing a language barrier and asks if you
// can help. For each of the people in their group, you write down the questions for which they answer
// "yes", one per line. For example:
//
// `abcx
// abcy
// abcz
// `
//
// In this group, there are *`6`* questions to which anyone answered "yes": `a`, `b`, `c`, `x`, `y`,
// and `z`. (Duplicate answers to the same question don't count extra; each question counts at most
// once.)
//
// Another group asks for your help, then another, and eventually you've collected answers from every
// group on the plane (your puzzle input). Each group's answers are separated by a blank line, and
// within each group, each person's answers are on a single line. For example:
//
// `abc
// a
// b
// c
// ab
// ac
// a
// a
// a
// a
// b
// `
//
// This list represents answers from five groups:
//
// * The first group contains one person who answered "yes" to *`3`* questions: `a`, `b`, and `c`.
// * The second group contains three people; combined, they answered "yes" to *`3`* questions: `a`,
// `b`, and `c`.
// * The third group contains two people; combined, they answered "yes" to *`3`* questions: `a`, `b`,
// and `c`.
// * The fourth group contains four people; combined, they answered "yes" to only *`1`* question, `a`.
// * The last group contains one person who answered "yes" to only *`1`* question, `b`.
//
// In this example, the sum of these counts is `3 + 3 + 3 + 1 + 1` = *`11`*.
//
// For each group, count the number of questions to which anyone answered "yes". *What is the sum of
// those counts?*
//
// [1] https://en.wikipedia.org/wiki/Customs_declaration
//
//
// ## --- Part Two ---
//
// As you finish the last group's customs declaration, you notice that you misread one word in the
// instructions:
//
// You don't need to identify the questions to which *anyone* answered "yes"; you need to identify the
// questions to which *everyone* answered "yes"!
//
// Using the same example as above:
//
// `abc
// a
// b
// c
// ab
// ac
// a
// a
// a
// a
// b
// `
//
// This list represents answers from five groups:
//
// * In the first group, everyone (all 1 person) answered "yes" to *`3`* questions: `a`, `b`, and `c`.
// * In the second group, there is *no* question to which everyone answered "yes".
// * In the third group, everyone answered yes to only *`1`* question, `a`. Since some people did not
// answer "yes" to `b` or `c`, they don't count.
// * In the fourth group, everyone answered yes to only *`1`* question, `a`.
// * In the fifth group, everyone (all 1 person) answered "yes" to *`1`* question, `b`.
//
// In this example, the sum of these counts is `3 + 0 + 1 + 1 + 1` = *`6`*.
//
// For each group, count the number of questions to which *everyone* answered "yes". *What is the sum
// of those counts?*

fn main() {
    let input = include_str!("./input/2020-06.txt");

    let mut groups = vec![];
    let mut group = String::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            groups.push(group.trim().to_string());
            group = String::new();
        }

        group.push_str(&format!("{}\n", line));
    }
    groups.push(group.trim().to_string());

    use std::collections::HashSet;

    let mut total_1 = 0;
    let mut total_2 = 0;
    for group in groups {
        let set = group
            .chars()
            .filter(|ch| ch.is_ascii_alphabetic())
            .collect::<HashSet<char>>();
        total_1 += set.iter().count();

        let person_sets = group
            .lines()
            .map(|line| {
                line.chars()
                    .filter(|ch| ch.is_ascii_alphabetic())
                    .collect::<HashSet<char>>()
            })
            .collect::<Vec<HashSet<char>>>();

        for ch in 'a'..='z' {
            if person_sets.iter().all(|set| set.contains(&ch)) {
                total_2 += 1;
            }
        }
    }

    aoc_lib::set_part_1!(total_1);
    aoc_lib::set_part_2!(total_2);
}
