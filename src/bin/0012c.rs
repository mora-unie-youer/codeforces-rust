use std::{cmp::Reverse, collections::HashMap, io::BufRead};

/**
 * C. Fruits
 * time limit per test: 1 second
 * memory limit per test: 256 megabytes
 * input: standard input
 * output: standard output
 *
 * The spring is coming and it means that a lot of fruits appear on the counters. One sunny day little boy Valera decided to
 * go shopping. He made a list of m fruits he wanted to buy. If Valera want to buy more than one fruit of some kind, he
 * includes it into the list several times.
 *
 * When he came to the fruit stall of Ashot, he saw that the seller hadn't distributed price tags to the goods, but put all
 * price tags on the counter. Later Ashot will attach every price tag to some kind of fruits, and Valera will be able to
 * count the total price of all fruits from his list. But Valera wants to know now what can be the smallest total price (in
 * case of the most «lucky» for him distribution of price tags) and the largest total price (in case of the most «unlucky»
 * for him distribution of price tags).
 *
 * Input
 * The first line of the input contains two integer number n and m (1 ≤ n, m ≤ 100) — the number of price tags (which is
 * equal to the number of different kinds of fruits that Ashot sells) and the number of items in Valera's list. The second
 * line contains n space-separated positive integer numbers. Each of them doesn't exceed 100 and stands for the price of one
 * fruit of some kind. The following m lines contain names of the fruits from the list. Each name is a non-empty string of
 * small Latin letters which length doesn't exceed 32. It is guaranteed that the number of distinct fruits from the list is
 * less of equal to n. Also it is known that the seller has in stock all fruits that Valera wants to buy.
 *
 * Output
 * Print two numbers a and b (a ≤ b) — the minimum and the maximum possible sum which Valera may need to buy all fruits from his list.
 *
 * Examples
 *   Input
 *   5 3
 *   4 2 1 10 5
 *   apple
 *   orange
 *   mango
 *
 *   Output
 *   7 19
 *
 *   Input
 *   6 5
 *   3 5 1 6 8 1
 *   peach
 *   grapefruit
 *   banana
 *   orange
 *   orange
 *
 *   Output
 *   11 30
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (n, m) = first_line.split_once(' ').unwrap();
    let (n, m): (usize, usize) = (n.parse().unwrap(), m.parse().unwrap());

    let mut prices: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .take(n)
        .map(|v| v.parse().unwrap())
        .collect();
    prices.sort_unstable();

    let mut fruits: HashMap<String, usize> = HashMap::new();
    for line in lines.take(m) {
        let line = line.unwrap();
        *fruits.entry(line).or_default() += 1;
    }

    let mut fruits: Vec<usize> = fruits.into_values().collect();
    fruits.sort_unstable_by_key(|&v| Reverse(v));

    let min: usize = fruits.iter().zip(&prices).map(|(a, p)| a * p).sum();
    let max: usize = fruits
        .iter()
        .zip(prices.iter().rev())
        .map(|(a, p)| a * p)
        .sum();

    println!("{min} {max}");
}
