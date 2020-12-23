// See: https://adventofcode.com/2020/day/16
// ## --- Day 16: Ticket Translation ---
//
// As you're walking to yet another connecting flight, you realize that one of the legs of your
// re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in
// a language you don't understand. You should probably figure out what it says before you get to
// the train station after the next flight.
//
// Unfortunately, you can't actually *read* the words on the ticket. You can, however, read the
// numbers, and so you figure out *the fields these tickets must have* and *the valid ranges* for
// values in those fields.
//
// You collect the *rules for ticket fields*, the *numbers on your ticket*, and the *numbers on
// other nearby tickets* for the same train service (via the airport security cameras) together into
// a single document you can reference (your puzzle input).
//
// The *rules for ticket fields* specify a list of fields that exist *somewhere* on the ticket and
// the *valid ranges of values* for each field. For example, a rule like `class: 1-3 or 5-7` means
// that one of the fields in every ticket is named `class` and can be any value in the ranges `1-3`
// or `5-7` (inclusive, such that `3` and `5` are both valid in this field, but `4` is not).
//
// Each ticket is represented by a single line of comma-separated values. The values are the numbers
// on the ticket in the order they appear; every ticket has the same format. For example, consider
// this ticket:
//
// `.--------------------------------------------------------.
// | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
// |                                                        |
// | ??: 301  ??: 302             ???????: 303      ??????? |
// | ??: 401  ??: 402           ???? ????: 403    ????????? |
// '--------------------------------------------------------'
// `
//
// Here, `?` represents text in a language you don't understand. This ticket might be represented as
// `101,102,103,104,301,302,303,401,402,403`; of course, the actual train tickets you're looking at
// are *much* more complicated. In any case, you've extracted just the numbers in such a way that
// the first number is always the same specific field, the second number is always a different
// specific field, and so on - you just don't know what each position actually means!
//
// Start by determining which tickets are *completely invalid*; these are tickets that contain
// values which *aren't valid for any field*. Ignore *your ticket* for now.
//
// For example, suppose you have the following notes:
//
// `class: 1-3 or 5-7
// row: 6-11 or 33-44
// seat: 13-40 or 45-50
// your ticket:
// 7,1,14
// nearby tickets:
// 7,3,47
// 40,*4*,50
// *55*,2,20
// 38,6,*12*
// `
//
// It doesn't matter which position corresponds to which field; you can identify invalid *nearby
// tickets* by considering only whether tickets contain *values that are not valid for any field*.
// In this example, the values on the first *nearby ticket* are all valid for at least one field.
// This is not true of the other three *nearby tickets*: the values `4`, `55`, and `12` are are not
// valid for any field. Adding together all of the invalid values produces your *ticket scanning
// error rate*: `4 + 55 + 12` = *`71`*.
//
// Consider the validity of the *nearby tickets* you scanned. *What is your ticket scanning error
// rate?*
//
//
// ## --- Part Two ---
//
// Now that you've identified which tickets contain invalid values, *discard those tickets
// entirely*. Use the remaining valid tickets to determine which field is which.
//
// Using the valid ranges for each field, determine what order the fields appear on the tickets. The
// order is consistent between all tickets: if `seat` is the third field, it is the third field on
// every ticket, including *your ticket*.
//
// For example, suppose you have the following notes:
//
// `class: 0-1 or 4-19
// row: 0-5 or 8-19
// seat: 0-13 or 16-19
// your ticket:
// 11,12,13
// nearby tickets:
// 3,9,18
// 15,1,5
// 5,14,9
// `
//
// Based on the *nearby tickets* in the above example, the first position must be `row`, the second
// position must be `class`, and the third position must be `seat`; you can conclude that in *your
// ticket*, `class` is `12`, `row` is `11`, and `seat` is `13`.
//
// Once you work out which field is which, look for the six fields on *your ticket* that start with
// the word `departure`. *What do you get if you multiply those six values together?*

use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = include_str!("./input/2020-16.txt").trim();
    let groups = input.split("\n\n").collect::<Vec<_>>();

    let re = Regex::new(r#"((?:\d+)-(?:\d+))"#)?;
    let mut rules = vec![];
    for line in groups[0].split('\n') {
        let mut rule = vec![];
        for capture in re.captures_iter(line) {
            let mat = capture.get(1).unwrap();
            let s = mat.as_str();
            let (start, end) = aoc_lib::utils::string_split2("-", s);
            rule.push(start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap());
        }
        rules.push((line, rule));
    }

    let my_ticket = groups[1]
        .trim()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let tickets = groups[2]
        .trim()
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let invalid_ticket_numbers = |ticket: &[usize]| {
        ticket
            .iter()
            .copied()
            .filter(|num| {
                rules
                    .iter()
                    .all(|(_, ranges)| ranges.iter().all(|range| !range.contains(num)))
            })
            .collect::<Vec<_>>()
    };

    let mut invalid_values = vec![];
    for ticket in &tickets {
        invalid_values.append(&mut invalid_ticket_numbers(ticket));
    }

    aoc_lib::set_part_1!(invalid_values.iter().sum::<usize>());

    let tickets = tickets
        .iter()
        .filter(|ticket| invalid_ticket_numbers(ticket).is_empty())
        .collect::<Vec<_>>();

    // figure out where the rules are
    let mut possible_positions = HashMap::new();
    let ticket_number_len = tickets[0].len();
    for i in 0..rules.len() {
        let (_, ranges) = &rules[i];
        for j in 0..ticket_number_len {
            let is_possible_rule = tickets
                .iter()
                .copied()
                .all(|numbers| ranges.iter().any(|range| range.contains(&numbers[j])));

            if is_possible_rule {
                possible_positions.entry(i).or_insert(vec![]).push(j);
            }
        }
    }

    // solve which columns map to which rules
    let positions = loop {
        let singles = possible_positions
            .iter()
            .filter_map(|(rule_idx, ticket_idxs)| {
                if ticket_idxs.len() == 1 {
                    Some((*rule_idx, ticket_idxs[0]))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if singles.len() == rules.len() {
            break possible_positions
                .iter()
                .map(|(k, v)| (*k, v[0]))
                .collect::<HashMap<usize, usize>>();
        }

        for (rule_idx, ticket_idx) in &singles {
            for (r_idx, t_idxs) in possible_positions.iter_mut() {
                if r_idx != rule_idx {
                    t_idxs.retain(|idx| idx != ticket_idx);
                }
            }
        }
    };

    let rules_that_start_with_departure = rules
        .iter()
        .enumerate()
        .filter_map(|(idx, (name, _))| {
            if name.starts_with("departure") {
                Some(positions.get(&idx).unwrap())
            } else {
                None
            }
        })
        .copied()
        .collect::<Vec<_>>();

    let my_departure_values = my_ticket
        .iter()
        .enumerate()
        .filter_map(|(i, value)| {
            if rules_that_start_with_departure.contains(&i) {
                Some(value)
            } else {
                None
            }
        })
        .copied()
        .collect::<Vec<_>>();

    aoc_lib::set_part_2!(my_departure_values.iter().product::<usize>());

    Ok(())
}
