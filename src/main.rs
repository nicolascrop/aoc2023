mod resolvers;

use std::env;
use std::fs;

fn get_file_content(file_path: &String) -> String {
    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return content;
}

fn main() {
    let resolvers = [
        resolvers::advent_01::resolve,
        resolvers::advent_02::resolve,
        resolvers::advent_03::resolve,
        resolvers::advent_04::resolve,
        resolvers::advent_05::resolve,
        resolvers::advent_06::resolve,
        resolvers::advent_07::resolve,
        resolvers::advent_08::resolve,
        resolvers::advent_09::resolve,
        resolvers::advent_10::resolve,
        resolvers::advent_11::resolve,
        resolvers::advent_12::resolve,
        resolvers::advent_13::resolve,
        resolvers::advent_14::resolve,
        resolvers::advent_15::resolve,
        resolvers::advent_16::resolve,
        resolvers::advent_17::resolve,
        resolvers::advent_18::resolve,
        resolvers::advent_19::resolve,
        resolvers::advent_20::resolve,
        resolvers::advent_21::resolve,
        resolvers::advent_22::resolve,
        resolvers::advent_23::resolve,
        resolvers::advent_24::resolve,
    ];
    let args: Vec<String> = env::args().collect();
    let day: usize = args[1].parse().expect("Not a number!");
    let input = get_file_content(&args[2]);

    if day == 0 {
        println!("day >= 1");
        std::process::exit(-1);
    }
    let now = std::time::SystemTime::now();
    resolvers[day - 1](&input);
    let elapsed = now.elapsed().unwrap();
    println!("Computation done in {} ms (or {} sec)", elapsed.as_millis(), elapsed.as_secs());
}
