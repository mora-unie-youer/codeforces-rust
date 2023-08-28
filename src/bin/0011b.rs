use std::io::BufRead;

/**
 * B. Jumping Jack
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * Jack is working on his jumping skills recently. Currently he's located at point zero of the number line. He would like to
 * get to the point x. In order to train, he has decided that he'll first jump by only one unit, and each subsequent jump
 * will be exactly one longer than the previous one. He can go either left or right with each jump. He wonders how many
 * jumps he needs to reach x.
 *
 * Input
 * The input data consists of only one integer x ( - 109 ≤ x ≤ 109).
 *
 * Output
 * Output the minimal number of jumps that Jack requires to reach x.
 *
 * Examples
 *   Input
 *   2
 *
 *   Output
 *   3
 *
 *   Input
 *   6
 *
 *   Output
 *   3
 *
 *   Input
 *   0
 *
 *   Output
 *   0
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let x: isize = lines.next().unwrap().unwrap().parse().unwrap();
    let x = x.unsigned_abs();

    if x == 0 {
        println!("0");
        return;
    }

    let jumps = (1..).scan(0, |acc, i| {
        *acc += i;
        Some((i, *acc))
    });

    let mut answer = 0;
    for (i, pos) in jumps {
        if pos == x || pos > x && (pos - x) % 2 == 0 {
            answer = i;
            break;
        }
    }

    println!("{answer}");
}
