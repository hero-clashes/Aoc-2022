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

    let mut choosen_index = 0;
    'outer_loop: for (index,ch) in contents.chars().enumerate(){
        let mut set = HashSet::new();

        for i in index..index + 4{
            let ch2 = contents.chars().nth(i).unwrap();
            if !set.insert(ch2){
                continue 'outer_loop;
            }
        }
        choosen_index = index + 4;
        break;
    }

    println!("{choosen_index}");
}
