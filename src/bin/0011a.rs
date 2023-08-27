use std::io::BufRead;

/**
 * A. Increasing Sequence
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * A sequence a0, a1, ..., at - 1 is called increasing if ai - 1 < ai for each i: 0 < i < t.
 *
 * You are given a sequence b0, b1, ..., bn - 1 and a positive integer d. In each move you may choose one element of the
 * given sequence and add d to it. What is the least number of moves required to make the given sequence increasing?
 *
 * Input
 * The first line of the input contains two integer numbers n and d (2 ≤ n ≤ 2000, 1 ≤ d ≤ 106). The second line contains
 * space separated sequence b0, b1, ..., bn - 1 (1 ≤ bi ≤ 106).
 *
 * Output
 * Output the minimal number of moves needed to make the sequence increasing.
 *
 * Examples
 *   Input
 *   4 2
 *   1 3 3 2
 *   
 *   Output
 *   3
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (n, d) = first_line.split_once(' ').unwrap();
    let (n, d): (usize, usize) = (n.parse().unwrap(), d.parse().unwrap());

    let seq: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .take(n)
        .map(|v| v.parse().unwrap())
        .collect();

    let mut steps = 0;
    let mut prev = seq[0];
    for mut current in seq.into_iter().skip(1) {
        if current <= prev {
            let diff = prev - current;
            let steps_to_fix = (diff + d - 1) / d;

            steps += steps_to_fix;
            current += steps_to_fix * d;
            if current == prev {
                steps += 1;
                current += d;
            }
        }

        prev = current;
    }

    println!("{steps}");
}
