use std::io::BufRead;

/**
 * D. Mysterious Present
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * Peter decided to wish happy birthday to his friend from Australia and send him a card. To make his present more
 * mysterious, he decided to make a chain. Chain here is such a sequence of envelopes A = {a1,  a2,  ...,  an}, where the
 * width and the height of the i-th envelope is strictly higher than the width and the height of the (i  -  1)-th envelope
 * respectively. Chain size is the number of envelopes in the chain.
 *
 * Peter wants to make the chain of the maximum size from the envelopes he has, the chain should be such, that he'll be able
 * to put a card into it. The card fits into the chain if its width and height is lower than the width and the height of the
 * smallest envelope in the chain respectively. It's forbidden to turn the card and the envelopes.
 *
 * Peter has very many envelopes and very little time, this hard task is entrusted to you.
 *
 * Input
 * The first line contains integers n, w, h (1  ≤ n ≤ 5000, 1 ≤ w,  h  ≤ 106) — amount of envelopes Peter has, the card
 * width and height respectively. Then there follow n lines, each of them contains two integer numbers wi and hi — width and
 * height of the i-th envelope (1 ≤ wi,  hi ≤ 106).
 *
 * Output
 * In the first line print the maximum chain size. In the second line print the numbers of the envelopes (separated by
 * space), forming the required chain, starting with the number of the smallest envelope. Remember, please, that the card
 * should fit into the smallest envelope. If the chain of maximum size is not unique, print any of the answers.
 *
 * If the card does not fit into any of the envelopes, print number 0 in the single line.
 *
 * Examples
 *   Input
 *   2 1 1
 *   2 2
 *   2 2
 *
 *   Output
 *   1
 *   1
 *
 *   Input
 *   3 3 3
 *   5 4
 *   12 11
 *   9 8
 *
 *   Output
 *   3
 *   1 3 2
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line = first_line
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap());

    let (n, card_width, card_height) = (
        first_line.next().unwrap(),
        first_line.next().unwrap(),
        first_line.next().unwrap(),
    );

    let mut envelopes: Vec<(usize, (usize, usize))> = lines
        .take(n)
        .map(Result::unwrap)
        .map(|line| {
            line.split_once(' ')
                .map(|(w, h)| (w.parse().unwrap(), h.parse().unwrap()))
                .unwrap()
        })
        .enumerate()
        .filter(|&(_, (width, height))| width > card_width && height > card_height)
        .map(|(i, dim)| (i + 1, dim))
        .collect();
    envelopes.sort_by_key(|&(_, dim)| dim);

    if envelopes.is_empty() {
        println!("0");
        return;
    }

    let mut containment_table: Vec<usize> = vec![0; envelopes.len()];
    let mut children_table: Vec<Option<usize>> = vec![None; envelopes.len()];

    containment_table[0] = 1;
    for i in 1..envelopes.len() {
        let (width, height) = envelopes[i].1;
        for j in (0..=i - 1).rev() {
            let (child_width, child_height) = envelopes[j].1;

            if width > child_width
                && height > child_height
                && containment_table[j] >= containment_table[i]
            {
                containment_table[i] = 1 + containment_table[j];
                children_table[i] = Some(j);
            }
        }

        if containment_table[i] == 0 {
            containment_table[i] = 1;
        }
    }

    let (i, (answer, mut parent)) = containment_table
        .into_iter()
        .zip(children_table.clone())
        .enumerate()
        .max_by_key(|&(_, (count, _))| count)
        .unwrap();
    println!("{answer}");

    let mut chain = vec![envelopes[i].0.to_string()];
    while let Some(i) = parent {
        parent = children_table[i];
        chain.push(envelopes[i].0.to_string());
    }
    chain.reverse();
    println!("{}", chain.join(" "));
}
