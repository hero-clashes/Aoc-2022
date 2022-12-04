use itertools::Itertools;
use regex::Regex;
use std::{collections::HashSet, fs, ops::RangeToInclusive};

fn range_included(
    first_range: std::ops::RangeInclusive<i64>,
    second_range: std::ops::RangeInclusive<i64>,
) -> bool {
    for num in first_range {
        if second_range.contains(&num) {
            return true;
        }
    }
    return false;
}

fn range_included_two_ways(
    first_range: std::ops::RangeInclusive<i64>,
    second_range: std::ops::RangeInclusive<i64>,
) -> bool {
    let first = range_included(first_range.clone(), second_range.clone());

    let second = range_included(second_range, first_range);

    first || second
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    let reg = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for line in contents.lines() {
        let matches = reg.captures(line).unwrap();
        let first_range = i64::from_str_radix(matches.get(1).unwrap().as_str(), 10).unwrap()
            ..=i64::from_str_radix(matches.get(2).unwrap().as_str(), 10).unwrap();
        let second_range = i64::from_str_radix(matches.get(3).unwrap().as_str(), 10).unwrap()
            ..=i64::from_str_radix(matches.get(4).unwrap().as_str(), 10).unwrap();
        sum += range_included_two_ways(first_range, second_range) as i64;
    }

    println!("{sum}");
}
