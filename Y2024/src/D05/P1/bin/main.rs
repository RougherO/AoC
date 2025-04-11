use std::io;

fn main() {
    let mut list = [[false; 100]; 100];

    io::stdin()
        .lines()
        .take_while(|result| result.as_ref().map(|line| !line.is_empty()).unwrap())
        .for_each(|line| {
            let line = line.unwrap();

            let mut it = line.split("|").map(str::parse::<usize>).map(Result::unwrap);

            let x1 = it.next().unwrap();
            let x2 = it.next().unwrap();

            list[x1][x2] = true;
        });

    let result = io::stdin()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();

            let input = line
                .split(',')
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();

            if input.is_sorted_by(|&x, &y| list[x][y]) {
                let size = input.len();
                Some(input[size / 2])
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("{result}");
}
