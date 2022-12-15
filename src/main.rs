use array2d::Array2D;
use core::num;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::iter::Peekable;
use std::str::Lines;

fn dest(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
    // (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f32).sqrt() as f32
}
fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    let mut X = 1;
    let mut current_cycle = 1;
    let check = vec![20, 60, 100, 140, 180, 220];
    for line in contents.lines() {
        match line.split_once(" ") {
            Some(("addx", str)) => {
                if check.contains(&current_cycle) {
                    sum += X * current_cycle;
                }
                current_cycle += 1;
                if check.contains(&current_cycle) {
                    sum += X * current_cycle;
                }
                current_cycle += 1;
                X += str.parse::<i32>().unwrap();
            }
            Some((&_, _)) => todo!(),
            None => {
                if line == "noop" {
                    if check.contains(&current_cycle) {
                        sum += X * current_cycle;
                    }
                    current_cycle += 1;
                }
            }
        }
    }
    println!("{}", sum);
}
