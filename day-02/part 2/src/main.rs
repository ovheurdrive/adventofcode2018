use std::env::args;

fn main() {
    println!("{}", run(args().nth(1).expect("Please provide an input")).unwrap())
}

fn run(input: String) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();
    for (index, elem) in lines.iter().enumerate() {
        for elem2 in lines.iter().skip(index +1){
            if diff(elem, elem2).unwrap_or(0) == 1 {
                return Some(common(elem,elem2))
            }
        }
    }
    Some(String::from("No string in input matches"))
}

fn diff(a: &str, b: &str) -> Result<i32, String> {
    let (mut iter_a, mut iter_b, mut count) = (a.chars(), b.chars(), 0);
    loop {
        match (iter_a.next(), iter_b.next()) {
            (Some(x), Some(y)) => if x != y {
                count += 1
            },
            (None, None) => return Ok(count),
            _ => return Err("Args of different length".to_string()),
        }
    }
}

fn common(a: &str, b: &str) -> String {
    let mut res = String::new();
    let ( iter_a, iter_b) = (a.chars(), b.chars());
    for it in iter_a.zip(iter_b) {
        if it.0 == it.1 {
            res.push(it.0);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("Test example".to_string()), "Test example".to_string())
    }
}
