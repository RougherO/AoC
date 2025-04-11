fn main() {
    let tl = [-1, -1];
    let tr = [-1, 1];
    let bl = [1, -1];
    let br = [1, 1];

    let mat = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            line.bytes().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;

    let rows = mat.len();
    let cols = mat[0].len();

    for i in 1..(rows as isize) - 1 {
        for j in 1..(cols as isize) - 1 {
            if mat[i as usize][j as usize] == b'A' {
                let tli = i + tl[0];
                let tlj = j + tl[1];
                let tri = i + tr[0];
                let trj = j + tr[1];
                let bli = i + bl[0];
                let blj = j + bl[1];
                let bri = i + br[0];
                let brj = j + br[1];
                if (mat[tli as usize][tlj as usize] == b'M'
                    && mat[bri as usize][brj as usize] == b'S'
                    && mat[bli as usize][blj as usize] == b'M'
                    && mat[tri as usize][trj as usize] == b'S')
                    || (mat[tli as usize][tlj as usize] == b'S'
                        && mat[bri as usize][brj as usize] == b'M'
                        && mat[bli as usize][blj as usize] == b'S'
                        && mat[tri as usize][trj as usize] == b'M')
                    || (mat[bli as usize][blj as usize] == b'M'
                        && mat[bri as usize][brj as usize] == b'M'
                        && mat[tli as usize][tlj as usize] == b'S'
                        && mat[tri as usize][trj as usize] == b'S')
                    || (mat[bli as usize][blj as usize] == b'S'
                        && mat[bri as usize][brj as usize] == b'S'
                        && mat[tli as usize][tlj as usize] == b'M'
                        && mat[tri as usize][trj as usize] == b'M')
                {
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}");
}
