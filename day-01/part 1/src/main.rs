use std::fs;

fn main(){
    let mut res = 0;
    let input = fs::read_to_string("./input.txt").expect("File not found");
    let vec = input.split("\n");

    for s in vec {
        let val = s.parse::<i32>().unwrap();
        res += val;
    }
    println!("{}",res);
    
}