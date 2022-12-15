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

    let mut tail_poss = vec![(0, 0); 10];
    visted_set.insert((0, 0));

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
                for _i in 0..number {
                    let new_pos = (tail_poss[0].0 + dir.0, tail_poss[0].1 + dir.1);
                    // for (i, mut tail_pos) in tail_poss.iter_mut().enumerate() {
                    tail_poss[0] = new_pos;
                    'losop: for i in 1..tail_poss.len() {
                        let mut tail_pos = &tail_poss[i];
                        let dir_vic = (
                            tail_poss[i - 1].0 - tail_pos.0,
                            tail_poss[i - 1].1 - tail_pos.1,
                        );
                        if (dir_vic.0 as i32).abs() <= 1 && (dir_vic.1 as i32).abs() <= 1 {
                            continue;
                        }
                        let mut tail_pos_new = (tail_pos.0 + dir_vic.0.clamp(-1, 1), tail_pos.1 + dir_vic.1.clamp(-1, 1));

                        tail_poss[i] = tail_pos_new;
                    }
                    visted_set.insert(*tail_poss.last().unwrap());
                }
            }
            Some((&_, _)) => todo!(),
            None => panic!("shouldn't happens"),
        }
    }

    println!("{}", visted_set.len());
}
