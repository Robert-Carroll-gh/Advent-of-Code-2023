//this was what I orriginally tried writing before I realized how small the end project was
//actually supposed to be. Leaving this here to remind myself to prototype faster and not solve
//problems that don't exist yet

#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashMap;

fn main() {}

fn parse_input(input: &str) -> InputData {
    let (first_line, _) = input.split_once("\n").unwrap();

    let width: i32 = first_line.len() as i32;
    let height: i32 = input.lines().count() as i32;
    let start: Point = {
        let (index, _) = input
            .chars()
            .enumerate()
            .find(|(index, character)| character == &'S')
            .unwrap();
        let index = index as i32;
        let x: i32 = index % (width + 1);
        let y: i32 = (index / (width + 1)).parse::<f64>().unwrap().floor();
        Point(x, y)
    };

    InputData {
        width,
        height,
        start,
    }
}

struct State {
    point: Point,
    facing: Direction,
}

struct InputData {
    //map: String,
    width: i32,
    height: i32,
    start: Point,
}
impl InputData {
    fn char_at(&self, point: &Point) -> char {
        self.map
            .chars()
            .nth(((point.1 * (self.width + 1)) + point.0) as usize)
            .unwrap()
    }

    fn walk(&self, current_state: State) -> Option<State> {
        let next_point = current_state.point.add(current_state.facing.delta());
        let next_char = self.char_at(&next_point);

        match next_char {
            'S' => return None,
            _ => {
                let next_pipe = Pipe::new(next_char);
                let next_direction = next_pipe.redirect(current_state.facing);
                let next_state = State {
                    point: next_point,
                    facing: next_direction,
                };
                return Some(next_state);
            }
        }
    }
}

struct Point(i32, i32);
impl Point {
    fn add(&self, delta: Point) -> Point {
        Point(self.0 + delta.0, self.1 + delta.1)
    }
}

#[derive(Copy, Clone)]
struct Pipe(Direction, Direction);
impl Pipe {
    fn redirect(&self, entrance: Direction) -> Direction {
        if entrance == self.0 {
            self.1
        } else if entrance == self.1 {
            self.0
        } else {
            panic!()
        }
    }

    fn new(character: char) -> Pipe {
        HashMap::from([
            ('|', Pipe(North, South)),
            ('-', Pipe(East, West)),
            ('L', Pipe(North, East)),
            ('J', Pipe(North, West)),
            ('7', Pipe(West, South)),
            ('F', Pipe(East, South)),
        ])
        .get(&character)
        .expect("Invalid pipe char")
        .to_owned()
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    fn delta(&self) -> Point {
        match self {
            North => Point(0, -1),
            East => Point(1, 0),
            South => Point(0, 1),
            West => Point(-1, 0),
        }
    }
}
