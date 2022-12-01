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

    
    let max_index = data.iter().enumerate().max_by_key(|(_, &value)| value).map(|(idx, _)| idx).unwrap();
    println!("{}",data[max_index]);
}
