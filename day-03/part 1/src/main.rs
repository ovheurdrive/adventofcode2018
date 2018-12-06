use std::env::args;
#[macro_use] extern crate scan_fmt;

fn main() {
    println!("{}", run(&args().nth(1).expect("Please provide an input")))
}

fn run(input: &str) -> u32 {
    let mut fabric: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let mut res = 0;
    for line in input.lines() {
        let (id, x, y, w, h) = scan_fmt!(line, "#{} @ {},{}: {}x{}", i32, usize, usize, usize, usize);
        for i in x.unwrap()..x.unwrap()+w.unwrap() {
            for j in y.unwrap()..y.unwrap()+h.unwrap() {
                if fabric[i][j] > 0 {
                    res += 1;
                    fabric[i][j] = -1;
                }
                else if fabric[i][j] == 0 {
                    fabric[i][j] = id.unwrap();
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 4);
    }
}