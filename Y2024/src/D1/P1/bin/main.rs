use std::{io::stdin, iter::zip};

fn main() {
    let mut v1 = vec![];
    let mut v2 = vec![];

    stdin().lines().for_each(|line| {
        let line = line.unwrap();
        let mut iter = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());

        let x1 = iter.next().unwrap();
        let x2 = iter.next().unwrap();

        v1.push(x1);
        v2.push(x2);
    });

    v1.sort();
    v2.sort();

    let result = zip(v1, v2).map(|(x1, x2)| (x1 - x2).abs()).sum::<i32>();

    println!("{}", result);
}
