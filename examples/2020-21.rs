// See: https://adventofcode.com/2020/day/21
// ## --- Day 21: Allergen Assessment ---
//
// You reach the train's last stop and the closest you can get to your vacation island without
// getting wet. There aren't even any boats here, but nothing can stop you now: you build a raft.
// You just need a few days' worth of food for your journey.
//
// You don't speak the local language, so you can't read any ingredients lists. However, sometimes,
// allergens are listed in a language you *do* understand. You should be able to use this
// information to determine which ingredient contains which allergen and work out which foods are
// safe to take with you on your trip.
//
// You start by compiling a list of foods (your puzzle input), one food per line. Each line includes
// that food's *ingredients list* followed by some or all of the allergens the food contains.
//
// Each allergen is found in exactly one ingredient. Each ingredient contains zero or one allergen.
// *Allergens aren't always marked*; when they're listed (as in `(contains nuts, shellfish)` after
// an ingredients list), the ingredient that contains each listed allergen will be *somewhere in the
// corresponding ingredients list*. However, even if an allergen isn't listed, the ingredient that
// contains that allergen could still be present: maybe they forgot to label it, or maybe it was
// labeled in a language you don't know.
//
// For example, consider the following list of foods:
//
// `mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
// trh fvjkl sbzzf mxmxvkd (contains dairy)
// sqjhc fvjkl (contains soy)
// sqjhc mxmxvkd sbzzf (contains fish)
// `
//
// The first food in the list has four ingredients (written in a language you don't understand):
// `mxmxvkd`, `kfcds`, `sqjhc`, and `nhms`. While the food might contain other allergens, a few
// allergens the food definitely contains are listed afterward: `dairy` and `fish`.
//
// The first step is to determine which ingredients *can't possibly* contain any of the allergens in
// any food in your list. In the above example, none of the ingredients `kfcds`, `nhms`, `sbzzf`, or
// `trh` can contain an allergen. Counting the number of times any of these ingredients appear in
// any ingredients list produces *`5`*: they all appear once each except `sbzzf`, which appears
// twice.
//
// Determine which ingredients cannot possibly contain any of the allergens in your list. *How many
// times do any of those ingredients appear?*
//
//
// ## --- Part Two ---
//
// Now that you've isolated the inert ingredients, you should have enough information to figure out
// which ingredient contains which allergen.
//
// In the above example:
//
// * `mxmxvkd` contains `dairy`.
// * `sqjhc` contains `fish`.
// * `fvjkl` contains `soy`.
//
// Arrange the ingredients *alphabetically by their allergen* and separate them by commas to produce
// your *canonical dangerous ingredient list*. (There should *not be any spaces* in your canonical
// dangerous ingredient list.) In the above example, this would be *`mxmxvkd,sqjhc,fvjkl`*.
//
// Time to stock your raft with supplies. *What is your canonical dangerous ingredient list?*

use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};

macro_rules! c {
    ($cap:expr, $n:expr) => {
        $cap.get($n).unwrap().as_str().trim()
    };
}

fn main() -> Result<()> {
    let input = include_str!("./input/2020-21.txt").trim();
    let re_ingredients = Regex::new(r"(.*)(\(.*\))")?;
    let re_allergens = Regex::new(r"\(contains (.*)\)")?;

    let mut ingredients_counts = HashMap::new();
    let mut allergens_to_ingredients = vec![];
    let mut ingredients_to_allergens = HashMap::new();
    for line in input.lines() {
        let captures = re_ingredients.captures(line).unwrap();
        let ingredients = c!(captures, 1).split_whitespace().collect::<Vec<_>>();
        let captures = re_allergens.captures(c!(captures, 2)).unwrap();
        let allergens = c!(captures, 1).split(", ").collect::<Vec<_>>();

        allergens_to_ingredients.push((allergens.clone(), ingredients.clone()));

        for ing in &ingredients {
            *ingredients_counts.entry(*ing).or_insert(0) += 1;
            for al in &allergens {
                ingredients_to_allergens.entry(*ing).or_insert(HashSet::new()).insert(*al);
            }
        }
    }

    // find ingredients without any allergens
    for ing in ingredients_counts.keys() {
        let possible = ingredients_to_allergens.get_mut(ing).unwrap();
        for al in possible.clone() {
            for (allergens, ingredients) in &allergens_to_ingredients {
                if allergens.contains(&al) && !ingredients.contains(&ing) {
                    possible.remove(&al);
                }
            }
        }
    }

    let no_allergens_count = ingredients_to_allergens
        .iter()
        .filter_map(|(ing, al)| if al.is_empty() { ingredients_counts.get(ing) } else { None })
        .sum::<usize>();
    aoc_lib::set_part_1!(no_allergens_count);

    // remove all ingredients without any allergens
    ingredients_to_allergens.retain(|_, al| !al.is_empty());

    // map remaining allergens directly to ingredients
    // NOTE: this assumes that there will always be a 1->1 mapping on each step
    while ingredients_to_allergens
        .iter()
        .any(|(_, als)| als.len() > 1)
    {
        let singles = ingredients_to_allergens
            .iter()
            .filter_map(|(_, als)| {
                if als.len() == 1 {
                    Some(als.iter().next().unwrap())
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for single in singles {
            for (_, als) in ingredients_to_allergens.iter_mut() {
                if als.len() > 1 {
                    als.remove(single);
                }
            }
        }
    }

    let mut foods_to_avoid = ingredients_to_allergens
        .iter()
        .map(|(ing, als)| (als.iter().next().unwrap(), ing))
        .collect::<Vec<_>>();
    foods_to_avoid.sort();
    let foods_to_avoid = foods_to_avoid
        .iter()
        .map(|(_, ing)| ing.to_string())
        .collect::<Vec<_>>()
        .join(",");

    aoc_lib::set_part_2!(foods_to_avoid);

    Ok(())
}
