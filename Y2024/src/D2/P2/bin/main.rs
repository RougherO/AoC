use std::io::stdin;

fn is_valid(list: Vec<i32>) -> bool {
    let dec = list[0] > list[1];

    for i in 1..list.len() {
        let diff = list[i] - list[i - 1];

        if !((diff < 0 && dec || diff > 0 && !dec) && diff.abs() >= 1 && diff.abs() <= 3) {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut result = 0;

    stdin().lines().for_each(|line| {
        let line = line.unwrap();
        let list = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let valid = (0..list.len())
            .map(|index| {
                let mut l = list.clone();
                l.remove(index);
                is_valid(l)
            })
            .reduce(|x, y| {
                return x || y;
            })
            .unwrap();

        if valid {
            result += 1;
        }
    });

    println!("{result}");
}
