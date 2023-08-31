use std::io::BufRead;

/**
 * B. Correct Solution?
 * time limit per test: 2 seconds
 * memory limit per test: 256 megabytes
 * input: standard input
 * output: standard output
 *
 * One cold winter evening Alice and her older brother Bob was sitting at home near the fireplace and giving each other
 * interesting problems to solve. When it was Alice's turn, she told the number n to Bob and said:
 *
 * —Shuffle the digits in this number in order to obtain the smallest possible number without leading zeroes.
 *
 * —No problem! — said Bob and immediately gave her an answer.
 *
 * Alice said a random number, so she doesn't know whether Bob's answer is correct. Help her to find this out, because
 * impatient brother is waiting for the verdict.
 *
 * Input
 * The first line contains one integer n (0 ≤ n ≤ 109) without leading zeroes. The second lines contains one integer
 * m (0 ≤ m ≤ 109) — Bob's answer, possibly with leading zeroes.
 *
 * Output
 * Print OK if Bob's answer is correct and WRONG_ANSWER otherwise.
 *
 * Examples
 *   Input
 *   3310
 *   1033
 *   
 *   Output
 *   OK
 *   
 *   Input
 *   4
 *   5
 *   
 *   Output
 *   WRONG_ANSWER
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut n: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let m: String = lines.next().unwrap().unwrap();

    n.sort_unstable();
    let first_digit = n.iter().position(|&ch| ch != '0').unwrap_or(0);
    let mut smallest = String::new();
    smallest.push(n.remove(first_digit));
    smallest.extend(n);

    if smallest == m {
        println!("OK");
    } else {
        println!("WRONG_ANSWER");
    }
}
