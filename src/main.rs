use std::env;

mod input;
mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let func = args.get(1).expect("Must provide a runtime argument.");
    println!("----------------------------------------");
    println!("| Running: {}", func);
    println!("----------------------------------------");

    match &func[..] {
        "d1s1" => solutions::day1::d1s1(),
        "d1s2" => solutions::day1::d1s2(),
        "d2s1" => solutions::day2::d2s1(),
        "d2s2" => solutions::day2::d2s2(),
        "d3s1" => solutions::day3::d3s1(),
        "d3s2" => solutions::day3::d3s2(),
        _ => {println!("Invalid argument.")},
    }
}
