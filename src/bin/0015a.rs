use std::io::BufRead;

/**
 * A. Cottage Village
 * time limit per test: 2 seconds
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * A new cottage village called «Flatville» is being built in Flatland. By now they have already built in «Flatville» n
 * square houses with the centres on the Оx-axis. The houses' sides are parallel to the coordinate axes. It's known that no
 * two houses overlap, but they can touch each other.
 *
 * The architect bureau, where Peter works, was commissioned to build a new house in «Flatville». The customer wants his
 * future house to be on the Оx-axis, to be square in shape, have a side t, and touch at least one of the already built
 * houses. For sure, its sides should be parallel to the coordinate axes, its centre should be on the Ox-axis and it
 * shouldn't overlap any of the houses in the village.
 *
 * Peter was given a list of all the houses in «Flatville». Would you help him find the amount of possible positions of the
 * new house?
 *
 * Input
 * The first line of the input data contains numbers n and t (1 ≤ n, t ≤ 1000). Then there follow n lines, each of them
 * contains two space-separated integer numbers: xi ai, where xi — x-coordinate of the centre of the i-th house, and ai —
 * length of its side ( - 1000 ≤ xi ≤ 1000, 1 ≤ ai ≤ 1000).
 *
 * Output
 * Output the amount of possible positions of the new house.
 *
 * Examples
 *   Input
 *   2 2
 *   0 4
 *   6 2
 *   
 *   Output
 *   4
 *   
 *   Input
 *   2 2
 *   0 4
 *   5 2
 *   
 *   Output
 *   3
 *   
 *   Input
 *   2 3
 *   0 4
 *   5 2
 *   
 *   Output
 *   2
 *   
 *   Note
 *   It is possible for the x-coordinate of the new house to have non-integer value.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (n, t) = first_line.split_once(' ').unwrap();
    let (n, t): (usize, f64) = (n.parse().unwrap(), t.parse().unwrap());

    let mut ranges: Vec<(f64, f64)> = lines
        .take(n)
        .map(Result::unwrap)
        .map(|line| {
            line.split_once(' ')
                .map(|(x, a)| (x.parse().unwrap(), a.parse().unwrap()))
                .unwrap()
        })
        .map(|(x, a): (f64, f64)| (x - a / 2., x + a / 2.))
        .collect();
    ranges.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut positions = 2;
    for chunk in ranges.windows(2) {
        let (a, b) = (chunk[0], chunk[1]);
        let space = b.0 - a.1;

        positions += (space >= t) as usize;
        positions += (space > t) as usize;
    }

    println!("{positions}");
}
