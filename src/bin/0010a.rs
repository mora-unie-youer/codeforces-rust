use std::io::BufRead;

/**
 * A. Power Consumption Calculation
 * time limit per test: 1 second
 * memory limit per test: 256 megabytes
 * input: standard input
 * output: standard output
 *
 * Tom is interested in power consumption of his favourite laptop. His laptop has three modes. In normal mode laptop
 * consumes P1 watt per minute. T1 minutes after Tom moved the mouse or touched the keyboard for the last time, a
 * screensaver starts and power consumption changes to P2 watt per minute. Finally, after T2 minutes from the start of the
 * screensaver, laptop switches to the "sleep" mode and consumes P3 watt per minute. If Tom moves the mouse or touches the
 * keyboard when the laptop is in the second or in the third mode, it switches to the first (normal) mode. Tom's work with
 * the laptop can be divided into n time periods [l1, r1], [l2, r2], ..., [ln, rn]. During each interval Tom continuously
 * moves the mouse and presses buttons on the keyboard. Between the periods Tom stays away from the laptop. Find out the
 * total amount of power consumed by the laptop during the period [l1, rn].
 *
 * Input
 * The first line contains 6 integer numbers n, P1, P2, P3, T1, T2 (1 ≤ n ≤ 100, 0 ≤ P1, P2, P3 ≤ 100, 1 ≤ T1, T2 ≤ 60).
 * The following n lines contain description of Tom's work. Each i-th of these lines contains two space-separated integers
 * li and ri (0 ≤ li < ri ≤ 1440, ri < li + 1 for i < n), which stand for the start and the end of the i-th period of work.
 *
 * Output
 * Output the answer to the problem.
 *
 * Examples
 *   Input
 *   1 3 2 1 5 10
 *   0 10
 *
 *   Output
 *   30
 *
 *   Input
 *   2 8 4 2 5 10
 *   20 30
 *   50 100
 *
 *   Output
 *   570
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line = first_line
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap());
    let n = first_line.next().unwrap() as usize;
    let p1: isize = first_line.next().unwrap();
    let p2 = first_line.next().unwrap();
    let p3 = first_line.next().unwrap();
    let t1 = first_line.next().unwrap();
    let t2 = first_line.next().unwrap();

    let mut ranges: Vec<(isize, isize)> = lines
        .take(n)
        .map(Result::unwrap)
        .map(|line| {
            line.split_once(' ')
                .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
                .unwrap()
        })
        .collect();
    ranges.sort_unstable();

    let mut total_power = 0;
    let mut last_end = ranges[0].0;
    for (start, end) in ranges {
        // Used time
        total_power += (end - start) * p1;

        // Unused time
        let unused_time = start - last_end;
        total_power += t1.min(unused_time) * p1;
        total_power += 0.max(t2.min(unused_time - t1)) * p2;
        total_power += 0.max(unused_time - t1 - t2) * p3;

        last_end = end;
    }

    println!("{total_power}");
}
