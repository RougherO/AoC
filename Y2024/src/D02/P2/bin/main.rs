use std::io::stdin;

fn f(list: &[i32]) -> (Vec<bool>, Vec<bool>, Vec<bool>, Vec<bool>) {
    let size = list.len();

    let (mut prefixinc, mut prefixdec, mut suffixinc, mut suffixdec) = (
        vec![false; size],
        vec![false; size],
        vec![false; size],
        vec![false; size],
    );

    (
        prefixinc[0],
        prefixdec[0],
        suffixinc[size - 1],
        suffixdec[size - 1],
    ) = (true, true, true, true);

    for i in 1..size {
        let diff = list[i] - list[i - 1];

        if diff < 1 || diff > 3 {
            break;
        }
        prefixinc[i] = true;
    }

    for i in 1..size {
        let diff = list[i - 1] - list[i];

        if diff < 1 || diff > 3 {
            break;
        }
        prefixdec[i] = true;
    }

    for i in (0..size - 1).rev() {
        let diff = list[i + 1] - list[i];

        if diff < 1 || diff > 3 {
            break;
        }
        suffixinc[i] = true;
    }

    for i in (0..size - 1).rev() {
        let diff = list[i] - list[i + 1];

        if diff < 1 || diff > 3 {
            break;
        }
        suffixdec[i] = true;
    }

    return (prefixinc, prefixdec, suffixinc, suffixdec);
}

fn main() {
    let mut result = 0;

    stdin().lines().for_each(|line| {
        let line = line.unwrap();
        let list = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let (prefixinc, prefixdec, suffixinc, suffixdec) = f(&list);

        let size = list.len();

        if suffixinc[1] || suffixdec[1] || prefixinc[size - 2] || prefixdec[size - 2] {
            result += 1;
            return;
        }

        for i in 1..size - 1 {
            if prefixinc[i - 1] && suffixinc[i + 1] {
                let diff = list[i + 1] - list[i - 1];

                if diff >= 1 && diff <= 3 {
                    result += 1;
                    return;
                }
            } else if prefixdec[i - 1] && suffixdec[i + 1] {
                let diff = list[i - 1] - list[i + 1];

                if diff >= 1 && diff <= 3 {
                    result += 1;
                    return;
                }
            }
        }
    });

    println!("{result}");
}
