use std::io::BufRead;

/**
 * B. The least round way
 * time limit per test: 2 seconds
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * There is a square matrix n × n, consisting of non-negative integer numbers. You should find such a way on it that
 *   starts in the upper left cell of the matrix;
 *   each following cell is to the right or down from the current cell;
 *   the way ends in the bottom right cell.
 *
 * Moreover, if we multiply together all the numbers along the way, the result should be the least "round". In other words,
 * it should end in the least possible number of zeros.
 *
 * Input
 * The first line contains an integer number n (2 ≤ n ≤ 1000), n is the size of the matrix. Then follow n lines containing
 * the matrix elements (non-negative integer numbers not exceeding 109).
 *
 * Output
 * In the first line print the least number of trailing zeros. In the second line print the correspondent way itself.
 *
 * Examples
 *   Input
 *   3
 *   1 2 3
 *   4 5 6
 *   7 8 9
 *   
 *   Output
 *   0
 *   DDRR
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut matrix = vec![vec![0; n]; n];
    let mut zero_at = None;

    for (y, line) in lines.take(n).enumerate() {
        let line = line.unwrap();
        for (x, v) in line.split_ascii_whitespace().take(n).enumerate() {
            let v = v.parse().unwrap();
            if v == 0 {
                matrix[y][x] = 10;
                zero_at = Some((y, x));
            } else {
                matrix[y][x] = v;
            }
        }
    }

    let mut power_matrix = vec![vec![(0, 0); n]; n];
    let mut direction_matrix = vec![vec![['\0', '\0']; n]; n];

    power_matrix[0][0] = (find_power(matrix[0][0], 2), find_power(matrix[0][0], 5));

    // Filling first row and column
    for i in 1..n {
        // First row
        power_matrix[0][i].0 = find_power(matrix[0][i], 2) + power_matrix[0][i - 1].0;
        power_matrix[0][i].1 = find_power(matrix[0][i], 5) + power_matrix[0][i - 1].1;
        // First column
        power_matrix[i][0].0 = find_power(matrix[i][0], 2) + power_matrix[i - 1][0].0;
        power_matrix[i][0].1 = find_power(matrix[i][0], 5) + power_matrix[i - 1][0].1;
        // Direction on this cells
        direction_matrix[0][i][0] = 'R';
        direction_matrix[0][i][1] = 'R';
        direction_matrix[i][0][0] = 'D';
        direction_matrix[i][0][1] = 'D';
    }

    // Filling all cells with powers and directions
    for row in 1..n {
        for col in 1..n {
            let up_power = power_matrix[row - 1][col];
            let left_power = power_matrix[row][col - 1];

            let current_power = (
                find_power(matrix[row][col], 2),
                find_power(matrix[row][col], 5),
            );

            if up_power.0 < left_power.0 {
                power_matrix[row][col].0 = up_power.0 + current_power.0;
                direction_matrix[row][col][0] = 'D';
            } else {
                power_matrix[row][col].0 = left_power.0 + current_power.0;
                direction_matrix[row][col][0] = 'R';
            }

            if up_power.1 < left_power.1 {
                power_matrix[row][col].1 = up_power.1 + current_power.1;
                direction_matrix[row][col][1] = 'D';
            } else {
                power_matrix[row][col].1 = left_power.1 + current_power.1;
                direction_matrix[row][col][1] = 'R';
            }
        }
    }

    let (min_twos, min_fives) = power_matrix[n - 1][n - 1];
    let min_round = min_twos.min(min_fives);

    // Printing path
    let mut path = String::new();
    if zero_at.is_some() && min_round != 0 {
        // Printing zeroes
        println!("1");

        let Some((_y, x)) = zero_at else { unreachable!() };

        // Going through 0
        for _ in 0..x {
            path.push('R');
        }

        for _ in 0..n - 1 {
            path.push('D');
        }

        for _ in x..n - 1 {
            path.push('R');
        }
    } else {
        // Printing zeroes
        println!("{min_round}");

        // Better way is with least power
        let i = (min_twos >= min_fives) as usize;

        let (mut row, mut col) = (n - 1, n - 1);

        while !(row == 0 && col == 0) {
            let ch = direction_matrix[row][col][i];
            path.push(ch);

            if ch == 'R' {
                col -= 1;
            } else {
                row -= 1;
            }
        }

        // Reversing the whole path, as we started at the end
        path = path.chars().rev().collect();
    }

    println!("{path}");
}

fn find_power(mut v: usize, base: usize) -> usize {
    let mut power = 0;

    while v > 0 && v % base == 0 {
        power += 1;
        v /= base;
    }

    power
}
