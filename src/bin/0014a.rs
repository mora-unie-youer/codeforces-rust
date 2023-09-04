use std::{collections::HashSet, io::BufRead};

/**
 * A. Letter
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * A boy Bob likes to draw. Not long ago he bought a rectangular graph (checked) sheet with n rows and m columns. Bob shaded
 * some of the squares on the sheet. Having seen his masterpiece, he decided to share it with his elder brother, who lives
 * in Flatland. Now Bob has to send his picture by post, but because of the world economic crisis and high oil prices, he
 * wants to send his creation, but to spend as little money as possible. For each sent square of paper (no matter whether it
 * is shaded or not) Bob has to pay 3.14 burles. Please, help Bob cut out of his masterpiece a rectangle of the minimum
 * cost, that will contain all the shaded squares. The rectangle's sides should be parallel to the sheet's sides.
 *
 * Input
 * The first line of the input data contains numbers n and m (1 ≤ n, m ≤ 50), n — amount of lines, and m — amount of columns
 * on Bob's sheet. The following n lines contain m characters each. Character «.» stands for a non-shaded square on the
 * sheet, and «*» — for a shaded square. It is guaranteed that Bob has shaded at least one square.
 *
 * Output
 * Output the required rectangle of the minimum cost. Study the output data in the sample tests to understand the output
 * format better.
 *
 * Examples
 *   Input
 *   6 7
 *   .......
 *   ..***..
 *   ..*....
 *   ..***..
 *   ..*....
 *   ..***..
 *   
 *   Output
 *   ***
 *   *..
 *   ***
 *   *..
 *   ***
 *   
 *   Input
 *   3 3
 *   ***
 *   *.*
 *   ***
 *   
 *   Output
 *   ***
 *   *.*
 *   ***
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (n, m) = first_line.split_once(' ').unwrap();
    let (n, m): (usize, usize) = (n.parse().unwrap(), m.parse().unwrap());

    let grid: HashSet<(usize, usize)> = lines
        .take(n)
        .map(Result::unwrap)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .take(m)
                .enumerate()
                .filter(|&(_, ch)| ch == '*')
                .map(|(x, _)| (x, y))
                .collect::<Vec<_>>()
        })
        .collect();

    let min_x = *grid.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *grid.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *grid.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *grid.iter().map(|(_, y)| y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if grid.contains(&(x, y)) {
                print!("*");
            } else {
                print!(".");
            }
        }

        println!();
    }
}
