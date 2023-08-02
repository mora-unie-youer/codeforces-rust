use std::io::BufRead;

/**
 * B. Before an Exam
 * time limit per test: 0.5 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * Tomorrow Peter has a Biology exam. He does not like this subject much, but d days ago he learnt that he would have to
 * take this exam. Peter's strict parents made him prepare for the exam immediately, for this purpose he has to study not
 * less than minTime_i and not more than maxTime_i hours per each i-th day. Moreover, they warned Peter that a day before
 * the exam they would check how he has followed their instructions.
 *
 * So, today is the day when Peter's parents ask him to show the timetable of his preparatory studies. But the boy has
 * counted only the sum of hours sumTime spent him on preparation, and now he wants to know if he can show his parents a
 * timetable sсhedule with d numbers, where each number sсhedule_i stands for the time in hours spent by Peter each i-th day
 * on biology studies, and satisfying the limitations imposed by his parents, and at the same time the sum total of all
 * schedule_i should equal to sumTime.
 *
 * Input
 * The first input line contains two integer numbers d, sumTime (1 ≤ d ≤ 30, 0 ≤ sumTime ≤ 240) — the amount of days,
 * during which Peter studied, and the total amount of hours, spent on preparation. Each of the following d lines contains
 * two integer numbers minTimei, maxTime_i (0 ≤ minTime_i ≤ maxTime_i ≤ 8), separated by a space — minimum and maximum amount
 * of hours that Peter could spent in the i-th day.
 *
 * Output
 * In the first line print YES, and in the second line print d numbers (separated by a space), each of the numbers — amount
 * of hours, spent by Peter on preparation in the corresponding day, if he followed his parents' instructions; or print NO
 * in the unique line. If there are many solutions, print any of them.
 *
 * Examples
 *   Input
 *   1 48
 *   5 7
 *   
 *   Output
 *   NO
 *   
 *   Input
 *   2 5
 *   0 1
 *   3 5
 *   
 *   Output
 *   YES
 *   1 4
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (d, sum_time) = first_line.split_once(' ').unwrap();
    let (d, sum_time): (usize, usize) = (d.parse().unwrap(), sum_time.parse().unwrap());

    let mut days = Vec::with_capacity(d);
    let (mut total_min, mut total_max) = (0, 0);
    for line in lines.take(d) {
        let line = line.unwrap();
        let (min, max) = line.split_once(' ').unwrap();
        let (min, max): (usize, usize) = (min.parse().unwrap(), max.parse().unwrap());

        days.push((min, max));
        total_min += min;
        total_max += max;
    }

    if sum_time < total_min || sum_time > total_max {
        println!("NO");
        return;
    }

    let mut total_spent = total_min;
    let mut day_times = Vec::with_capacity(d);
    for (min_time, max_time) in days {
        let diff = sum_time - total_spent;

        if diff == 0 {
            day_times.push(min_time.to_string());
        } else {
            let available = max_time - min_time;
            let spent = available.min(diff);

            let day = min_time + spent;
            day_times.push(day.to_string());
            total_spent += spent;
        }
    }

    println!("YES");
    println!("{}", day_times.join(" "));
}
