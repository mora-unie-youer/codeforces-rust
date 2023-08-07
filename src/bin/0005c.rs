use std::{cmp::Ordering, io::BufRead};

/**
 * C. Longest Regular Bracket Sequence
 * time limit per test: 2 seconds
 * memory limit per test: 256 megabytes
 * input: standard input
 * output: standard output
 *
 * This is yet another problem dealing with regular bracket sequences.
 *
 * We should remind you that a bracket sequence is called regular, if by inserting «+» and «1» into it we can get a correct
 * mathematical expression. For example, sequences «(())()», «()» and «(()(()))» are regular, while «)(», «(()» and «(()))(»
 * are not.
 *
 * You are given a string of «(» and «)» characters. You are to find its longest substring that is a regular bracket
 * sequence. You are to find the number of such substrings as well.
 *
 * Input
 * The first line of the input file contains a non-empty string, consisting of «(» and «)» characters. Its length does not
 * exceed 106.
 *
 * Output
 * Print the length of the longest substring that is a regular bracket sequence, and the number of such substrings. If there
 * are no such substrings, write the only line containing "0 1".
 *
 * Examples
 *   Input
 *   )((())))(()())
 *
 *   Output
 *   6 2
 *
 *   Input
 *   ))(
 *
 *   Output
 *   0 1
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let s = line.as_bytes();

    let mut lengths = vec![0; s.len()];
    let (mut max, mut count) = (0, 1);
    for i in 1..s.len() {
        let start = match i.checked_sub(lengths[i - 1] + 1) {
            Some(v) => v,
            None => continue,
        };

        if s[i] == b')' && s[start] == b'(' {
            lengths[i] = lengths[i - 1] + 2;

            if i > lengths[i - 1] + 2 {
                lengths[i] += lengths[i - lengths[i - 1] - 2];
            }

            match lengths[i].cmp(&max) {
                Ordering::Equal => count += 1,
                Ordering::Greater => {
                    max = lengths[i];
                    count = 1;
                }
                _ => (),
            }
        }
    }

    println!("{max} {count}");
}
