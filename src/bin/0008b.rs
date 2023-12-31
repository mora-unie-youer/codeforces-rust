use std::{collections::HashMap, io::BufRead};

/**
 * B. Obsession with Robots
 * time limit per test: 2 seconds
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * The whole world got obsessed with robots,and to keep pace with the progress, great Berland's programmer Draude decided to
 * build his own robot. He was working hard at the robot. He taught it to walk the shortest path from one point to another,
 * to record all its movements, but like in many Draude's programs, there was a bug — the robot didn't always walk the
 * shortest path. Fortunately, the robot recorded its own movements correctly. Now Draude wants to find out when his robot
 * functions wrong. Heh, if Draude only remembered the map of the field, where he tested the robot, he would easily say if
 * the robot walked in the right direction or not. But the field map was lost never to be found, that's why he asks you to
 * find out if there exist at least one map, where the path recorded by the robot is the shortest.
 *
 * The map is an infinite checkered field, where each square is either empty, or contains an obstruction. It is also known
 * that the robot never tries to run into the obstruction. By the recorded robot's movements find out if there exist at
 * least one such map, that it is possible to choose for the robot a starting square (the starting square should be empty)
 * such that when the robot moves from this square its movements coincide with the recorded ones (the robot doesn't run
 * into anything, moving along empty squares only), and the path from the starting square to the end one is the shortest.
 *
 * In one movement the robot can move into the square (providing there are no obstrutions in this square) that has common
 * sides with the square the robot is currently in.
 *
 * Input
 * The first line of the input file contains the recording of the robot's movements. This recording is a non-empty string,
 * consisting of uppercase Latin letters L, R, U and D, standing for movements left, right, up and down respectively.
 * The length of the string does not exceed 100.
 *
 * Output
 * In the first line output the only word OK (if the above described map exists), or BUG (if such a map does not exist).
 *
 * Examples
 * Input
 * LLUUUR
 *
 * Output
 * OK
 *
 * Input
 * RRUULLDD
 *
 * Output
 * BUG
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let path = lines.next().unwrap().unwrap();

    let mut visited = HashMap::new();
    visited.insert((0, 0), 0);

    let (mut x, mut y) = (0, 0);
    let mut length = 0;
    for ch in path.chars() {
        let (dx, dy) = direction(ch);
        (x, y) = (x + dx, y + dy);
        length += 1;

        if visited.insert((x, y), length).is_some() {
            println!("BUG");
            return;
        }

        for neighbor in "LRUD".chars() {
            let (dx, dy) = direction(neighbor);
            let (nx, ny) = (x + dx, y + dy);

            if visited
                .get(&(nx, ny))
                .map(|len| len + 1 < length)
                .unwrap_or(false)
            {
                println!("BUG");
                return;
            }
        }
    }

    println!("OK");
}

fn direction(ch: char) -> (isize, isize) {
    match ch {
        'L' => (-1, 0),
        'R' => (1, 0),
        'U' => (0, -1),
        'D' => (0, 1),

        _ => unreachable!(),
    }
}
