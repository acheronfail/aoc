// See: https://adventofcode.com/2019/day/25
// ## --- Day 25: Cryostasis ---
//
// As you approach Santa's ship, your sensors report two important details:
//
// First, that you might be too late: the internal temperature is `-40` degrees.
//
// Second, that one faint life signature is somewhere on the ship.
//
// The airlock door is locked with a code; your best option is to send in a small droid to
// investigate the situation. You attach your ship to Santa's, break a small hole in the hull, and
// let the droid run in before you seal it up again. Before your ship starts freezing, you detach
// your ship and set it to automatically stay within range of Santa's ship.
//
// This droid can follow basic instructions and report on its surroundings; you can communicate with
// it through an [Intcode][1] program (your puzzle input) running on an [ASCII-capable][2] computer.
//
// As the droid moves through its environment, it will describe what it encounters. When it says
// `Command?`, you can give it a single instruction terminated with a newline (ASCII code `10`).
// Possible instructions are:
//
// * *Movement* via `north`, `south`, `east`, or `west`.
// * To *take* an item the droid sees in the environment, use the command `take <name of item>`. For
// example, if the droid reports seeing a `red ball`, you can pick it up with `take red ball`.
// * To *drop* an item the droid is carrying, use the command `drop <name of item>`. For example, if
// the droid is carrying a `green ball`, you can drop it with `drop green ball`.
// * To get a *list of all of the items* the droid is currently carrying, use the command `inv` (for
// "inventory").
//
// Extra spaces or other characters aren't allowed - instructions must be provided precisely.
//
// Santa's ship is a *Reindeer-class starship*; these ships use pressure-sensitive floors to
// determine the identity of droids and crew members. The standard configuration for these starships
// is for all droids to weigh exactly the same amount to make them easier to detect. If you need to
// get past such a sensor, you might be able to reach the correct weight by carrying items from the
// environment.
//
// Look around the ship and see if you can find the *password for the main airlock*.
//
// [1] 9
// [2] 17
//
//
// ## --- Part Two ---
//
// As you move through the main airlock, the air inside the ship is already heating up to reasonable
// levels. Santa explains that he didn't notice you coming because he was just taking a quick nap.
// The ship wasn't frozen; he just had the thermostat set to "North Pole".
//
// You make your way over to the navigation console. It beeps. "Status: Stranded. Please supply
// measurements from *49 stars* to recalibrate."
//
// "49 stars? But the Elves told me you needed fifty--"
//
// Santa just smiles and nods his head toward the window. There, in the distance, you can see the
// center of the Solar System: the Sun!
//
// The navigation console beeps again.

use anyhow::Result;
use aoc_lib::utils::prompt_from_stdin;
use std::io::Write;

use _2019::io::{IntRead, IntWrite};
use _2019::{ints_from_str, Int, Program, StopReason};

struct Bits(Vec<u8>);

impl IntRead for &mut Bits {
    fn int_read(&mut self) -> Option<Int> {
        match self.0.len() {
            0 => None,
            _ => Some(self.0.remove(0) as Int),
        }
    }
}

impl IntWrite for &mut Bits {
    fn int_write(&mut self, int: Int) {
        self.0.push(int as u8);
    }
}

fn main() -> Result<()> {
    let input = include_str!("./input/2019-25.txt").trim();
    let ints = ints_from_str(input);

    let mut droid = Program::new(ints);
    let mut input = Bits(vec![]);
    let mut output = Bits(vec![]);
    loop {
        match droid.run(&mut input, &mut output) {
            StopReason::Halt => {
                println!("{}", String::from_utf8_lossy(&output.0));
                break;
            }
            StopReason::WaitingForInput => {
                let output = output.0.drain(..).collect::<Vec<_>>();
                let answer = prompt_from_stdin(Some(&format!(
                    "{} ",
                    String::from_utf8_lossy(&output).trim_end()
                )))?;
                input.0.write_all(format!("{}\n", answer).as_bytes())?;
                println!();
            }
        }
    }

    // CORRECT: brochure + ice cream + asterisk + bottle + heater
    // BY WEIGHT:
    //   spool, brochure
    //   space heater, asterisk, bottle
    //   shell, jam
    //   ice cream
    //
    // ice cream + all small == too heavy
    // ice cream + all small - asterisk == too heavy
    // ice cream + all small - asterisk - space heater == too light
    // ice cream + all small - asterisk - bottle == too light
    // ice cream + all small - asterisk - spool == too heavy
    // ice cream + all small - asterisk - brochure == too heavy
    //
    // shell + space heater == too light
    // shell + jam == too light
    // shell + brochure == too light
    // shell + ice cream == too heavy
    //
    // ice cream + brochure == too light
    // ice cream + space heater == too light
    // ice cream + spool == too light
    // ice cream + asterisk == too light
    // ice cream + bottle == too light
    // ice cream + jam == too heavy
    // ice cream + shell == too heavy
    //
    // space heater + shell == too light
    // space heater + ice cream == too light
    // space heater + brochure == too light
    // space heater + jam == too light
    //
    // jam + space heater == too light
    // jam + shell == too light
    // jam + brochure == too light
    // jam + ice cream == too heavy
    //
    // brochure + space heater == too light
    // brochure + shell == too light
    // brochure + jam == too light
    // brochure + ice cream == too light

    aoc_lib::set_part_1!(2105377);

    // TODO: can't do part 2 yet, need to complete all the other puzzles
    // aoc_lib::set_part_2!(0);

    Ok(())
}
