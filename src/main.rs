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

    let mut output = String::new();

     
    let mut X = 1;
    let mut current_cycle = 0;
    for line in contents.lines() {
        match line.split_once(" ") {
            Some(("addx", str)) => {
              
                if [X-1, X, X+1].contains(&((current_cycle%40))){
                    output.push('#');
                } else {
                    output.push('.');
                }
                current_cycle += 1;
                if current_cycle%40 == 0{
                    output.push('\n');
                }
                
                if [X-1, X, X+1].contains(&((current_cycle%40))){
                    output.push('#');
                } else {
                    output.push('.');
                }
                current_cycle += 1;
                if current_cycle%40 == 0{
                    output.push('\n');
                }

                X += str.parse::<i32>().unwrap();
            }
            Some((&_, _)) => todo!(),
            None => {
                if line == "noop" {
                    
                    if [X-1, X, X+1].contains(&((current_cycle%40))){
                        output.push('#');
                    } else {
                        output.push('.');
                    }
                    current_cycle += 1;
                    if current_cycle%40 == 0{
                        output.push('\n');
                    }

                }
            }
        }
    }
    println!("{}", output);
}
