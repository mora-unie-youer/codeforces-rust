use std::io::BufRead;

/**
 * C. Looking for Order
 * time limit per test: 4 seconds
 * memory limit per test: 512 megabytes
 * input: standard input
 * output: standard output
 *
 * Girl Lena likes it when everything is in order, and looks for order everywhere. Once she was getting ready for the
 * University and noticed that the room was in a mess — all the objects from her handbag were thrown about the room. Of
 * course, she wanted to put them back into her handbag. The problem is that the girl cannot carry more than two objects at
 * a time, and cannot move the handbag. Also, if he has taken an object, she cannot put it anywhere except her handbag — her
 * inherent sense of order does not let her do so.
 *
 * You are given the coordinates of the handbag and the coordinates of the objects in some Сartesian coordinate system. It
 * is known that the girl covers the distance between any two objects in the time equal to the squared length of the segment
 * between the points of the objects. It is also known that initially the coordinates of the girl and the handbag are the
 * same. You are asked to find such an order of actions, that the girl can put all the objects back into her handbag in a
 * minimum time period.
 *
 * Input
 * The first line of the input file contains the handbag's coordinates xs, ys. The second line contains number
 * n (1 ≤ n ≤ 24) — the amount of objects the girl has. The following n lines contain the objects' coordinates. All the
 * coordinates do not exceed 100 in absolute value. All the given positions are different. All the numbers are integer.
 *
 * Output
 * In the first line output the only number — the minimum time the girl needs to put the objects into her handbag.
 *
 * In the second line output the possible optimum way for Lena. Each object in the input is described by its index number
 * (from 1 to n), the handbag's point is described by number 0. The path should start and end in the handbag's point. If
 * there are several optimal paths, print any of them.
 *
 * Examples
 * Input
 * 0 0
 * 2
 * 1 1
 * -1 1
 *
 * Output
 * 8
 * 0 1 2 0
 *
 * Input
 * 1 1
 * 3
 * 4 3
 * 3 4
 * 0 0
 *
 * Output
 * 32
 * 0 1 2 0 3 0
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let handbag = lines.next().unwrap().unwrap();
    let (hx, hy) = handbag.split_once(' ').unwrap();
    let handbag: (isize, isize) = (hx.parse().unwrap(), hy.parse().unwrap());

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let points: Vec<(isize, isize)> = lines
        .take(n)
        .map(Result::unwrap)
        .filter_map(|line| {
            line.split_once(' ')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        })
        .collect();

    let mut distances = vec![vec![0; n]; n];
    for (i, &point1) in points.iter().enumerate() {
        for (j, &point2) in points.iter().enumerate() {
            let total_distance =
                distance(handbag, point1) + distance(point1, point2) + distance(point2, handbag);
            distances[i][j] = total_distance;
        }
    }

    // Iterating over all cases
    let mut costs = vec![0; 1 << n];
    let mut best_moves = vec![0; 1 << n];
    for mask in 1usize..1 << n {
        let i = mask.trailing_zeros() as usize;
        let mask_without_i = mask & !(1 << i);

        let mut best_cost = distances[i][i] + costs[mask_without_i];
        let mut best_move = 0;

        for j in i + 1..n {
            if mask & (1 << j) == 0 {
                continue;
            }

            let mask_without_ij = mask_without_i & !(1 << j);
            let cost = distances[i][j] + costs[mask_without_ij];
            if cost < best_cost {
                best_cost = cost;
                best_move = j;
            }
        }

        costs[mask] = best_cost;
        best_moves[mask] = best_move;
    }

    let initial_mask = (1 << n) - 1;
    println!("{}", costs[initial_mask]);

    let mut mask = initial_mask;
    let mut movements = vec![0.to_string()];
    while mask > 0 {
        let i = mask.trailing_zeros() as usize;
        let best_move = best_moves[mask];

        movements.push((i + 1).to_string());
        if best_move != 0 {
            movements.push((best_move + 1).to_string());
            mask &= !(1 << best_move);
        }

        movements.push(0.to_string());
        mask &= !(1 << i);
    }

    println!("{}", movements.join(" "));
}

fn distance((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
    let dx = x2 - x1;
    let dy = y2 - y1;
    dx * dx + dy * dy
}
