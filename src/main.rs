use array2d::Array2D;
use core::num;
use inpt::{inpt, Inpt};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::iter::Peekable;
use std::str::Lines;

#[derive(Inpt, Default, Debug, Clone)]
#[inpt(regex = r"Operation: new = (.+) (.) (.+)")]
struct operation {
    lhs: String,
    op: String,
    rhs: String,
}

impl operation {
    fn compute(&self, old: u64) -> u64 {
        let lhs_val = match self.lhs.as_str() {
            "old" => old,
            str => str.parse().unwrap(),
        };

        let rhs_val = match self.rhs.as_str() {
            "old" => old,
            str => str.parse().unwrap(),
        };

        match self.op.as_str() {
            "*" => lhs_val * rhs_val,
            "+" => lhs_val + rhs_val,
            "-" => lhs_val - rhs_val,
            "/" => lhs_val / rhs_val,
            _ => todo!()
        }
    }
}
#[derive(Inpt, Default, Debug, Clone)]
#[inpt(
    regex = r"Test: divisible by (\d+)    If true: throw to monkey (\d+)    If false: throw to monkey (\d+)"
)]
struct test {
    divisible_by: u64,
    true_brench: u64,
    false_brench: u64,
}
impl test {
    fn item_go_to(&self, worry: u64) -> u64 {
        if worry % self.divisible_by == 0 {
            self.true_brench
        } else {
            self.false_brench
        }
    }
}
#[derive(Inpt, Default, Debug, Clone)]

struct Monkey {
    items: Vec<u64>,
    oper: operation,
    test: test,
    number_of_items_inspected: u64
}

#[derive(Inpt, Default, Debug)]
#[inpt(regex = r"Starting items:")]
struct items {
    #[inpt(after)]
    i: Vec<u64>,
}
fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut Monkeys = Vec::new();

    for ch in &contents.lines().chunks(7) {
        let (f, second, third, fourth, fifth, sixth, s) = ch.collect_tuple().unwrap();
        let mut monk = Monkey::default();
        monk.items = inpt::<items>(second).unwrap().i;
        monk.oper = inpt(third).unwrap();
        monk.test = inpt(&(fourth.to_string() + &fifth.to_string() + &sixth.to_string())).unwrap();

        Monkeys.push(monk);
    }

    const number_of_rounds: u64 = 10000;

    for round in 1..=number_of_rounds {
        let s = 4 + Monkeys[0].test.false_brench;
        for i in 0..Monkeys.len() {
            let current_monkey = Monkeys[i].clone();
            for item in &current_monkey.items {
                Monkeys[i].number_of_items_inspected+= 1;
                let mut worry = *item;
                worry = current_monkey.oper.compute(worry);
                worry %= Monkeys.iter().map(|m| m.test.divisible_by).product::<u64>();

                Monkeys[current_monkey.test.item_go_to(worry) as usize]
                    .items
                    .push(worry);
            }

            Monkeys[i].items.clear();
        }
    }

    Monkeys.sort_by(|a,b| a.number_of_items_inspected.cmp(&b.number_of_items_inspected));
    Monkeys.reverse();
    let score = Monkeys[0].number_of_items_inspected * Monkeys[1].number_of_items_inspected;
    println!("{}",score);
}
