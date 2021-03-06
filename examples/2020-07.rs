// See: https://adventofcode.com/2020/day/7
// ## --- Day 7: Handy Haversacks ---
//
// You land at the regional airport in time for your next flight. In fact, it looks like you'll even
// have time to grab some food: all flights are currently delayed due to *issues in luggage
// processing*.
//
// Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and
// their contents; bags must be color-coded and must contain specific quantities of other color-coded
// bags. Apparently, nobody responsible for these regulations considered how long they would take to
// enforce!
//
// For example, consider the following rules:
//
// `light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.
// `
//
// These rules specify the required contents for 9 bag types. In this example, every `faded blue` bag
// is empty, every `vibrant plum` bag contains 11 bags (5 `faded blue` and 6 `dotted black`), and so
// on.
//
// You have a `*shiny gold*` bag. If you wanted to carry it in at least one other bag, how many
// different bag colors would be valid for the outermost bag? (In other words: how many colors can,
// eventually, contain at least one `shiny gold` bag?)
//
// In the above rules, the following options would be available to you:
//
// * A `bright white` bag, which can hold your `shiny gold` bag directly.
// * A `muted yellow` bag, which can hold your `shiny gold` bag directly, plus some other bags.
// * A `dark orange` bag, which can hold `bright white` and `muted yellow` bags, either of which could
// then hold your `shiny gold` bag.
// * A `light red` bag, which can hold `bright white` and `muted yellow` bags, either of which could
// then hold your `shiny gold` bag.
//
// So, in this example, the number of bag colors that can eventually contain at least one `shiny gold`
// bag is `*4*`.
//
// *How many bag colors can eventually contain at least one `shiny gold` bag?* (The list of rules is
// quite long; make sure you get all of it.)
//
//
// ## --- Part Two ---
//
// It's getting pretty expensive to fly these days - not because of ticket prices, but because of the
// ridiculous number of bags you need to buy!
//
// Consider again your `shiny gold` bag and the rules from the above example:
//
// * `faded blue` bags contain `0` other bags.
// * `dotted black` bags contain `0` other bags.
// * `vibrant plum` bags contain `11` other bags: 5 `faded blue` bags and 6 `dotted black` bags.
// * `dark olive` bags contain `7` other bags: 3 `faded blue` bags and 4 `dotted black` bags.
//
// So, a single `shiny gold` bag must contain 1 `dark olive` bag (and the 7 bags within it) plus 2
// `vibrant plum` bags (and the 11 bags within *each* of those): `1 + 1*7 + 2 + 2*11` = `*32*` bags!
//
// Of course, the actual rules have a small chance of going several levels deeper than this example; be
// sure to count all of the bags, even if the nesting becomes topologically impractical!
//
// Here's another example:
//
// `shiny gold bags contain 2 dark red bags.
// dark red bags contain 2 dark orange bags.
// dark orange bags contain 2 dark yellow bags.
// dark yellow bags contain 2 dark green bags.
// dark green bags contain 2 dark blue bags.
// dark blue bags contain 2 dark violet bags.
// dark violet bags contain no other bags.
// `
//
// In this example, a single `shiny gold` bag must contain `*126*` other bags.
//
// *How many individual bags are required inside your single `shiny gold` bag?*

use regex::Regex;
use std::collections::{HashMap, HashSet};

fn n_bag_colors_containing(color: &str, parent_rule_map: &HashMap<String, Vec<String>>) -> usize {
    fn search(parents: &HashMap<String, Vec<String>>, colors: &mut HashSet<String>, color: &str) {
        if let Some(parent_bags) = parents.get(color) {
            for parent_bag in parent_bags {
                colors.insert(parent_bag.clone());
                search(parents, colors, parent_bag);
            }
        }
    }

    let mut colors = HashSet::new();
    search(parent_rule_map, &mut colors, color);
    colors.len()
}

fn n_bags_required_inside(
    color: &str,
    child_rule_map: &HashMap<String, Vec<(String, usize)>>,
) -> usize {
    child_rule_map.get(color).map_or(0, |children| {
        children.iter().fold(0, |result, (color, count)| {
            result + count + count * n_bags_required_inside(color, &child_rule_map)
        })
    })
}

fn main() {
    let input = include_str!("./input/2020-07.txt");
    let mut parent_rule_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut child_rule_map: HashMap<String, Vec<(String, usize)>> = HashMap::new();

    input.trim().lines().for_each(|line| {
        let (bag_color, bag_contents) =
            aoc_lib::utils::string_split2(" bags contain ", line.trim());

        let child_rules = child_rule_map
            .entry(bag_color.to_string())
            .or_insert(vec![]);
        if !bag_contents.contains("no other bags") {
            let re = Regex::new(r"(\d+)(.+?)bags?").unwrap();
            for cap in re.captures_iter(&bag_contents) {
                let child_name = (&cap[2]).trim().to_string();
                let child_count = cap[1].parse::<usize>().unwrap();
                child_rules.push((child_name.clone(), child_count));

                let parent_rules = parent_rule_map.entry(child_name).or_insert(vec![]);
                parent_rules.push(bag_color.to_string());
            }
        }
    });

    aoc_lib::set_part_1!(n_bag_colors_containing("shiny gold", &parent_rule_map));
    aoc_lib::set_part_2!(n_bags_required_inside("shiny gold", &child_rule_map));
}
