use std::env::args;

fn main() {
    println!("{}", run(args().nth(1).expect("Please provide an input")))
}

fn run(input: String) -> i32 {
    let (mut twos, mut threes): (i32, i32) = (0, 0);
    let  v: Vec<&str> = input.lines().collect();
    for elem in v {
        let (mut has_two, mut has_three) = (false, false);
        for letter in elem.chars() {
            if elem.matches(letter).count() == 2 && !has_two {
                twos += 1;
                has_two = true;
            }
            if elem.matches(letter).count() == 3 && !has_three {
                threes += 1;
                has_three = true;
            }

            if has_three && has_two {
                break;
            }
        }
    }
    twos * threes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let input = "abcdef\nbababc\nabbcde\naabcdd\nabcccd\nabcdee\nababab";
        assert_eq!(run(input), 12);
    }
}
