use std::{collections::HashMap, io::BufRead};

/**
 * A. Winner
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * The winner of the card game popular in Berland "Berlogging" is determined according to the following rules. If at the end
 * of the game there is only one player with the maximum number of points, he is the winner. The situation becomes more
 * difficult if the number of such players is more than one. During each round a player gains or loses a particular number
 * of points. In the course of the game the number of points is registered in the line "name score", where name is a
 * player's name, and score is the number of points gained in this round, which is an integer number. If score is negative,
 * this means that the player has lost in the round. So, if two or more players have the maximum number of points (say, it
 * equals to m) at the end of the game, than wins the one of them who scored at least m points first. Initially each player
 * has 0 points. It's guaranteed that at the end of the game at least one player has a positive number of points.
 *
 * Input
 * The first line contains an integer number n (1  ≤  n  ≤  1000), n is the number of rounds played. Then follow n lines,
 * containing the information about the rounds in "name score" format in chronological order, where name is a string of
 * lower-case Latin letters with the length from 1 to 32, and score is an integer number between -1000 and 1000, inclusive.
 *
 * Output
 * Print the name of the winner.
 *
 * Examples:
 *   Input
 *   3
 *   mike 3
 *   andrew 5
 *   mike 2
 *   
 *   Output
 *   andrew
 *   
 *   Input
 *   3
 *   andrew 3
 *   andrew 2
 *   mike 5
 *   
 *   Output
 *   andrew
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut scores: HashMap<String, isize> = HashMap::new();
    let mut all_rounds = vec![];
    for line in lines.take(n) {
        let line = line.unwrap();
        let (name, score_diff) = line.split_once(' ').unwrap();
        let name = name.to_owned();
        let score_diff: isize = score_diff.parse().unwrap();

        let score = scores.entry(name.clone()).or_default();
        *score += score_diff;
        all_rounds.push((name.clone(), *score));
    }

    let max_score = scores.values().max().cloned().unwrap();
    let winners: Vec<_> = scores
        .into_iter()
        .filter(|&(_, score)| score == max_score)
        .map(|(name, _)| name)
        .collect();
    let first_winner = all_rounds
        .into_iter()
        .find(|(name, score)| winners.contains(name) && score >= &max_score)
        .map(|(name, _)| name)
        .unwrap();
    println!("{first_winner}");
}
