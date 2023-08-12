use std::{cmp::Ordering, collections::VecDeque, io::BufRead};

/**
 * C. Alice, Bob and Chocolate
 * time limit per test: 2 seconds
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * Alice and Bob like games. And now they are ready to start a new game. They have placed n chocolate bars in a line. Alice
 * starts to eat chocolate bars one by one from left to right, and Bob — from right to left. For each chocololate bar the
 * time, needed for the player to consume it, is known (Alice and Bob eat them with equal speed). When the player consumes a
 * chocolate bar, he immediately starts with another. It is not allowed to eat two chocolate bars at the same time, to leave
 * the bar unfinished and to make pauses. If both players start to eat the same bar simultaneously, Bob leaves it to Alice
 * as a true gentleman.
 *
 * How many bars each of the players will consume?
 *
 * Input
 * The first line contains one integer n (1 ≤ n ≤ 105) — the amount of bars on the table. The second line contains a
 * sequence t1, t2, ..., tn (1 ≤ ti ≤ 1000), where ti is the time (in seconds) needed to consume the i-th bar (in the order
 * from left to right).
 *
 * Output
 * Print two numbers a and b, where a is the amount of bars consumed by Alice, and b is the amount of bars consumed by Bob.
 *
 * Examples
 *   Input
 *   5
 *   2 9 8 2 7
 *   
 *   Output
 *   2 3
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut bars: VecDeque<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .take(n)
        .map(|v| v.parse().unwrap())
        .collect();

    let (mut alice_sum, mut alice_count) = (0, 0);
    let (mut bob_sum, mut bob_count) = (0, 0);
    while !bars.is_empty() {
        match alice_sum.cmp(&bob_sum) {
            Ordering::Greater => {
                bob_sum += bars.pop_back().unwrap();
                bob_count += 1;
            }
            _ => {
                alice_sum += bars.pop_front().unwrap();
                alice_count += 1;
            }
        }
    }

    println!("{alice_count} {bob_count}")
}
