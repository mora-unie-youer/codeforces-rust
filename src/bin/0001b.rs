use std::io::BufRead;

/**
 * B. Spreadsheets
 * time limit per test: 10 seconds
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * In the popular spreadsheets systems (for example, in Excel) the following numeration of columns is used. The first column
 * has number A, the second — number B, etc. till column 26 that is marked by Z. Then there are two-letter numbers: column
 * 27 has number AA, 28 — AB, column 52 is marked by AZ. After ZZ there follow three-letter numbers, etc.
 *
 * The rows are marked by integer numbers starting with 1. The cell name is the concatenation of the column and the row
 * numbers. For example, BC23 is the name for the cell that is in column 55, row 23.
 *
 * Sometimes another numeration system is used: RXCY, where X and Y are integer numbers, showing the column and the row
 * numbers respectfully. For instance, R23C55 is the cell from the previous example.
 *
 * Your task is to write a program that reads the given sequence of cell coordinates and produce each item written according
 * to the rules of another numeration system.
 *
 * Input
 * The first line of the input contains integer number n (1 ≤ n ≤ 105), the number of coordinates in the test. Then there
 * follow n lines, each of them contains coordinates. All the coordinates are correct, there are no cells with the column
 * and/or the row numbers larger than 106 .
 *
 * Output
 * Write n lines, each line should contain a cell coordinates in the other numeration system.
 *
 * Examples:
 *   Input
 *   2
 *   R23C55
 *   BC23
 *   
 *   Output
 *   BC23
 *   R23C55
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        println!("{}", convert(&line));
    }
}

fn convert(s: &str) -> String {
    if is_excel(s) {
        let split = s.chars().position(|ch| ch.is_ascii_digit()).unwrap();
        let (col, row) = (&s[..split], &s[split..]);
        let col = col
            .chars()
            .fold(0, |acc, ch| acc * 26 + (ch as u8 - b'A' + 1) as usize);
        format!("R{row}C{col}")
    } else {
        let mut parts = s.split(['R', 'C']);
        let (row, col) = (parts.nth(1).unwrap(), parts.next().unwrap());
        let (row, mut col): (usize, usize) = (row.parse().unwrap(), col.parse().unwrap());

        let mut col_string = String::new();
        while col > 0 {
            col_string.push((b'A' + ((col - 1) % 26) as u8) as char);
            col = (col - 1) / 26;
        }
        let col_string: String = col_string.chars().rev().collect();

        format!("{col_string}{row}")
    }
}

fn is_excel(s: &str) -> bool {
    let col_len = s.chars().take_while(|ch| !ch.is_ascii_digit()).count();
    let row_len = s
        .chars()
        .rev()
        .take_while(|ch| !ch.is_ascii_alphabetic())
        .count();

    s.len() == col_len + row_len
}
