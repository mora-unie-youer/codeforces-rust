use std::{cmp::Reverse, io::BufRead};

/**
 * B. Lorry
 * time limit per test: 2 seconds
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * A group of tourists is going to kayak and catamaran tour. A rented lorry has arrived to the boat depot to take kayaks and
 * catamarans to the point of departure. It's known that all kayaks are of the same size (and each of them occupies the
 * space of 1 cubic metre), and all catamarans are of the same size, but two times bigger than kayaks (and occupy the space
 * of 2 cubic metres).
 *
 * Each waterborne vehicle has a particular carrying capacity, and it should be noted that waterborne vehicles that look the
 * same can have different carrying capacities. Knowing the truck body volume and the list of waterborne vehicles in the
 * boat depot (for each one its type and carrying capacity are known), find out such set of vehicles that can be taken in
 * the lorry, and that has the maximum total carrying capacity. The truck body volume of the lorry can be used effectively,
 * that is to say you can always put into the lorry a waterborne vehicle that occupies the space not exceeding the free
 * space left in the truck body.
 *
 * Input
 * The first line contains a pair of integer numbers n and v (1 ≤ n ≤ 105; 1 ≤ v ≤ 109), where n is the number of
 * waterborne vehicles in the boat depot, and v is the truck body volume of the lorry in cubic metres. The following n lines
 * contain the information about the waterborne vehicles, that is a pair of numbers t_i, p_i (1 ≤ t_i ≤ 2; 1 ≤ p_i ≤ 104),
 * where t_i is the vehicle type (1 – a kayak, 2 – a catamaran), and p_i is its carrying capacity. The waterborne vehicles
 * are enumerated in order of their appearance in the input file.
 *
 * Output
 * In the first line print the maximum possible carrying capacity of the set. In the second line print a string consisting
 * of the numbers of the vehicles that make the optimal set. If the answer is not unique, print any of them.
 *
 * Examples
 *   Input
 *   3 2
 *   1 2
 *   2 7
 *   1 3
 *   
 *   Output
 *   7
 *   2
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first = lines.next().unwrap().unwrap();
    let (n, v) = first.split_once(' ').unwrap();

    let (n, v): (usize, usize) = (n.parse().unwrap(), v.parse().unwrap());
    let mut kayak = vec![];
    let mut catamaran = vec![];

    for (i, line) in lines.take(n).enumerate() {
        let line = line.unwrap();
        let (t, p) = line.split_once(' ').unwrap();
        let (t, p): (usize, usize) = (t.parse().unwrap(), p.parse().unwrap());

        match t {
            1 => kayak.push((i + 1, p)),
            2 => catamaran.push((i + 1, p)),
            _ => unreachable!(),
        }
    }

    // Sort by capacity
    kayak.sort_by_key(|&(_, p)| Reverse(p));
    catamaran.sort_by_key(|&(_, p)| Reverse(p));

    let mut total_capacity = 0;

    let mut kayak_count = kayak.len().min(v);
    total_capacity += kayak
        .iter()
        .take(kayak_count)
        .map(|(_, p)| p)
        .sum::<usize>();

    let mut catamaran_count = catamaran.len().min((v - kayak_count) / 2);
    total_capacity += catamaran
        .iter()
        .take(catamaran_count)
        .map(|(_, p)| p)
        .sum::<usize>();

    let mut max_capacity = total_capacity;
    let mut best_kayak_count = kayak_count;
    let mut best_catamaran_count = catamaran_count;
    // Removing kayaks if possible
    while let Some(new_kayak_count) = kayak_count.checked_sub(1) {
        // If there's no available catamarans -> breaking
        if catamaran_count >= catamaran.len() {
            break;
        }

        // Removing kayaks to make space for catamaran
        kayak_count = new_kayak_count;
        total_capacity -= kayak[kayak_count].1;

        // Adding catamaran if possible
        if new_kayak_count + (catamaran_count + 1) * 2 <= v {
            total_capacity += catamaran[catamaran_count].1;
            catamaran_count += 1;
        }

        // Checking for best result
        if total_capacity > max_capacity {
            max_capacity = total_capacity;
            best_kayak_count = kayak_count;
            best_catamaran_count = catamaran_count;
        }
    }

    println!("{max_capacity}");
    let vehicles: Vec<_> = kayak
        .into_iter()
        .take(best_kayak_count)
        .chain(catamaran.into_iter().take(best_catamaran_count))
        .map(|(i, _)| i.to_string())
        .collect();
    println!("{}", vehicles.join(" "))
}
