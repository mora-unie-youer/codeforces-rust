use std::io::BufRead;

/**
 * B. Center Alignment
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * Almost every text editor has a built-in function of center text alignment. The developers of the popular in Berland text
 * editor «Textpad» decided to introduce this functionality into the fourth release of the product.
 *
 * You are to implement the alignment in the shortest possible time. Good luck!
 *
 * Input
 * The input file consists of one or more lines, each of the lines contains Latin letters, digits and/or spaces. The lines
 * cannot start or end with a space. It is guaranteed that at least one of the lines has positive length. The length of each
 * line and the total amount of the lines do not exceed 1000.
 *
 * Output
 * Format the given text, aligning it center. Frame the whole text with characters «*» of the minimum size. If a line cannot
 * be aligned perfectly (for example, the line has even length, while the width of the block is uneven), you should place
 * such lines rounding down the distance to the left or to the right edge and bringing them closer left or right
 * alternatively (you should start with bringing left). Study the sample tests carefully to understand the output format
 * better.
 *
 * Examples
 *   Input
 *   This  is
 *   
 *   Codeforces
 *   Beta
 *   Round
 *   5
 *   
 *   Output
 *   ************
 *   * This  is *
 *   *          *
 *   *Codeforces*
 *   *   Beta   *
 *   *  Round   *
 *   *     5    *
 *   ************
 *   
 *   Input
 *   welcome to the
 *   Codeforces
 *   Beta
 *   Round 5
 *   
 *   and
 *   good luck
 *   
 *   Output
 *   ****************
 *   *welcome to the*
 *   *  Codeforces  *
 *   *     Beta     *
 *   *   Round 5    *
 *   *              *
 *   *      and     *
 *   *  good luck   *
 *   ****************
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let lines = lines.collect::<Result<Vec<String>, _>>().unwrap();
    let length = lines.iter().map(|line| line.len()).max().unwrap();

    // Start of block
    for _ in 0..length + 2 {
        print!("*");
    }
    println!();

    let mut i = 0;
    for line in lines.into_iter() {
        let spaces = length - line.len();
        i += (spaces % 2 == 1) as usize;
        print!("*");

        if spaces % 2 == 1 && i % 2 == 0 {
            for _ in 0..=spaces / 2 {
                print!(" ");
            }
        } else {
            for _ in 0..spaces / 2 {
                print!(" ");
            }
        }

        print!("{line}");

        if spaces % 2 == 1 && i % 2 == 1 {
            for _ in 0..=spaces / 2 {
                print!(" ");
            }
        } else {
            for _ in 0..spaces / 2 {
                print!(" ");
            }
        }

        print!("*");

        println!();
    }

    // End of block
    for _ in 0..length + 2 {
        print!("*");
    }
    println!();
}
