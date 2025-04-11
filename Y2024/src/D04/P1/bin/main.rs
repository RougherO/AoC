fn find(mat: &Vec<Vec<u8>>, i: i32, j: i32, dy: i32, dx: i32, text: &[u8], index: usize) -> bool {
    if index == text.len() {
        return true;
    }

    if i < 0 || j < 0 || i >= mat.len() as i32 || j >= mat[0].len() as i32 {
        return false;
    }

    if mat[i as usize][j as usize] as u8 == text[index] {
        return find(mat, i + dy, j + dx, dy, dx, text, index + 1);
    }

    return false;
}

fn main() {
    let text = b"XMAS";

    let mat = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            line.bytes().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;

    let rows = mat.len() as i32;
    let cols = mat[0].len() as i32;

    for i in 0..rows {
        for j in 0..cols {
            if mat[i as usize][j as usize] == b'X' {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if i == j && i == 0 {
                            continue;
                        }
                        if find(&mat, i + dy, j + dx, dy, dx, text, 1) {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{ans}");
}
