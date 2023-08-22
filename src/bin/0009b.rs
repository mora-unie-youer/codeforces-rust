use std::io::BufRead;

/**
 * B. Running Student
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * And again a misfortune fell on Poor Student. He is being late for an exam.
 *
 * Having rushed to a bus stop that is in point (0, 0), he got on a minibus and they drove along a straight line, parallel
 * to axis OX, in the direction of increasing x.
 *
 * Poor Student knows the following:
 *   during one run the minibus makes n stops, the i-th stop is in point (xi, 0)
 *   coordinates of all the stops are different
 *   the minibus drives at a constant speed, equal to vb
 *   it can be assumed the passengers get on and off the minibus at a bus stop momentarily
 *   Student can get off the minibus only at a bus stop
 *   Student will have to get off the minibus at a terminal stop, if he does not get off earlier
 *   the University, where the exam will be held, is in point (xu, yu)
 *   Student can run from a bus stop to the University at a constant speed vs as long as needed
 *   a distance between two points can be calculated according to the following formula:
 *     d = sqrt((x_2 - x_1) ^ 2 + (y_2 - y_1) ^ 2)
 *   Student is already on the minibus, so, he cannot get off at the first bus stop
 * Poor Student wants to get to the University as soon as possible. Help him to choose the bus stop, where he should get
 * off. If such bus stops are multiple, choose the bus stop closest to the University.
 *
 * Input
 * The first line contains three integer numbers: 2 ≤ n ≤ 100, 1 ≤ vb, vs ≤ 1000. The second line contains n non-negative
 * integers in ascending order: coordinates xi of the bus stop with index i. It is guaranteed that x1 equals to zero, and
 * xn ≤ 105. The third line contains the coordinates of the University, integers xu and yu, not exceeding 105 in absolute
 * value.
 *
 * Output
 * In the only line output the answer to the problem — index of the optimum bus stop.
 *
 * Examples
 *   Input
 *   4 5 2
 *   0 2 4 6
 *   4 1
 *   
 *   Output
 *   3
 *   
 *   Input
 *   2 1 1
 *   0 100000
 *   100000 100000
 *   
 *   Output
 *   2
 *   
 *   Note
 *   As you know, students are a special sort of people, and minibuses usually do not hurry. That's why you should not be
 * surprised, if Student's speed is higher than the speed of the minibus.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line = first_line
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap());

    let n: usize = first_line.next().unwrap();
    let vb = first_line.next().unwrap();
    let vs = first_line.next().unwrap();
    let (vb, vs) = (vb as f64, vs as f64);

    let bus_stops: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .take(n)
        .map(|v| v.parse().unwrap())
        .collect();

    let university_line = lines.next().unwrap().unwrap();
    let (ux, uy) = university_line.split_once(' ').unwrap();
    let (ux, uy): (isize, isize) = (ux.parse().unwrap(), uy.parse().unwrap());
    let (ux, uy) = (ux as f64, uy as f64);

    let mut time_to_stop = vec![0.; bus_stops.len()];
    for i in 1..time_to_stop.len() {
        let prev_stop = bus_stops[i - 1];
        let stop = bus_stops[i];
        time_to_stop[i] = time_to_stop[i - 1] + (stop - prev_stop) as f64 / vb;
    }

    let bus_stop = bus_stops
        .into_iter()
        .zip(time_to_stop)
        .map(|(x, bus_time)| {
            let (dx, dy) = (ux - x as f64, uy);
            let distance = (dx * dx + dy * dy).sqrt();
            let student_time = distance / vs;

            (bus_time + student_time, distance)
        })
        .enumerate()
        .skip(1)
        .min_by(|(_, (at, ad)), (_, (bt, bd))| {
            at.partial_cmp(bt)
                .unwrap()
                .then(ad.partial_cmp(bd).unwrap())
        })
        .map(|(i, _)| i + 1)
        .unwrap();

    println!("{bus_stop}");
}
