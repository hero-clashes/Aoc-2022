use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    
    let mut index = 0;
    let mut data: Vec<i64> = Vec::new();
    data.push(0);

    for i in contents.lines(){
        if i.is_empty(){
            index += 1;
            data.push(0);
        } else {
            let num = i64::from_str_radix(i,10).unwrap();
            data[index] += num;
        }
        
    }

    
    let mut data = data.iter().enumerate().collect::<Vec<_>>();
    data.sort_by(|a,b| a.1.cmp(b.1));
    data.reverse();
    
    println!("{}",data[0..3].iter().fold(0, |acc, &x| acc + x.1));
}
