use std::io::BufRead;

/**
* A. Theatre Square
* time limit per test: 1 second
* memory limit per test: 256 megabytes
* input: standard input
* output: standard output
*
* Theatre Square in the capital city of Berland has a rectangular shape with the size n × m meters. On the occasion of the
* city's anniversary, a decision was taken to pave the Square with square granite flagstones. Each flagstone is of the
* size a × a.
*
* What is the least number of flagstones needed to pave the Square? It's allowed to cover the surface larger than the
* Theatre Square, but the Square has to be covered. It's not allowed to break the flagstones. The sides of flagstones
* should be parallel to the sides of the Square.
*
*
* Input
* The input contains three positive integer numbers in the first line: n,  m and a (1 ≤  n, m, a ≤ 109).
*
* Output
* Write the needed number of flagstones.
*
*
* Example 1:
*   Input
*   6 6 4
*
*   Output
*   4
*/
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let input = lines.next().unwrap().unwrap();
    let mut input = input.split_ascii_whitespace();
    let (n, m, a): (usize, usize, usize) = (
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
    );

    let x = (n as f64 / a as f64).ceil() as usize;
    let y = (m as f64 / a as f64).ceil() as usize;
    println!("{}", x * y);
}
