use regex::Regex;
use std::io::stdin;

fn main() {
    let re = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();

    let mut valid = true;
    let mut result = 0;

    stdin().lines().for_each(|line| {
        let line = line.unwrap();

        for c in re.captures_iter(&line) {
            match &c[0] {
                "don't()" => valid = false,
                "do()" => valid = true,
                _ => {
                    if valid {
                        let x = c[1].parse::<i32>().unwrap();
                        let y = c[2].parse::<i32>().unwrap();
                        result += x * y;
                    }
                }
            }
        }
    });

    println!("{result}");
}
