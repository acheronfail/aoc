// See: https://adventofcode.com/2020/day/18
// ## --- Day 18: Operation Order ---
//
// As you look out the window and notice a heavily-forested continent slowly appear over the
// horizon, you are interrupted by the child sitting next to you. They're curious if you could help
// them with their math homework.
//
// Unfortunately, it seems like this "math" [follows different rules][1] than you remember.
//
// The homework (your puzzle input) consists of a series of expressions that consist of addition
// (`+`), multiplication (`*`), and parentheses (`(...)`). Just like normal math, parentheses
// indicate that the expression inside must be evaluated before it can be used by the surrounding
// expression. Addition still finds the sum of the numbers on both sides of the operator, and
// multiplication still finds the product.
//
// However, the rules of *operator precedence* have changed. Rather than evaluating multiplication
// before addition, the operators have the *same precedence*, and are evaluated left-to-right
// regardless of the order in which they appear.
//
// For example, the steps to evaluate the expression `1 + 2 * 3 + 4 * 5 + 6` are as follows:
//
// `*1 + 2* * 3 + 4 * 5 + 6
// *3   * 3* + 4 * 5 + 6
// *9   + 4* * 5 + 6
// *13   * 5* + 6
// *65   + 6*
// *71*
// `
//
// Parentheses can override this order; for example, here is what happens if parentheses are added
// to form `1 + (2 * 3) + (4 * (5 + 6))`:
//
// `1 + *(2 * 3)* + (4 * (5 + 6))
// *1 +    6*    + (4 * (5 + 6))
// 7      + (4 * *(5 + 6)*)
// 7      + *(4 *   11   )*
// *7      +     44*
// *51*
// `
//
// Here are a few more examples:
//
// * `2 * 3 + (4 * 5)` becomes *`26`*.
// * `5 + (8 * 3 + 9 + 3 * 4 * 3)` becomes *`437`*.
// * `5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))` becomes *`12240`*.
// * `((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2` becomes *`13632`*.
//
// Before you can help with the homework, you need to understand it yourself. *Evaluate the
// expression on each line of the homework; what is the sum of the resulting values?*
//
// [1] https://www.youtube.com/watch?v=3QtRK7Y2pPU&t=15
//
//
// ## --- Part Two ---
//
// You manage to answer the child's questions and they finish part 1 of their homework, but get
// stuck when they reach the next section: *advanced* math.
//
// Now, addition and multiplication have *different* precedence levels, but they're not the ones
// you're familiar with. Instead, addition is evaluated *before* multiplication.
//
// For example, the steps to evaluate the expression `1 + 2 * 3 + 4 * 5 + 6` are now as follows:
//
// `*1 + 2* * 3 + 4 * 5 + 6
// 3   * *3 + 4* * 5 + 6
// 3   *   7   * *5 + 6*
// *3   *   7*   *  11
// *21       *  11*
// *231*
// `
//
// Here are the other examples from above:
//
// * `1 + (2 * 3) + (4 * (5 + 6))` still becomes *`51`*.
// * `2 * 3 + (4 * 5)` becomes *`46`*.
// * `5 + (8 * 3 + 9 + 3 * 4 * 3)` becomes *`1445`*.
// * `5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))` becomes *`669060`*.
// * `((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2` becomes *`23340`*.
//
// *What do you get if you add up the results of evaluating the homework problems using these new
// rules?*

use anyhow::Result;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref EXPR_RE: Regex = Regex::new(r"\(([^()]+?)\)").unwrap();
    static ref WHITESPACE_RE: Regex = Regex::new(r"\s").unwrap();
    static ref WORD_RE: Regex = Regex::new(r"\b").unwrap();
}

#[derive(Copy, Clone)]
enum Instruction {
    Add,
    Mult,
    Number(usize),
}

impl Instruction {
    pub fn n(self) -> usize {
        match self {
            Instruction::Number(n) => n,
            _ => panic!("n() called on non Number variant"),
        }
    }
}

fn split_expression(s: &str) -> Vec<Instruction> {
    let expression = WHITESPACE_RE.replace_all(s, "");
    WORD_RE
        .split(&expression)
        .filter(|instr| !instr.is_empty())
        .map(|s| match s {
            "+" => Instruction::Add,
            "*" => Instruction::Mult,
            n => Instruction::Number(n.parse().unwrap()),
        })
        .collect()
}

fn eval_left_to_right(s: &str) -> usize {
    let mut instructions = split_expression(s);

    // there's always more than one instruction, so the first must be a number
    let mut result = instructions.remove(0).n();
    for pair in instructions.chunks(2) {
        let (op, operand) = (&pair[0], &pair[1]);
        match op {
            Instruction::Add => result += operand.n(),
            Instruction::Mult => result *= operand.n(),
            _ => unreachable!(),
        }
    }

    result
}

fn eval_add_over_mult(s: &str) -> usize {
    let mut instructions = split_expression(s);

    // there's always more than one instruction, so the first must be a number
    let mut result = instructions.remove(0).n();
    let mut numbers = vec![];
    for pair in instructions.chunks(2) {
        let (op, operand) = (&pair[0], &pair[1].n());
        match op {
            // collapse all Add instructions
            Instruction::Add => result += operand,
            // build up the remaining Mult instructions
            Instruction::Mult => {
                numbers.push(result);
                result = *operand;
            }
            _ => unreachable!(),
        }
    }
    numbers.push(result);

    numbers.iter().product::<usize>()
}

fn eval_str(s: &str, f: fn(&str) -> usize) -> usize {
    let mut s = s.to_string();
    while EXPR_RE.is_match(&s) {
        let new_s = EXPR_RE
            .replace(&s, |captures: &Captures| {
                format!("{}", f(captures.get(1).unwrap().as_str()))
            })
            .to_string();

        s = new_s;
    }

    f(&s)
}

fn main() -> Result<()> {
    let input = include_str!("./input/2020-18.txt").trim();
    let expressions = input.lines().collect::<Vec<_>>();

    aoc_lib::set_part_1!(expressions
        .iter()
        .map(|s| eval_str(s, eval_left_to_right))
        .sum::<usize>());

    aoc_lib::set_part_2!(expressions
        .iter()
        .map(|s| eval_str(s, eval_add_over_mult))
        .sum::<usize>());

    Ok(())
}
