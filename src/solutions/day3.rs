use crate::input::get_input_as_string;

fn input_raw() -> String {
    get_input_as_string("https://adventofcode.com/2021/day/3/input")
}

struct InputData {
    data: Vec<u32>,
    bit_width: usize,
}
fn input() -> InputData {
    let raw = input_raw();
    let bit_width = raw.find("\n").unwrap();
    let data = raw
        .split("\n")
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect();
    InputData { data, bit_width }
}

pub fn d3s1() {
    let input = input();
    let mut digit = 1u32;
    let mut i = 0;
    let mut gamma = 0;
    let mut epsilon = 0;

    while i < input.bit_width {
        let ones_less_zeroes = get_ones_less_zeroes(&input.data, digit);
        if ones_less_zeroes > 0 {
            gamma += digit;
        } else {
            epsilon += digit;
        }
        i += 1;
        digit *= 2;
    }

    println! {"g: {}, e: {}", gamma, epsilon};
    println! {"power: {}", gamma*epsilon};
}

pub fn d3s2() {
    let input = input();
    let bit_width = input.bit_width;
    let mut most_common = input.data.clone();
    let mut least_common = input.data.clone();

    let base:u32 = 2;
    let mut digit: u32;

    digit = base.pow((bit_width-1) as u32);
    while most_common.len() > 1 {
        print_numbers(&most_common, bit_width, digit, "Processing O2 rating");
        most_common = split_vec(&most_common, digit, true);
        digit /= 2;
    }

    digit = base.pow((bit_width-1) as u32);
    while least_common.len() > 1 {
        print_numbers(&least_common, bit_width, digit, "Processing CO2 rating");
        least_common = split_vec(&least_common, digit, false);
        digit /= 2;
    }
    println!("O2 result: [{:0>bit_width$}]", format!("{:b}", most_common[0]));
    println!("O2 rating: {}", most_common[0]);
    println!("CO2 result: [{:0>bit_width$}]", format!("{:b}", least_common[0]));
    println!("CO2 rating: {}", least_common[0]);
    println!("Life Support: {}", most_common[0] * least_common[0]);
}

fn print_numbers(input: &Vec<u32>, bit_width: usize, digit: u32, header: &str) {
    println!("{}", header);
    println!("*{:*>bit_width$}*", "*");
    println!("*{:0>bit_width$}*", format!("{digit:b}"));
    for x in input {
        println!("[{:0>bit_width$}]", format!("{x:b}"));
    }
    println!("*{:*>bit_width$}*", "*");
    println!("");
}

fn get_ones_less_zeroes(input: &Vec<u32>, digit: u32) -> i32 {
    let mut ones_less_zeroes = 0;
    for x in input {
        if digit & x > 0 {
            ones_less_zeroes += 1;
        } else {
            ones_less_zeroes -= 1;
        }
    }

    ones_less_zeroes
}

fn split_vec(input: &Vec<u32>, digit: u32, most_common: bool) -> Vec<u32> {
    let ones_less_zeroes = get_ones_less_zeroes(input, digit);
    let count = (input.len() as i32 / 2) + ones_less_zeroes.abs();
    if most_common {
        if ones_less_zeroes < 0 {
            filter_vec_by_nth_digit(input, digit, false, count as usize)
        } else {
            filter_vec_by_nth_digit(input, digit, true, count as usize)
        }
    } else {
        if ones_less_zeroes < 0 {
            filter_vec_by_nth_digit(input, digit, true, count as usize)
        } else {
            filter_vec_by_nth_digit(input, digit, false, count as usize)
        }
    }
}

fn filter_vec_by_nth_digit(input: &Vec<u32>, digit: u32, one: bool, count: usize) -> Vec<u32> {
    let mut result = Vec::<u32>::with_capacity(count);
    for x in input {
        if digit & x > 0 && one {
            result.push(*x);
        } else if digit & x == 0 && !one {
            result.push(*x);
        }
    }
    
    result
}
