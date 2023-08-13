use std::io::BufRead;

/**
 * D. Lizards and Basements 2
 * time limit per test: 2 seconds
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * This is simplified version of the problem used on the original contest. The original problem seems to have too difiicult
 * solution. The constraints for input data have been reduced.
 *
 * Polycarp likes to play computer role-playing game «Lizards and Basements». At the moment he is playing it as a magician.
 * At one of the last levels he has to fight the line of archers. The only spell with which he can damage them is a fire
 * ball. If Polycarp hits the i-th archer with his fire ball (they are numbered from left to right), the archer loses a
 * health points. At the same time the spell damages the archers adjacent to the i-th (if any) — they lose
 * b (1 ≤ b < a ≤ 10) health points each.
 *
 * As the extreme archers (i.e. archers numbered 1 and n) are very far, the fire ball cannot reach them. Polycarp can hit
 * any other archer with his fire ball.
 *
 * The amount of health points for each archer is known. An archer will be killed when this amount is less than 0. What is
 * the minimum amount of spells Polycarp can use to kill all the enemies?
 *
 * Polycarp can throw his fire ball into an archer if the latter is already killed.
 *
 * Input
 * The first line of the input contains three integers n, a, b (3 ≤ n ≤ 10; 1 ≤ b < a ≤ 10). The second line contains a
 * sequence of n integers — h1, h2, ..., hn (1 ≤ hi ≤ 15), where hi is the amount of health points the i-th archer has.
 *
 * Output
 * In the first line print t — the required minimum amount of fire balls.
 *
 * In the second line print t numbers — indexes of the archers that Polycarp should hit to kill all the archers in t shots.
 * All these numbers should be between 2 and n - 1. Separate numbers with spaces. If there are several solutions, output
 * any of them. Print numbers in any order.
 *
 * Examples
 *   Input
 *   3 2 1
 *   2 2 2
 *
 *   Output
 *   3
 *   2 2 2
 *
 *   Input
 *   4 3 1
 *   1 4 1 1
 *
 *   Output
 *   4
 *   2 2 3 3
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line = first_line
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap());
    let n: usize = first_line.next().unwrap();
    let a = first_line.next().unwrap() as isize;
    let b = first_line.next().unwrap() as isize;

    let mut healths: Vec<isize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .take(n)
        .map(|v| v.parse().unwrap())
        .collect();

    let mut min_fireballs = std::usize::MAX;
    let mut min_attacks = vec![];

    simulate(
        &mut healths,
        (a, b),
        1,
        &mut vec![],
        &mut min_fireballs,
        &mut min_attacks,
    );

    println!("{min_fireballs}");

    let attacks: Vec<_> = min_attacks
        .into_iter()
        .map(|i| i + 1)
        .map(|v| v.to_string())
        .collect();
    println!("{}", attacks.join(" "));
}

fn simulate(
    healths: &mut [isize],
    damage @ (a, b): (isize, isize),
    position: usize,

    attacks: &mut Vec<usize>,

    min_fireballs: &mut usize,
    min_attacks: &mut Vec<usize>,
) {
    // If we have more fireballs than minimum -> stopping this branch
    if attacks.len() > *min_fireballs {
        return;
    }

    // We damaged all the archers. Now we can try to write new answer
    if position == healths.len() - 1 {
        if attacks.len() < *min_fireballs {
            *min_fireballs = attacks.len();
            *min_attacks = attacks.clone();
        }

        return;
    }

    // Calculating amount of attacks at current position
    // Left and center could be dead at previous steps, right cannot
    let to_kill_left = 0.max((healths[position - 1] + b) / b);
    let to_kill_center = 0.max((healths[position] + a) / a);
    let to_kill_right = (healths[position + 1] + b) / b;

    // If we at last position -> we just need to kill last 3 archers
    if position == healths.len() - 2 {
        // Fetching required amount of attacks
        let count = to_kill_left.max(to_kill_center.max(to_kill_right));

        // Saving old fireballs count
        let fireballs = attacks.len();

        // Adding attacks
        attacks.extend(vec![position; count as usize]);

        // Going to trigger dfs gate
        simulate(
            healths,
            damage,
            position + 1,
            attacks,
            min_fireballs,
            min_attacks,
        );

        // Truncating attacks
        attacks.truncate(fireballs);
        return;
    }

    // So now we need to attack. There we need to make some amount of attacks to kill first archer.
    let minimum_count = to_kill_left;
    let maximum_count = to_kill_left.max(to_kill_center.max(to_kill_right));
    for count in minimum_count..=maximum_count {
        // Modifying healths for next iterations
        healths[position - 1] -= b * count;
        healths[position] -= a * count;
        healths[position + 1] -= b * count;

        // Saving old fireballs count
        let fireballs = attacks.len();

        // Adding attacks
        attacks.extend(vec![position; count as usize]);

        // Running simulation further
        simulate(
            healths,
            damage,
            position + 1,
            attacks,
            min_fireballs,
            min_attacks,
        );

        // Truncating attacks
        attacks.truncate(fireballs);

        // Restoring healths
        healths[position - 1] += b * count;
        healths[position] += a * count;
        healths[position + 1] += b * count;
    }
}
