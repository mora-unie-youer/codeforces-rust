use std::io::BufRead;

/**
 * A. Train and Peter
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * Peter likes to travel by train. He likes it so much that on the train he falls asleep.
 *
 * Once in summer Peter was going by train from city A to city B, and as usual, was sleeping. Then he woke up, started to
 * look through the window and noticed that every railway station has a flag of a particular colour.
 *
 * The boy started to memorize the order of the flags' colours that he had seen. But soon he fell asleep again.
 * Unfortunately, he didn't sleep long, he woke up and went on memorizing the colours. Then he fell asleep again, and that
 * time he slept till the end of the journey.
 *
 * At the station he told his parents about what he was doing, and wrote two sequences of the colours that he had seen
 * before and after his sleep, respectively.
 *
 * Peter's parents know that their son likes to fantasize. They give you the list of the flags' colours at the stations that
 * the train passes sequentially on the way from A to B, and ask you to find out if Peter could see those sequences on the
 * way from A to B, or from B to A. Remember, please, that Peter had two periods of wakefulness.
 *
 * Peter's parents put lowercase Latin letters for colours. The same letter stands for the same colour, different letters —
 * for different colours.
 *
 * Input
 * The input data contains three lines. The first line contains a non-empty string, whose length does not exceed 105, the
 * string consists of lowercase Latin letters — the flags' colours at the stations on the way from A to B. On the way from
 * B to A the train passes the same stations, but in reverse order.
 *
 * The second line contains the sequence, written by Peter during the first period of wakefulness. The third line contains
 * the sequence, written during the second period of wakefulness. Both sequences are non-empty, consist of lowercase Latin
 * letters, and the length of each does not exceed 100 letters. Each of the sequences is written in chronological order.
 *
 * Output
 * Output one of the four words without inverted commas:
 *   «forward» — if Peter could see such sequences only on the way from A to B;
 *   «backward» — if Peter could see such sequences on the way from B to A;
 *   «both» — if Peter could see such sequences both on the way from A to B, and on the way from B to A;
 *   «fantasy» — if Peter could not see such sequences.
 *
 * Examples
 *   Input
 *   atob
 *   a
 *   b
 *   
 *   Output
 *   forward
 *   
 *   Input
 *   aaacaaa
 *   aca
 *   aa
 *   
 *   Output
 *   both
 *
 * Note
 * It is assumed that the train moves all the time, so one flag cannot be seen twice. There are no flags at stations A and B
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let path = lines.next().unwrap().unwrap();
    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();

    let left = path
        .find(&a)
        .and_then(|i| path[i + a.len()..].find(&b))
        .is_some();

    // Reversing names
    let a: String = a.chars().rev().collect();
    let b: String = b.chars().rev().collect();
    let right = path.rfind(&a).and_then(|i| path[..i].rfind(&b)).is_some();

    match (left, right) {
        (true, true) => println!("both"),
        (true, _) => println!("forward"),
        (_, true) => println!("backward"),
        _ => println!("fantasy"),
    }
}
