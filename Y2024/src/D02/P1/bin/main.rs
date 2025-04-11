use std::io::stdin;

fn main() {
    let mut result = 0;

    stdin().lines().for_each(|line| {
        let line = line.unwrap();
        let list = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let dec = list[0] > list[1];

        for i in 1..list.len() {
            let diff = list[i] - list[i - 1];

            if !((diff < 0 && dec || diff > 0 && !dec) && diff.abs() >= 1 && diff.abs() <= 3) {
                return;
            }
        }

        result += 1;
    });

    println!("{}", result);
}
