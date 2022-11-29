use crate::input::get_input_as_string;

fn input_raw() -> String {
    get_input_as_string("https://adventofcode.com/2021/day/1/input")
}

fn input() -> Vec<i32> {
    input_raw()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

pub fn d1s1() {
    let input = input();
    let mut count: i32 = 0;
    let mut previous: i32 = input[0];
    for x in &input[1..] {
        if x > &previous {
            count += 1;
        }
        previous = *x;
    }
    println!("{}", count);
}

pub fn d1s2() {
    let input = input();
    let mut count: i32 = 0;
    let mut i = 3;
    while i < input.len() {
        if input[i] + input[i - 1] + input[i - 2] > input[i - 1] + input[i - 2] + input[i - 3] {
			count += 1;
		}
		i += 1;
    }
    println!("{}", count);
}
