use std::{collections::HashSet, io::BufRead};

/**
 * B. President's Office
 * time limit per test: 2 seconds
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * President of Berland has a very vast office-room, where, apart from him, work his subordinates. Each subordinate, as well
 * as President himself, has his own desk of a unique colour. Each desk is rectangular, and its sides are parallel to the
 * office walls. One day President decided to establish an assembly, of which all his deputies will be members.
 * Unfortunately, he does not remember the exact amount of his deputies, but he remembers that the desk of each his deputy
 * is adjacent to his own desk, that is to say, the two desks (President's and each deputy's) have a common side of a
 * positive length.
 *
 * The office-room plan can be viewed as a matrix with n rows and m columns. Each cell of this matrix is either empty, or
 * contains a part of a desk. An uppercase Latin letter stands for each desk colour. The «period» character («.») stands for
 * an empty cell.
 *
 * Input
 * The first line contains two separated by a space integer numbers n, m (1 ≤ n, m ≤ 100) — the length and the width of the
 * office-room, and c character — the President's desk colour. The following n lines contain m characters each — the
 * office-room description. It is guaranteed that the colour of each desk is unique, and each desk represents a continuous
 * subrectangle of the given matrix. All colours are marked by uppercase Latin letters.
 *
 * Output
 * Print the only number — the amount of President's deputies.
 *
 * Examples
 *   Input
 *   3 4 R
 *   G.B.
 *   .RR.
 *   TTT.
 *   
 *   Output
 *   2
 *   
 *   Input
 *   3 3 Z
 *   ...
 *   .H.
 *   ..Z
 *   
 *   Output
 *   0
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line = first_line.split_ascii_whitespace();
    let n: usize = first_line.next().unwrap().parse().unwrap();
    let m: usize = first_line.next().unwrap().parse().unwrap();
    let c = first_line.next().unwrap().as_bytes()[0];

    let mut grid = vec![vec![b'.'; m]; n];
    for (i, line) in lines.take(n).enumerate() {
        let line = line.unwrap();
        for (j, ch) in line.bytes().take(m).enumerate() {
            grid[i][j] = ch;
        }
    }

    let mut deputies = HashSet::new();
    for y in 0..n {
        for x in 0..m {
            if grid[y][x] == c {
                let neighbors = get_neighbors(x, y, n, m);

                for (x, y) in neighbors {
                    let cell = grid[y][x];
                    if cell != b'.' && cell != c {
                        deputies.insert(cell);
                    }
                }
            }
        }
    }

    println!("{}", deputies.len());
}

fn get_neighbors(x: usize, y: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    if x > 0 {
        neighbors.push((x - 1, y));
    }

    if y > 0 {
        neighbors.push((x, y - 1));
    }

    if x < m - 1 {
        neighbors.push((x + 1, y));
    }

    if y < n - 1 {
        neighbors.push((x, y + 1));
    }

    neighbors
}
