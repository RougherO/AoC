use regex::Regex;
use std::io::stdin;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut result = 0;
    stdin().lines().for_each(|line| {
        let line = line.unwrap();

        let ans = re
            .captures_iter(&line)
            .map(|c| {
                let x = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let y = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
                return x * y;
            })
            .sum::<i32>();

        result += ans;
    });

    println!("{result}");
}
