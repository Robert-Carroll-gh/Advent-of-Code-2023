#![allow(dead_code)]
#![allow(unused)]

fn main() {
    let input = input();
    println!("{}", part2());
}

fn part2() -> usize {
    record_breaking_options(51699878, 377117112241505)
}

fn part1(input: &str) -> usize {
    let (time_available, current_record) = parse_input(input);
    
    time_available.iter().enumerate().map(|(index, time)| {
        let record = current_record[index];
        dbg!(record_breaking_options(*time, record))
    }).product()
}

#[test]
fn test1() {
    assert_eq!(288, part1(test_input()))
}

fn record_breaking_options(time: usize, record: usize) -> usize {
    (0..time).filter(|charge_time| (charge_time * (time - charge_time)) > record).count()
}

fn parse_input(input: &str) -> ([usize; 4], [usize; 4]) {
    let mut lines = input.lines().map(|line| {
        let mut words = line.split(" ").filter_map(|word| word.parse::<usize>().ok());
        [words.next().unwrap(), words.next().unwrap(), words.next().unwrap(), words.next().unwrap()]
    });
    (lines.next().unwrap(), lines.next().unwrap())
}

fn test_input() -> &'static str {
"Time:      7  15   30 4
Distance:  9  40  200 2"
}

fn input() -> &'static str {
"Time:        51     69     98     78
Distance:   377   1171   1224   1505"
}
