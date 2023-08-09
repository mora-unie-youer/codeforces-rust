use std::{collections::VecDeque, io::BufRead};

/**
 * E. Bindian Signalizing
 * time limit per test: 4 seconds
 * memory limit per test: 256 megabytes
 * input: standard input
 * output: standard output
 *
 * Everyone knows that long ago on the territory of present-day Berland there lived Bindian tribes. Their capital was
 * surrounded by n hills, forming a circle. On each hill there was a watchman, who watched the neighbourhood day and night.
 *
 * In case of any danger the watchman could make a fire on the hill. One watchman could see the signal of another watchman,
 * if on the circle arc connecting the two hills there was no hill higher than any of the two. As for any two hills there
 * are two different circle arcs connecting them, the signal was seen if the above mentioned condition was satisfied on at
 * least one of the arcs. For example, for any two neighbouring watchmen it is true that the signal of one will be seen by
 * the other.
 *
 * An important characteristics of this watch system was the amount of pairs of watchmen able to see each other's signals.
 * You are to find this amount by the given heights of the hills.
 *
 * Input
 * The first line of the input data contains an integer number n (3 ≤ n ≤ 10^6), n — the amount of hills around the capital.
 * The second line contains n numbers — heights of the hills in clockwise order. All height numbers are integer and lie
 * between 1 and 10^9.
 *
 * Output
 * Print the required amount of pairs.
 *
 * Examples
 *   Input
 *   5
 *   1 2 4 5 3
 *   
 *   Output
 *   7
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let hills = lines.next().unwrap().unwrap();
    let mut hills: VecDeque<usize> = hills
        .split_ascii_whitespace()
        .take(n)
        .map(|v| v.parse().unwrap())
        .collect();

    let (mut max_hill, mut max_pos) = (0, 0);
    for (i, &hill) in hills.iter().enumerate() {
        if hill > max_hill {
            max_hill = hill;
            max_pos = i;
        }
    }

    // Rotating hills so that max element is first
    hills.rotate_left(max_pos);

    // Looking at left side of hills
    let mut left = vec![std::usize::MAX; hills.len()];
    for i in 1..hills.len() {
        left[i] = i - 1;
        while left[i] > 0 && hills[i] >= hills[left[i]] {
            left[i] = left[left[i]];
        }
    }

    // Looking at right side of hills
    let mut count = vec![0; hills.len()];
    let mut right = vec![std::usize::MAX; hills.len()];
    for i in (0..hills.len()).rev() {
        right[i] = i + 1;
        while right[i] < hills.len() && hills[i] > hills[right[i]] {
            right[i] = right[right[i]];
        }

        if right[i] < hills.len() && hills[i] == hills[right[i]] {
            count[i] = count[right[i]] + 1;
            right[i] = right[right[i]];
        }
    }

    let mut answer = 0;
    for i in 1..hills.len() {
        answer += count[i] + 1;
        if left[i] != 0 || right[i] != hills.len() {
            answer += 1;
        }
    }

    println!("{answer}");
}
