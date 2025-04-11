use std::{collections::HashMap, io::stdin};

fn main() {
    let mut v1 = vec![];
    let mut v2 = HashMap::new();

    stdin().lines().for_each(|line| {
        let line = line.unwrap();
        let mut iter = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());

        let x1 = iter.next().unwrap();
        let x2 = iter.next().unwrap();

        v1.push(x1);
        v2.entry(x2).and_modify(|x| *x += 1).or_insert(1);
    });

    let result = v1
        .iter()
        .map(|&x| v2.get(&x).unwrap_or(&0) * x)
        .sum::<i32>();

    println!("{}", result);
}
