use std::io::BufRead;

/**
 * C. Industrial Nim
 * time limit per test: 2 seconds
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * There are n stone quarries in Petrograd.
 *
 * Each quarry owns mi dumpers (1 ≤ i ≤ n). It is known that the first dumper of the i-th quarry has xi stones in it, the
 * second dumper has xi + 1 stones in it, the third has xi + 2, and the mi-th dumper (the last for the i-th quarry) has
 * xi + mi - 1 stones in it.
 *
 * Two oligarchs play a well-known game Nim. Players take turns removing stones from dumpers. On each turn, a player can
 * select any dumper and remove any non-zero amount of stones from it. The player who cannot take a stone loses.
 *
 * Your task is to find out which oligarch will win, provided that both of them play optimally. The oligarchs asked you not
 * to reveal their names. So, let's call the one who takes the first stone «tolik» and the other one «bolik».
 *
 * Input
 * The first line of the input contains one integer number n (1 ≤ n ≤ 10^5) — the amount of quarries. Then there follow n
 * lines, each of them contains two space-separated integers xi and mi (1 ≤ xi, mi ≤ 10^16) — the amount of stones in the
 * first dumper of the i-th quarry and the number of dumpers at the i-th quarry.
 *
 * Output
 * Output «tolik» if the oligarch who takes a stone first wins, and «bolik» otherwise.
 *
 * Examples
 * Input
 * 2
 * 2 1
 * 3 2
 *
 * Output
 * tolik
 *
 * Input
 * 4
 * 1 1
 * 1 1
 * 1 1
 * 1 1
 *
 * Output
 * bolik
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut answer = 0;
    for line in lines.take(n) {
        let line = line.unwrap();
        let (x, m) = line.split_once(' ').unwrap();
        let (x, m): (usize, usize) = (x.parse().unwrap(), m.parse().unwrap());
        answer ^= xor(x - 1) ^ xor(x + m - 1);
    }

    if answer == 0 {
        println!("bolik");
    } else {
        println!("tolik");
    }
}

fn xor(x: usize) -> usize {
    match x % 4 {
        0 => x,
        1 => 1,
        2 => x + 1,
        _ => 0,
    }
}
