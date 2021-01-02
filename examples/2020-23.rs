// See: https://adventofcode.com/2020/day/23
// ## --- Day 23: Crab Cups ---
//
// The small crab challenges *you* to a game! The crab is going to mix up some cups, and you have to
// predict where they'll end up.
//
// The cups will be arranged in a circle and labeled *clockwise* (your puzzle input). For example,
// if your labeling were `32415`, there would be five cups in the circle; going clockwise around the
// circle from the first cup, the cups would be labeled `3`, `2`, `4`, `1`, `5`, and then back to
// `3` again.
//
// Before the crab starts, it will designate the first cup in your list as the *current cup*. The
// crab is then going to do *100 moves*.
//
// Each *move*, the crab does the following actions:
//
// * The crab picks up the *three cups* that are immediately *clockwise* of the *current cup*. They
// are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
// * The crab selects a *destination cup*: the cup with a *label* equal to the *current cup's* label
// minus one. If this would select one of the cups that was just picked up, the crab will keep
// subtracting one until it finds a cup that wasn't just picked up. If at any point in this
// process the value goes below the lowest value on any cup's label, it *wraps around* to the
// highest value on any cup's label instead.
// * The crab places the cups it just picked up so that they are *immediately clockwise* of the
// destination cup. They keep the same order as when they were picked up.
// * The crab selects a new *current cup*: the cup which is immediately clockwise of the current
// cup.
//
// For example, suppose your cup labeling were `389125467`. If the crab were to do merely 10 moves,
// the following changes would occur:
//
// `-- move 1 --
// cups: (3) 8  9  1  2  5  4  6  7
// pick up: 8, 9, 1
// destination: 2
// -- move 2 --
// cups:  3 (2) 8  9  1  5  4  6  7
// pick up: 8, 9, 1
// destination: 7
// -- move 3 --
// cups:  3  2 (5) 4  6  7  8  9  1
// pick up: 4, 6, 7
// destination: 3
// -- move 4 --
// cups:  7  2  5 (8) 9  1  3  4  6
// pick up: 9, 1, 3
// destination: 7
// -- move 5 --
// cups:  3  2  5  8 (4) 6  7  9  1
// pick up: 6, 7, 9
// destination: 3
// -- move 6 --
// cups:  9  2  5  8  4 (1) 3  6  7
// pick up: 3, 6, 7
// destination: 9
// -- move 7 --
// cups:  7  2  5  8  4  1 (9) 3  6
// pick up: 3, 6, 7
// destination: 8
// -- move 8 --
// cups:  8  3  6  7  4  1  9 (2) 5
// pick up: 5, 8, 3
// destination: 1
// -- move 9 --
// cups:  7  4  1  5  8  3  9  2 (6)
// pick up: 7, 4, 1
// destination: 5
// -- move 10 --
// cups: (5) 7  4  1  8  3  9  2  6
// pick up: 7, 4, 1
// destination: 3
// -- final --
// cups:  5 (8) 3  7  4  1  9  2  6
// `
//
// In the above example, the cups' values are the labels as they appear moving clockwise around the
// circle; the *current cup* is marked with `( )`.
//
// After the crab is done, what order will the cups be in? Starting *after the cup labeled `1`*,
// collect the other cups' labels clockwise into a single string with no extra characters; each
// number except `1` should appear exactly once. In the above example, after 10 moves, the cups
// clockwise from `1` are labeled `9`, `2`, `6`, `5`, and so on, producing *`92658374`*. If the crab
// were to complete all 100 moves, the order after cup `1` would be *`67384529`*.
//
// Using your labeling, simulate 100 moves. *What are the labels on the cups after cup `1`?*
//
//
// ## --- Part Two ---
//
// Due to what you can only assume is a mistranslation (you're not exactly fluent in Crab), you are
// quite surprised when the crab starts arranging *many* cups in a circle on your raft - *one
// million* (`1000000`) in total.
//
// Your labeling is still correct for the first few cups; after that, the remaining cups are just
// numbered in an increasing fashion starting from the number after the highest number in your list
// and proceeding one by one until one million is reached. (For example, if your labeling were
// `54321`, the cups would be numbered `5`, `4`, `3`, `2`, `1`, and then start counting up from `6`
// until one million is reached.) In this way, every number from one through one million is used
// exactly once.
//
// After discovering where you made the mistake in translating Crab Numbers, you realize the small
// crab isn't going to do merely 100 moves; the crab is going to do *ten million* (`10000000`)
// moves!
//
// The crab is going to hide your *stars* - one each - under the *two cups that will end up
// immediately clockwise of cup `1`*. You can have them if you predict what the labels on those cups
// will be when the crab is finished.
//
// In the above example (`389125467`), this would be `934001` and then `159792`; multiplying these
// together produces *`149245887792`*.
//
// Determine which two cups will end up immediately clockwise of cup `1`. *What do you get if you
// multiply their labels together?*

use anyhow::Result;

fn play(cups: &Vec<usize>, count: usize) -> Vec<usize> {
    // build up a map (using a vec) of cup_number->cup_to_the_right
    let mut right_of = vec![0; cups.len() + 1];
    for (i, n) in cups.iter().enumerate() {
        right_of[*n] = cups[(i + 1) % cups.len()];
    }

    let max = *cups.iter().max().unwrap();
    let mut cur = cups[0];
    for _ in 0..count {
        // the values of the "removed" cups
        let r1 = right_of[cur];
        let r2 = right_of[r1];
        let r3 = right_of[r2];
        // "remove" the three cups by setting the next cup after the current to the cup after them
        right_of[cur] = right_of[r3];

        // find destination cup
        let mut nxt = if cur > 1 { cur - 1 } else { max };
        while [r1, r2, r3].contains(&nxt) {
            nxt = if nxt > 1 { nxt - 1 } else { max };
        }

        // splice in the "removed" cups
        let tmp = right_of[nxt];
        right_of[nxt] = r1;
        right_of[r3] = tmp;

        // current is now the cup right of `cur`
        cur = right_of[cur];
    }

    // rebuild the circle of cups
    let mut cups = vec![];
    let mut i = 1;
    for _ in 0..right_of.len() - 1 {
        let nxt = right_of[i];
        cups.push(nxt);
        i = nxt;
    }
    cups
}

fn main() -> Result<()> {
    let input = include_str!("./input/2020-23.txt").trim();
    let mut label = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    aoc_lib::set_part_1!(play(&label, 100)
        .iter()
        .cycle()
        .skip_while(|n| **n != 1)
        .skip(1)
        .take(label.len() - 1)
        .map(|n| n.to_string())
        .collect::<String>());

    // part 2
    label.extend(10..=1_000_000);
    let result = play(&label, 10_000_000);
    let start = result.iter().position(|n| *n == 1).unwrap();
    let a = result[(start + 1) % 1_000_000];
    let b = result[(start + 2) % 1_000_000];
    aoc_lib::set_part_2!(a * b);

    Ok(())
}
