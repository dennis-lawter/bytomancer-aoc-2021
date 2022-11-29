use std::str::FromStr;

use crate::input::get_input_as_string;

fn input() -> String {
    get_input_as_string("https://adventofcode.com/2021/day/2/input")
}

enum Movement {
    Forward,
    Up,
    Down,
}
impl FromStr for Movement {
    type Err = ();
    fn from_str(input: &str) -> Result<Movement, ()> {
        match input {
            "forward" => Ok(Movement::Forward),
            "up" => Ok(Movement::Up),
            "down" => Ok(Movement::Down),
            _ => Err(()),
        }
    }
}

struct Command {
    pub movement: Movement,
    pub scalar: i64,
}
impl FromStr for Command {
    type Err = ();
    fn from_str(input: &str) -> Result<Command, ()> {
        let mut split = input.split(" ");
        let movement = Movement::from_str(split.next().unwrap()).unwrap();
        let scalar = split.next().unwrap().parse::<i64>().unwrap();
        Ok(Self {
            movement: movement,
            scalar: scalar,
        })
    }
}

pub fn d2s1() {
    let input = input();
    let split = input.split("\n");

    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;

    for s in split {
        let command = Command::from_str(s).expect("Invalid command");
        match command.movement {
            Movement::Forward => horizontal += command.scalar,
            Movement::Up => depth -= command.scalar,
            Movement::Down => depth += command.scalar,
        }
    }

    println!("{}", horizontal * depth);
}

pub fn d2s2() {
    let input = input();
    let split = input.split("\n");

    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;

    for s in split {
        let command = Command::from_str(s).expect("Invalid command");
        match command.movement {
            Movement::Forward => {
                horizontal += command.scalar;
                depth += aim * command.scalar
            }
            Movement::Up => aim -= command.scalar,
            Movement::Down => aim += command.scalar,
        }
    }

    println!("{}", horizontal * depth);
}
