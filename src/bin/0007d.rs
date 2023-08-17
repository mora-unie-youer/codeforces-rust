use std::io::BufRead;

/**
 * D. Palindrome Degree
 * time limit per test: 1 second
 * memory limit per test: 256 megabytes
 * input: standard input
 * output: standard output
 *
 * String s of length n is called k-palindrome, if it is a palindrome itself, and its prefix and suffix of length are
 * (k - 1)-palindromes. By definition, any string (even empty) is 0-palindrome.
 *
 * Let's call the palindrome degree of string s such a maximum number k, for which s is k-palindrome. For example, "abaaba"
 * has degree equals to 3.
 *
 * You are given a string. Your task is to find the sum of the palindrome degrees of all its prefixes.
 *
 * Input
 * The first line of the input data contains a non-empty string, consisting of Latin letters and digits. The length of the
 * string does not exceed 5·106. The string is case-sensitive.
 *
 * Output
 * Output the only number — the sum of the polindrome degrees of all the string's prefixes.
 *
 * Examples
 *   Input
 *   a2A
 *   
 *   Output
 *   1
 *   
 *   Input
 *   abacaba
 *   
 *   Output
 *   6
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let s = lines.next().unwrap().unwrap();

    let mut power = vec![0; s.len() + 1];
    let (mut pre, mut ba, mut fa) = (0, 0, 1);
    for (i, ch) in s.bytes().enumerate() {
        pre = pre * 17 + ch as usize;
        ba += fa * ch as usize;
        fa *= 17;

        let i = i + 1;
        if pre == ba {
            power[i] = power[(i / 2)] + 1;
        } else {
            power[i] = 0;
        }
    }

    let answer: usize = power.into_iter().sum();
    println!("{}", answer);
}
