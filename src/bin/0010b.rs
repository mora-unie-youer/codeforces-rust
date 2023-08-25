use std::io::BufRead;

/**
 * B. Cinema Cashier
 * time limit per test: 1 second
 * memory limit per test: 256 megabytes
 * input: standard input
 * output: standard output
 *
 * All cinema halls in Berland are rectangles with K rows of K seats each, and K is an odd number. Rows and seats are
 * numbered from 1 to K. For safety reasons people, who come to the box office to buy tickets, are not allowed to choose
 * seats themselves. Formerly the choice was made by a cashier, but now this is the responsibility of a special seating
 * program. It was found out that the large majority of Berland's inhabitants go to the cinema in order to watch a movie,
 * that's why they want to sit as close to the hall center as possible. Moreover, a company of M people, who come to watch
 * a movie, want necessarily to occupy M successive seats in one row. Let's formulate the algorithm, according to which the
 * program chooses seats and sells tickets. As the request for M seats comes, the program should determine the row number x
 * and the segment [yl, yr] of the seats numbers in this row, where yr - yl + 1 = M. From all such possible variants as a
 * final result the program should choose the one with the minimum function value of total seats remoteness from the center.
 * Say, — the row and the seat numbers of the most "central" seat. Then the function value of seats remoteness from the hall
 * center is . If the amount of minimum function values is more than one, the program should choose the one that is closer
 * to the screen (i.e. the row number x is lower). If the variants are still multiple, it should choose the one with the
 * minimum yl. If you did not get yet, your task is to simulate the work of this program.
 *
 * Input
 * The first line contains two integers N and K (1 ≤ N ≤ 1000, 1 ≤ K ≤ 99) — the amount of requests and the hall size
 * respectively. The second line contains N space-separated integers Mi from the range [1, K] — requests to the program.
 *
 * Output
 * Output N lines. In the i-th line output «-1» (without quotes), if it is impossible to find Mi successive seats in one
 * row, otherwise output three numbers x, yl, yr. Separate the numbers with a space.
 *
 * Examples
 * Input
 * 2 1
 * 1 1
 *
 * Output
 * 1 1 1
 * -1
 *
 * Input
 * 4 3
 * 1 2 3 1
 *
 * Output
 * 2 2 2
 * 1 1 2
 * 3 1 3
 * 2 1 1
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (n, k) = first_line.split_once(' ').unwrap();
    let (n, k): (usize, usize) = (n.parse().unwrap(), k.parse().unwrap());

    let ms: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .take(n)
        .map(|v| v.parse().unwrap())
        .collect();

    let mut grid = vec![vec![false; k]; k];
    for m in ms {
        let (score, x, y) = (0..k)
            .map(|x| {
                (0..=k - m)
                    .map(|y| (remoteness(x, y, m, &grid), x, y))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap();

        if score == WORST {
            println!("-1");
        } else {
            println!("{} {} {}", x + 1, y + 1, y + m);
            for y in y..y + m {
                grid[x][y] = true;
            }
        }
    }
}

// Just some constant which will never be lower than remoteness function
const WORST: usize = 99 * 100;
fn remoteness(x: usize, y: usize, m: usize, grid: &[Vec<bool>]) -> usize {
    let center = grid.len() / 2;

    // Check if we fit here
    if y + m > grid.len() {
        return WORST;
    }

    let mut value = 0;
    for y in y..y + m {
        if grid[x][y] {
            return WORST;
        }

        value += center.abs_diff(x) + center.abs_diff(y);
    }

    value
}
