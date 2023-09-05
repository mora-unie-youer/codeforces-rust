use std::io::BufRead;

/**
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (n, x0) = first_line.split_once(' ').unwrap();
    let (n, x0): (usize, isize) = (n.parse().unwrap(), x0.parse().unwrap());

    let ranges: Vec<(isize, isize)> = lines
        .take(n)
        .map(Result::unwrap)
        .filter_map(|line| {
            line.split_once(' ')
                .map(|(from, to)| (from.parse().unwrap(), to.parse().unwrap()))
        })
        .collect();

    // Finding intersection
    let mut common_range = (std::isize::MIN, std::isize::MAX);
    for (from, to) in ranges.into_iter() {
        let mut v = [from, to];
        v.sort_unstable();
        let [from, to] = v;

        common_range.0 = from.max(common_range.0);
        common_range.1 = to.min(common_range.1);
    }

    if common_range.0 > common_range.1 {
        println!("-1");
    } else if common_range.0 <= x0 && x0 <= common_range.1 {
        println!("0");
    } else {
        println!(
            "{}",
            std::cmp::min(x0.abs_diff(common_range.0), x0.abs_diff(common_range.1))
        );
    }
}
