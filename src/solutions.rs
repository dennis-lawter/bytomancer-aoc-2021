use crate::input::get_input_as_string;

fn d1input() -> String {
	let url = "https://adventofcode.com/2021/day/1/input".to_string();
	get_input_as_string(&url).trim_end().to_string()
}

pub fn d1s1() {
	let d1input = d1input();
    let mut split = d1input.split("\n");
    let mut count: i32 = 0;
    let mut previous: i32 = split.next().unwrap().parse::<i32>().expect("Invalid input");
    for s in split {
        let x: i32 = s.parse::<i32>().expect("Invalid input");
        if x > previous {
            count += 1;
        }
        previous = x;
    }
    println!("{}", count);
}

pub fn d1s2() {
	let d1input = d1input();
    let mut split = d1input.split("\n");
    let mut count: i32 = 0;
    let mut a: i32 = split.next().unwrap().parse::<i32>().expect("Invalid input");
    let mut b: i32 = split.next().unwrap().parse::<i32>().expect("Invalid input");
    let mut c: i32 = split.next().unwrap().parse::<i32>().expect("Invalid input");
    for s in split {
        let x: i32 = s.parse::<i32>().expect("Invalid input");
        if x + b + c > a + b + c {
            count += 1;
        }
        a = b;
        b = c;
        c = x;
    }
    println!("{}", count);
}
