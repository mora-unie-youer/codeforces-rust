use std::io::BufRead;

/**
 * A. Numbers
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * Little Petya likes numbers a lot. He found that number 123 in base 16 consists of two digits: the first is 7 and the
 * second is 11. So the sum of digits of 123 in base 16 is equal to 18.
 *
 * Now he wonders what is an average value of sum of digits of the number A written in all bases from 2 to A - 1.
 *
 * Note that all computations should be done in base 10. You should find the result as an irreducible fraction, written
 * in base 10.
 *
 * Input
 * Input contains one integer number A (3 ≤ A ≤ 1000).
 *
 * Output
 * Output should contain required average value in format «X/Y», where X is the numerator and Y is the denominator.
 *
 * Examples
 *   Input
 *   5
 *   
 *   Output
 *   7/3
 *   
 *   Input
 *   3
 *   
 *   Output
 *   2/1
 *   
 *   Note
 *   In the first sample number 5 written in all bases from 2 to 4 looks so: 101, 12, 11. Sums of digits are 2, 3 and 2,
 * respectively.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let count = n - 2;

    let mut sum: usize = 0;
    for base in 2..n {
        let mut number = n;
        while number > 0 {
            sum += number % base;
            number /= base;
        }
    }

    let a = sum;
    let b = count;
    let c = gcd(a, b);
    println!("{}/{}", a / c, b / c);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
