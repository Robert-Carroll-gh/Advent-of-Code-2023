#![allow(dead_code)]
#![allow(unused)]

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part2(input))
}

fn part2(input: &str) -> usize {
    let map = input.chars().collect::<Vec<char>>();
    let start = input.chars().position(|x| x == 'S').unwrap();
    let width = input.chars().position(|x| x == '\n').unwrap() + 1;

    let mut walker = Walker {
        pos: start,

        //Requires manually checking direction for now because I didn't feel like checking for that
        //properly
        direction: Direction::East,
    };

    let mut path: Vec<usize> = Vec::new();
    path.push(start);

    walker.step(&map, width);
    while walker.pos != start {
        path.push(walker.pos);
        walker.step(&map, width);
    }

    let mut in_polygon = false;
    (0..map.len()).fold(0, |acc, i| {
        if !path.contains(&i) {
            if in_polygon {
                return acc + 1;
            };
        } else {
            if let 'L' | '|' | 'J' = &map[i] {
                in_polygon = !in_polygon;
            }
        }
        return acc;
    })
}

#[test]
fn part2test1() {
    assert_eq!(part2(include_str!("p2t1.txt")), 4);
}

#[test]
fn part2test2() {
    assert_eq!(part2(include_str!("p2t2.txt")), 8);
}

#[test]
fn part2test3() {
    assert_eq!(part2(include_str!("p2t3.txt")), 10);
}

struct Walker {
    pos: usize,
    direction: Direction,
}

impl Walker {
    fn step(&mut self, map: &Vec<char>, width: usize) -> char {
        self.pos = match self.direction {
            Direction::East => self.pos + 1,
            Direction::West => self.pos - 1,
            Direction::North => self.pos - width,
            Direction::South => self.pos + width,
        };

        use std::collections::HashMap;
        let mut tiles = HashMap::new();
        tiles.insert('|', Some([Direction::North, Direction::South]));
        tiles.insert('-', Some([Direction::East, Direction::West]));
        tiles.insert('L', Some([Direction::East, Direction::North]));
        tiles.insert('J', Some([Direction::North, Direction::West]));
        tiles.insert('7', Some([Direction::West, Direction::South]));
        tiles.insert('F', Some([Direction::East, Direction::South]));
        tiles.insert('.', None);
        tiles.insert('S', None);
        tiles.insert('\n', None);

        let tile = map[self.pos];
        let opens = tiles.get(&tile).unwrap();

        if opens.is_none() {
            return tile;
        }

        let opens = opens.as_ref().unwrap();
        self.direction = if reverse(self.direction) == opens.as_ref()[0] {
            opens.as_ref()[1]
        } else if reverse(self.direction) == opens.as_ref()[1] {
            opens.as_ref()[0]
        } else {
            self.direction
        };

        return tile;
    }

    fn loop_length(&mut self, map: &Vec<char>, width: usize) -> usize {
        let start: usize = self.pos;

        self.step(map, width);
        let mut steps: usize = 1;

        while self.pos != start {
            self.step(map, width);
            steps += 1;
        }
        return steps;
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn reverse(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn part1(input: &str) -> usize {
    let map = input.chars().collect::<Vec<char>>();
    let start = input.chars().position(|x| x == 'S').unwrap();
    let width = input.chars().position(|x| x == '\n').unwrap() + 1;

    let mut walker = Walker {
        pos: start,
        direction: Direction::East,
    };

    walker.loop_length(&map, width) / 2
}
