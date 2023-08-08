use std::io::BufRead;

/**
 * D. Follow Traffic Rules
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * Everybody knows that the capital of Berland is connected to Bercouver (the Olympic capital) by a direct road. To improve
 * the road's traffic capacity, there was placed just one traffic sign, limiting the maximum speed. Traffic signs in
 * Berland are a bit peculiar, because they limit the speed only at that point on the road where they are placed. Right
 * after passing the sign it is allowed to drive at any speed.
 *
 * It is known that the car of an average Berland citizen has the acceleration (deceleration) speed of a km/h2, and has
 * maximum speed of v km/h. The road has the length of l km, and the speed sign, limiting the speed to w km/h, is placed
 * d km (1 ≤ d < l) away from the capital of Berland. The car has a zero speed at the beginning of the journey. Find the
 * minimum time that an average Berland citizen will need to get from the capital to Bercouver, if he drives at the optimal
 * speed.
 *
 * The car can enter Bercouver at any speed.
 *
 * Input
 * The first line of the input file contains two integer numbers a and v (1 ≤ a, v ≤ 10000). The second line contains three
 * integer numbers l, d and w (2 ≤ l ≤ 10000; 1 ≤ d < l; 1 ≤ w ≤ 10000).
 *
 * Output
 * Print the answer with at least five digits after the decimal point.
 *
 * Examples
 *   Input
 *   1 1
 *   2 1 3
 *   
 *   Output
 *   2.500000000000
 *   
 *   Input
 *   5 70
 *   200 170 40
 *   
 *   Output
 *   8.965874696353
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let mut line = line.split(' ').map(|v| v.parse().unwrap());
    let a: f64 = line.next().unwrap();
    let v = line.next().unwrap();

    let line = lines.next().unwrap().unwrap();
    let mut line = line.split(' ').map(|v| v.parse().unwrap());
    let l: f64 = line.next().unwrap();
    let d = line.next().unwrap();
    let w = line.next().unwrap();

    let total_time = if w > v || (2. * a * d) <= w * w {
        // If car's limit is lower than sign's limit, we will drive all the way with one max speed
        // or
        // If speed we can reach on this distance is lower than sign's limit, we will drive all
        // the way with one max speed
        get_time_for_range(a, v, l, 0.)
    } else {
        // So now we need to drive with two different "methods".

        // Time to accelerate
        let t = w / a;
        // Time to reach max speed
        let t0 = (d / a + t * t / 2.).sqrt();
        // Time to decellerate to limited speed
        let t1 = t0 - t;

        if a * t0 > v {
            // If we reach speed in that time bigger than our car's limit

            // Time to reach max speed
            let t0 = v / a;
            // Time to deccelerate to limited speed
            let t2 = (v - w) / a;
            // Time of moving on max speed
            let t1 = (d - a * t0 * t0 / 2. - v * t2 + a * t2 * t2 / 2.) / v;

            t0 + t1 + t2 + get_time_for_range(a, v, l - d, w)
        } else {
            t0 + t1 + get_time_for_range(a, v, l - d, w)
        }
    };

    println!("{total_time:.16}")
}

fn get_time_for_range(a: f64, v: f64, x: f64, v0: f64) -> f64 {
    // Time and path if we only accelerated
    let t0 = (v - v0) / a;
    let x0 = a * t0 * t0 / 2. + t0 * v0;

    if x0 > x {
        // This case is more complex.
        // Just solving quadratic equation: at^2 + 2vt - x = 0
        (-v0 + (v0 * v0 + 2. * a * x).sqrt()) / a
    } else {
        // This case is simple -> we then move with limited speed
        t0 + (x - x0) / v
    }
}
