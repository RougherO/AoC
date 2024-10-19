use std::fs;

macro_rules! matches {
    ($src:ident, $index:expr, ($expect:literal, $return:expr)) => {
        match $src.get($index..$index + $expect.len()) {
            Some($expect) => $return,
            _ => None,
        }
    };
    ($src:ident, $index:expr, ($expect:literal, $return:expr), $(($expect_rest:literal, $return_rest:expr))*) => {
        match $src.get($index..$index + $expect.len()) {
            Some($expect) => $return,
            _ => matches!($src, $index, $(($expect_rest, $return_rest))*),
        }
    };
}

fn main() {
    println!(
        "{}",
        fs::read_to_string("input.txt")
            .expect("Could not read file")
            .lines()
            .map(|line| {
                let (l, r);

                let f = |(i, c)| match c {
                    '0'..='9' => c.to_digit(10),
                    'o' => matches!(line, i + 1, ("ne", Some(1))),
                    't' => matches!(
                        line,
                        i + 1,
                        ("w", matches!(line, i + 2, ("o", Some(2)))),
                        ("h", matches!(line, i + 2, ("ree", Some(3))))
                    ),
                    'f' => matches!(line, i + 1, ("our", Some(4)), ("ive", Some(5))),
                    's' => matches!(
                        line,
                        i + 1,
                        ("i", matches!(line, i + 2, ("x", Some(6)))),
                        ("e", matches!(line, i + 2, ("ven", Some(7))))
                    ),
                    'e' => matches!(line, i + 1, ("ight", Some(8))),
                    'n' => matches!(line, i + 1, ("ine", Some(9))),
                    _ => None,
                };

                l = line.chars().enumerate().find_map(f).unwrap();

                r = line
                    .chars()
                    .rev()
                    .enumerate()
                    .find_map(|(i, c)| f((line.len() - i, c)))
                    .unwrap();

                l * 10 + r
            })
            .sum::<u32>()
    );
}
