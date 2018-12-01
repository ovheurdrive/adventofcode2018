use std::fs;

fn main(){
    let mut current_f = 0;
    let mut res = i32::max_value();

    let input = fs::read_to_string("./input.txt").expect("File not found");

    let mut frequencies: Vec<i32> = Vec::new();
    frequencies.push(current_f);

    while res == i32::max_value() {
        let vec = input.split("\n");
        for s in vec {
            let val = s.parse::<i32>().unwrap();
            current_f += val;

            if !frequencies.contains(&current_f) {
                frequencies.push(current_f);
            }
            else {
                res = current_f;
                break;
            }
        }
        //println!("{:?}", frequencies);
    }
    
    println!("{}",res);
    
}