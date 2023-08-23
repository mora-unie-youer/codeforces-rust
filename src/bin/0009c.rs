use std::io::BufRead;

/**
 * C. Hexadecimal's Numbers
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * One beautiful July morning a terrible thing happened in Mainframe: a mean virus Megabyte somehow got access to the memory
 * of his not less mean sister Hexadecimal. He loaded there a huge amount of n different natural numbers from 1 to n to
 * obtain total control over her energy.
 *
 * But his plan failed. The reason for this was very simple: Hexadecimal didn't perceive any information, apart from numbers
 * written in binary format. This means that if a number in a decimal representation contained characters apart from 0 and
 * 1, it was not stored in the memory. Now Megabyte wants to know, how many numbers were loaded successfully.
 *
 * Input
 * Input data contains the only number n (1 ≤ n ≤ 109).
 *
 * Output
 * Output the only number — answer to the problem.
 *
 * Examples
 *   Input
 *   10
 *   
 *   Output
 *   2
 *   
 *   Note
 *   For n = 10 the answer includes numbers 1 and 10.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut result = 0;
    process(1, n, &mut result);
    println!("{result}");
}

fn process(k: usize, n: usize, result: &mut usize) {
    if k > n {
        return;
    }

    *result += 1;
    process(k * 10, n, result);
    process(k * 10 + 1, n, result);
}
