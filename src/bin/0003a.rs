use std::io::BufRead;

/**
 * A. Shortest path of the king
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * The king is left alone on the chessboard. In spite of this loneliness, he doesn't lose heart, because he has business of
 * national importance. For example, he has to pay an official visit to square t. As the king is not in habit of wasting his
 * time, he wants to get from his current position s to square t in the least number of moves. Help him to do this.
 *
 * In one move the king can get to the square that has a common side or a common vertex with the square the king is
 * currently in (generally there are 8 different squares he can move to).
 *
 * Input
 * The first line contains the chessboard coordinates of square s, the second line — of square t.
 *
 * Chessboard coordinates consist of two characters, the first one is a lowercase Latin letter (from a to h), the second one
 * is a digit from 1 to 8.
 *
 * Output
 * In the first line print n — minimum number of the king's moves. Then in n lines print the moves themselves. Each move is
 * described with one of the 8: L, R, U, D, LU, LD, RU or RD.
 *
 * L, R, U, D stand respectively for moves left, right, up and down (according to the picture), and 2-letter combinations
 * stand for diagonal moves. If the answer is not unique, print any of them.
 *
 * Examples
 *   Input
 *   a8
 *   h1
 *
 *   Output
 *   7
 *   RD
 *   RD
 *   RD
 *   RD
 *   RD
 *   RD
 *   RD
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let from = lines.next().unwrap().map(parse_coordinate).unwrap();
    let to = lines.next().unwrap().map(parse_coordinate).unwrap();

    let mut current = from;
    let mut moves = vec![];

    // Checking for diagonal moves
    let dx = to.0 - current.0;
    let dy = to.1 - current.1;
    if dx != 0 && dy != 0 {
        let direction = match (dx.signum(), dy.signum()) {
            (1, 1) => "RU",
            (1, -1) => "RD",
            (-1, 1) => "LU",
            (-1, -1) => "LD",
            _ => unreachable!(),
        };

        let count_x = dx.unsigned_abs();
        let count_y = dy.unsigned_abs();
        if count_x < count_y {
            moves.push((count_x, direction));
            current = (to.0, current.1 + count_x as isize * dy.signum());
        } else {
            moves.push((count_y, direction));
            current = (current.0 + count_y as isize * dx.signum(), to.1);
        }
    }

    // Checking for straight moves
    let dx = to.0 - current.0;
    let dy = to.1 - current.1;
    if !(dx == 0 && dy == 0) {
        let direction = match (dx.signum(), dy.signum()) {
            (1, _) => "R",
            (-1, _) => "L",
            (_, 1) => "U",
            (_, -1) => "D",
            _ => unreachable!(),
        };

        let count = dx.unsigned_abs().max(dy.unsigned_abs());
        moves.push((count, direction));
    }

    let total_moves: usize = moves.iter().map(|(count, _)| count).sum();
    println!("{total_moves}");
    for (count, dir) in moves {
        (0..count).for_each(|_| println!("{dir}"));
    }
}

fn parse_coordinate(s: String) -> (isize, isize) {
    let mut chars = s.chars();
    let x = chars.next().unwrap() as u8 - b'a' + 1;
    let y = chars.next().unwrap() as u8 - b'0';
    (x as isize, y as isize)
}
