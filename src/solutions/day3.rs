use crate::input::get_input_as_string;

fn input() -> String {
    get_input_as_string("https://adventofcode.com/2021/day/3/input")
}

pub fn d3s1() {
    let input = input();
    let mut split = input.split("\n");
    let first = split.nth(0).expect("invalid input");
    let mut data: Vec<i32> = Vec::new();
    for _ in first.chars() {
        data.push(0);
    }
    for s in split {
        let mut i = 0;
        for c in s.chars() {
            match c {
                '0' => data[i] -= 1,
                '1' => data[i] += 1,
                _ => {}
            }
            i += 1;
        }
    }
    println!("{:?}", data);

    let mut gamma = 0;
    let mut epsilon = 0;
    let mut digit = 1;

    data.reverse();
    for d in data {
        if d > 0 {
            gamma += digit;
        } else {
            epsilon += digit;
        }
        digit *= 2;
    }

    println! {"g: {}, e: {}", gamma, epsilon};
    println! {"power: {}", gamma*epsilon};
}

pub fn d3s2() {
    let input = input();
    let mut split = input.split("\n");
    let first = split.nth(0).expect("invalid input");
    let mut data: Vec<i32> = Vec::new();
    for _ in first.chars() {
        data.push(0);
    }
    for s in split {
        let mut i = 0;
        for c in s.chars() {
            match c {
                '0' => data[i] -= 1,
                '1' => data[i] += 1,
                _ => {}
            }
            i += 1;
        }
    }
    println!("{:?}", data);
}

// fn get_most_common(input: Vec<i32>, digit: i32) -> Vec<i32> {
//     let number_of_ones_less_zeros = 0;
//     for iter in input {
//         match iter.chars[]
//     }
// }
