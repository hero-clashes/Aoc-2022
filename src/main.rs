use std::{fs, collections::HashSet};
use itertools::Itertools;

fn Convert_Char_To_Priorities(ch: char)-> u32{
    if 'a' <= ch && ch <= 'z'{
        ch as u32 - 'a' as u32 + 1
    } else {
        ch as u32 - 'A' as u32 + 27
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;

    'outer_loop: for ch in &contents.lines().chunks(3) {
        let (first_line,second_line,thrid_line) = ch.collect_tuple().unwrap();
        let first_line_set = HashSet::<_>::from_iter(first_line.to_string().chars());
        let second_line_set = HashSet::<_>::from_iter(second_line.chars());

        for ch in thrid_line.chars(){
            if first_line_set.contains(&ch) && second_line_set.contains(&ch){
                sum += Convert_Char_To_Priorities(ch);
                continue 'outer_loop; 
            }
        }
    }

    println!("{sum}");
}
