use std::io::BufRead;

/**
 * C. How Many Squares?
 * time limit per test: 2 seconds
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * You are given a 0-1 rectangular matrix. What is the number of squares in it? A square is a solid square frame (border)
 * with linewidth equal to 1. A square should be at least 2 × 2. We are only interested in two types of squares:
 *   squares with each side parallel to a side of the matrix;
 *   squares with each side parallel to a diagonal of the matrix.
 *
 * For example the following matrix contains only one square of the first type:
 * 0000000
 * 0111100
 * 0100100
 * 0100100
 * 0111100
 *
 *
 * The following matrix contains only one square of the second type:
 * 0000000
 * 0010000
 * 0101000
 * 0010000
 * 0000000
 *
 * Regardless of type, a square must contain at least one 1 and can't touch (by side or corner) any foreign 1. Of course,
 * the lengths of the sides of each square should be equal.
 *
 * How many squares are in the given matrix?
 *
 * Input
 * The first line contains integer t (1 ≤ t ≤ 10000), where t is the number of test cases in the input. Then test cases
 * follow. Each case starts with a line containing integers n and m (2 ≤ n, m ≤ 250), where n is the number of rows and m
 * is the number of columns. The following n lines contain m characters each (0 or 1).
 *
 * The total number of characters in all test cases doesn't exceed 106 for any input file.
 *
 * Output
 * You should output exactly t lines, with the answer to the i-th test case on the i-th line.
 *
 * Examples
 *   Input
 *   2
 *   8 8
 *   00010001
 *   00101000
 *   01000100
 *   10000010
 *   01000100
 *   00101000
 *   11010011
 *   11000011
 *   10 10
 *   1111111000
 *   1000001000
 *   1011001000
 *   1011001010
 *   1000001101
 *   1001001010
 *   1010101000
 *   1001001000
 *   1000001000
 *   1111111000
 *   
 *   Output
 *   1
 *   2
 *   
 *   Input
 *   1
 *   12 11
 *   11111111111
 *   10000000001
 *   10111111101
 *   10100000101
 *   10101100101
 *   10101100101
 *   10100000101
 *   10100000101
 *   10111111101
 *   10000000001
 *   11111111111
 *   00000000000
 *   
 *   Output
 *   3
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let dimensions = lines.next().unwrap().unwrap();
        let (n, m) = dimensions.split_once(' ').unwrap();
        let (n, m): (usize, usize) = (n.parse().unwrap(), m.parse().unwrap());

        let mut grid: Vec<Vec<bool>> = lines
            .by_ref()
            .take(n)
            .map(Result::unwrap)
            .map(|line| line.chars().take(m).map(|ch| ch == '1').collect())
            .collect();

        let mut squares = 0;
        for y in 0..n {
            for x in 0..m {
                if grid[y][x] {
                    let sq1 = square1(&grid, x, y, n, m);
                    let sq2 = square2(&grid, x, y, n, m);
                    let p = perimeter(&mut grid, x, y, n, m);

                    if p == 4 * sq1.saturating_sub(1) || p == 4 * sq2.saturating_sub(1) {
                        squares += 1;
                    }
                }
            }
        }

        println!("{squares}");
    }
}

// Must be called only in left-upper corner of square
fn square1(grid: &[Vec<bool>], mut x: usize, mut y: usize, n: usize, m: usize) -> usize {
    let (mut up, mut right, mut down, mut left) = (1, 1, 1, 1);

    // Up
    while x < m - 1 && grid[y][x + 1] {
        x += 1;
        up += 1;
    }

    // Right
    while y < n - 1 && grid[y + 1][x] {
        y += 1;
        right += 1;
    }

    // Down
    while x > 0 && grid[y][x - 1] {
        x -= 1;
        down += 1;
    }

    // Left
    while y > 0 && grid[y - 1][x] {
        y -= 1;
        left += 1;
    }

    if up == right && right == down && down == left {
        up
    } else {
        0
    }
}

// Must be called in upper vertex
fn square2(grid: &[Vec<bool>], mut x: usize, mut y: usize, n: usize, m: usize) -> usize {
    let (mut up, mut right, mut down, mut left) = (1, 1, 1, 1);

    // Up (up-right)
    while x < m - 1 && y < n - 1 && grid[y + 1][x + 1] {
        x += 1;
        y += 1;
        up += 1;
    }

    // Right (down-right)
    while x > 0 && y < n - 1 && grid[y + 1][x - 1] {
        x -= 1;
        y += 1;
        right += 1;
    }

    // Down (down-left)
    while x > 0 && y > 0 && grid[y - 1][x - 1] {
        x -= 1;
        y -= 1;
        down += 1;
    }

    // Left (up-left)
    while x < m - 1 && y > 0 && grid[y - 1][x + 1] {
        x += 1;
        y -= 1;
        left += 1;
    }

    if up == right && right == down && down == left {
        up
    } else {
        0
    }
}

fn perimeter(grid: &mut [Vec<bool>], x: usize, y: usize, n: usize, m: usize) -> usize {
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    grid[y][x] = false;
    let mut cells = 1;
    for (dx, dy) in DIRECTIONS {
        if dx == -1 && x == 0 || dy == -1 && y == 0 {
            continue;
        }

        let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
        if ny < n && nx < m && grid[ny][nx] {
            cells += perimeter(grid, nx, ny, n, m);
        }
    }

    cells
}
