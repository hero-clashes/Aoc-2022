use core::num;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    fs,
    ops::RangeToInclusive,
};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut crates_on_top = String::new();
    let mut number_of_stacks = 0;
    let mut number_of_lines = 0;
    for (index, line) in contents.lines().enumerate() {
        if line.starts_with(" 1") {
            let r = Regex::new(r"(\d+)\s+$").unwrap();
            let iter = r.captures(&line).unwrap();

            number_of_stacks =
                i32::from_str_radix(iter.get(iter.len() - 1).unwrap().as_str(), 10).unwrap();
            number_of_lines = index;
            break;
        }
    }

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for i in 0..number_of_stacks {
        stacks.push(VecDeque::new());
    }

    let lines = contents.lines().collect_vec();
    for line in &lines[0..number_of_lines] {
        for i in 0..number_of_stacks {
            if i == 0 {
                let curr = line.chars().nth(1).unwrap();
                if curr != ' ' {
                    stacks[i as usize].push_back(curr);
                }
            } else {
                let curr = line.chars().nth((1 + (4 * i)) as usize).unwrap();
                if curr != ' ' {
                    stacks[i as usize].push_back(curr);
                }
            }
        }
    }

    for line in &lines[number_of_lines + 2..] {
        let reg = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let items = reg.captures(&line).unwrap();
        let number_of_items = i32::from_str_radix(items.get(1).unwrap().as_str(), 10).unwrap();
        let from_stack = i32::from_str_radix(items.get(2).unwrap().as_str(), 10).unwrap();
        let to_stack = i32::from_str_radix(items.get(3).unwrap().as_str(), 10).unwrap();
        let mut temp = VecDeque::new();
        for i in 0..number_of_items {
            let item = stacks[(from_stack - 1) as usize].pop_front().unwrap();
            temp.push_front(item);
        }
        for i in 0..number_of_items{
            stacks[(to_stack - 1) as usize].push_front(temp.pop_front().unwrap());
        }
    }

    for stack in stacks {
        crates_on_top.push(*stack.front().unwrap());
    }
    println!("{crates_on_top}");
}
