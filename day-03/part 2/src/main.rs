use std::env::args;
#[macro_use]
extern crate scan_fmt;

fn main() {
    println!("{}", run(&args().nth(1).expect("Please provide an input")))
}

struct Claim {
    id: u32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn run(input: &str) -> u32 {
    let mut claims: Vec<Claim> = Vec::new();
    let mut res = 0;
    for line in input.lines() {
        claims.push(parse(line));
    }

    for c1 in &claims {
        let mut no_overlap = true;
        for c2 in &claims {
            if c1.id != c2.id && overlap(c1, c2) {
                no_overlap = false;
                break;
            }
        }
        if no_overlap {
            res = c1.id;
        }
    }
    res
}

fn parse(line: &str) -> Claim {
    let safe = scan_fmt!(line, "#{} @ {},{}: {}x{}", u32, usize, usize, usize, usize);
    let (id, x, y, w, h) = (
        safe.0.unwrap(),
        safe.1.unwrap(),
        safe.2.unwrap(),
        safe.3.unwrap(),
        safe.4.unwrap(),
    );
    Claim { id, x, y, w, h }
}

fn overlap(c1: &Claim, c2: &Claim) -> bool {
    let b1 = c1.x >= c2.x && c1.x < c2.x + c2.w;
    let b2 = c2.x >= c1.x && c2.x < c1.x + c1.w;
    let b3 = c1.y >= c2.y && c1.y < c2.y + c2.h;
    let b4 = c2.y >= c1.y && c2.y < c1.y + c1.h;
    (b1 || b2) && (b3 || b4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 3);
    }
}
