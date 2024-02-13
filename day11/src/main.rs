#![allow(dead_code)]
#![allow(unused)]

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

fn part2(input: &str) -> usize {
    calculate(input, 1_000_000)
}

fn part1(input: &str) -> usize {
    calculate(input, 2)
}

fn calculate(input: &str, expansion_rate: usize) -> usize {
    let (galaxies, expanding_rows, expanding_collumns) = parse_input(input);
    let mut sum = 0;
    let mut combos = 0;
    // dbg!(&expanding_rows);
    // dbg!(&expanding_collumns);
    // dbg!(&galaxies);
    for i in (0..galaxies.len() - 1) {
        for j in (i + 1..galaxies.len()) {
            let x_distance = cardinal_distance(
                galaxies[i].x,
                galaxies[j].x,
                &expanding_rows,
                expansion_rate,
            );
            let y_distance = cardinal_distance(
                galaxies[i].y,
                galaxies[j].y,
                &expanding_collumns,
                expansion_rate,
            );
            let total_distance = x_distance + y_distance;
            combos += 1;
            sum += total_distance;
            // println!(
            //     "value {}: g{} to g{}: x = {}, y = {}, total = {}",
            //     combos,
            //     i + 1,
            //     j + 1,
            //     x_distance,
            //     y_distance,
            //     total_distance
            // );
        }
    }
    return sum;
}

#[derive(Debug)]
struct Galaxy {
    x: u8,
    y: u8,
}

fn parse_input(input: &str) -> (Vec<Galaxy>, Vec<u8>, Vec<u8>) {
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut empty_rows = [true; 255];
    let mut empty_collumns = [true; 255];

    let (mut walker_x, mut walker_y) = (0u8, 0u8);
    let (mut width, mut height) = (0u8, 0u8);

    //walking through the input, marking Galaxy positions,
    //and empty rows and collumns
    for line in input.lines() {
        for tile in line.bytes() {
            if tile == b'#' {
                galaxies.push(Galaxy {
                    x: walker_x,
                    y: walker_y,
                });
                empty_rows[walker_x as usize] = false;
                empty_collumns[walker_y as usize] = false;
            }
            walker_x += 1;
        }
        width = walker_x - 1;
        height = walker_y;
        walker_y += 1;
        walker_x = 0;
    }

    //creating sorted vecs of the positions of empty rows and collumns
    let mut expanding_rows = Vec::new();
    let mut expanding_collumns = Vec::new();
    for i in (0..std::cmp::max(width, height)) {
        if empty_rows[i as usize] {
            expanding_rows.push(i)
        };
        if empty_collumns[i as usize] {
            expanding_collumns.push(i)
        };
    }
    // dbg!(&empty_rows);
    // dbg!(&expanding_rows);

    return (galaxies, expanding_rows, expanding_collumns);
}

fn cardinal_distance(pos1: u8, pos2: u8, expansions: &Vec<u8>, expansion_rate: usize) -> usize {
    pos1.abs_diff(pos2) as usize
        + expansions
            .iter()
            .filter(|e| (pos1..pos2).contains(e) | (pos2..pos1).contains(e))
            .count()
            * (expansion_rate - 1)
}
