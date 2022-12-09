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

    let mut visted_set = HashSet::new();

    let mut current_pos = (0, 0);
    let mut tail_pos = current_pos;
    visted_set.insert(current_pos);

    for line in contents.lines() {
        match line.split_once(' ') {
            Some((dir, str)) => {
                let number: i32 = str.parse().unwrap();
                let dir = match dir {
                    "U" => (0, -1),
                    "D" => (0, 1),
                    "R" => (1, 0),
                    "L" => (-1, 0),
                    _ => todo!(),
                };
                'losop: for i in 0..number {
                    let new_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);

                    let mut vec = Vec::new();
                    for dx in [-1, 0, 1] {
                        for dy in [-1, 0, 1] {
                            let pos = (tail_pos.0 + dx, tail_pos.1 + dy);
                            if new_pos == pos {
                                current_pos = new_pos;
                                continue 'losop;
                            }
                            vec.push((pos, dest(new_pos, pos)));
                        }
                    }
                    vec.sort_by(|a, b| a.1.cmp(&b.1));
                    let tail_pos_new = vec[0];
                    tail_pos = tail_pos_new.0;
                    visted_set.insert(tail_pos_new.0);

                    current_pos = new_pos;
                }
            }
            Some((&_, _)) => todo!(),
            None => panic!("shouldn't happens"),
        }
    }
   
    
    println!("{}", visted_set.len());
}
