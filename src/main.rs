use std::{fs, collections::HashSet};

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

    'outer_loop: for line in contents.lines() {
        let first_half = &line[0..line.len()/2];
        let second_half = &line[line.len()/2..line.len()];
        let mut first_half_set = HashSet::new();
        for ch in first_half.chars(){
            first_half_set.insert(ch);
        }
        for ch in second_half.chars(){
            if first_half_set.contains(&ch){
                sum += Convert_Char_To_Priorities(ch);
                continue 'outer_loop; 
            }
        }
    }

    println!("{sum}");
}
