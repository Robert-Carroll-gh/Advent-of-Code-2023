#![allow(dead_code)]
#![allow(unused)]

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0u32;

    for line in input.lines() {
        sum += dbg!(process_line(line).get_score());
    }

    return sum;
}

#[test]
fn test1() {
    let test_input = include_str!("testinput.txt");
    assert_eq!(part1(test_input), 13)
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    given_numbers: Vec<u32>,
}

impl Card {
    fn get_score(&self) -> u32 {
        let mut matches = 0u32;
        for num in &self.given_numbers {
            if self.winning_numbers.contains(&num) {
                matches += 1;
            }
        }

        return match matches {
            0 => 0,
            _ => return 2u32.pow(matches - 1),
        };
    }
}

fn process_line(line: &str) -> Card {
    let line = line.strip_prefix("Card ").unwrap().trim();
    let id: u32 = line.chars().next().unwrap().to_digit(10).unwrap();

    let (winning_string, given_string) = line
        .trim_start_matches(char::is_numeric)
        .strip_prefix(": ")
        .unwrap()
        .split_once(" | ")
        .unwrap();
    let winning_numbers = get_numbers(winning_string);
    let given_numbers = get_numbers(given_string);

    return Card {
        id,
        winning_numbers,
        given_numbers,
    };

    fn get_numbers(list: &str) -> Vec<u32> {
        list.split(" ").filter_map(|s| s.parse().ok()).collect()
    }
}
