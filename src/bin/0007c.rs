use std::io::BufRead;

/**
 * C. Line
 * time limit per test: 1 second
 * memory limit per test: 256 megabytes
 * input: standard input
 * output: standard output
 *
 * A line on the plane is described by an equation Ax + By + C = 0. You are to find any point on this line, whose coordinates are integer numbers from  - 5·1018 to 5·1018 inclusive, or to find out that such points do not exist.
 *
 * Input
 * The first line contains three integers A, B and C ( - 2·109 ≤ A, B, C ≤ 2·109) — corresponding coefficients of the line equation. It is guaranteed that A2 + B2 > 0.
 *
 * Output
 * If the required point exists, output its coordinates, otherwise output -1.
 *
 * Examples
 *   Input
 *   2 5 3
 *   
 *   Output
 *   6 -3
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let mut parts = line.split_ascii_whitespace().map(|v| v.parse().unwrap());
    let a: isize = parts.next().unwrap();
    let b = parts.next().unwrap();
    let c = parts.next().unwrap();

    let (x, y, d) = extended_gcd(a, b);
    if c % d == 0 {
        let f = -c / d;
        let (x, y) = (x * f, y * f);
        println!("{x} {y}")
    } else {
        println!("-1");
    }
}

fn extended_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    if b == 0 {
        return (1, 0, a);
    }

    let (x, y, d) = extended_gcd(b, a % b);
    (y, x - a / b * y, d)
}
