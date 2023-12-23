use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Debug)]
struct NumberMap {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32
}

fn get_next_value(current: u32, destination_map: &Vec<NumberMap>) -> u32 {
    for destination in destination_map {
        let range_length: u32;
        if destination.range_length == 0 {
            range_length = 0;
        } else {
            range_length = destination.range_length -1
        }
        if destination.source_range_start <= current && destination.source_range_start + range_length >= current {
            // it matched
            let distance_from_start = current - destination.source_range_start;
            return destination.destination_range_start + distance_from_start;
        }
    }
    return current;
}

pub fn resolve(input: &String) {

    let lines: Vec<&str> = input.lines().collect();
    let seeds_list: Vec<&str> = lines[0].split(":").collect::<Vec<&str>>()[1].split(" ").filter(|seed| seed.len() > 0).collect();
    let mut seeds: Vec<u32> = Vec::new();
    let mut old_seeds:  Vec<u32> = Vec::new();

    if seeds_list.len() % 2 != 0 {
        println!("Seeds must be a pair (len: {})", seeds_list.len());
        return;
    }

    for i in 0..seeds_list.len() / 2 {
        old_seeds.push(seeds_list[i * 2].parse().unwrap());
        old_seeds.push(seeds_list[i * 2 + 1].parse().unwrap());

        let seed_start: u32 = seeds_list[i * 2].parse().unwrap();
        let seed_range: u32 = seeds_list[i * 2 + 1].parse().unwrap();
        for k in seed_start..seed_start + seed_range {
            seeds.push(k);
        }
    }

    println!("Old Seeds (len {})", old_seeds.len());
    println!("New Seeds (len {})", seeds.len());

    let mut processes: HashMap<String, Vec<NumberMap>> = HashMap::new();

    processes.insert("seed_to_soil_map".to_string(), Vec::new());
    processes.insert("soil_to_fertilizer_map".to_string(), Vec::new());
    processes.insert("fertilize_to_water_map".to_string(), Vec::new());
    processes.insert("water_to_light_map".to_string(), Vec::new());
    processes.insert("light_to_temperature_map".to_string(), Vec::new());
    processes.insert("temperature_to_humidity_map".to_string(), Vec::new());
    processes.insert("humidity_to_location_map".to_string(), Vec::new());

    let mut target: String = String::new();

    for line in lines {
        match line {
            "seed-to-soil map:" => target = "seed_to_soil_map".to_string(),
            "soil-to-fertilizer map:" => target = "soil_to_fertilizer_map".to_string(),
            "fertilizer-to-water map:" => target = "fertilize_to_water_map".to_string(),
            "water-to-light map:" => target = "water_to_light_map".to_string(),
            "light-to-temperature map:" => target = "light_to_temperature_map".to_string(),
            "temperature-to-humidity map:" => target = "temperature_to_humidity_map".to_string(),
            "humidity-to-location map:" => target = "humidity_to_location_map".to_string(),
            _ => {
                if target.len() == 0 {
                    continue
                }
                let numbers: Vec<&str> = line.split(" ").filter(|seed| seed.trim().len() > 0).collect();
                if numbers.len() == 3 {
                    let process = processes.get_mut(&target).unwrap();
                    process.push(NumberMap {
                        destination_range_start: numbers[0].parse().unwrap(),
                        source_range_start: numbers[1].parse().unwrap(),
                        range_length: numbers[2].parse().unwrap(),
                    });
                }
            }
        }
    }
    // println!("processes, {:?}", processes);
    // println!("seed-to-soil map: {:?}", processes.get(&"seed_to_soil_map".to_string()));
    // println!("soil-to-fertilizer map: {:?}", processes.get(&"soil_to_fertilizer_map".to_string()));
    // println!("fertilizer-to-water map: {:?}", processes.get(&"fertilize_to_water_map".to_string()));
    // println!("water-to-light map: {:?}", processes.get(&"water_to_light_map".to_string()));
    // println!("light-to-temperature map: {:?}", processes.get(&"light_to_temperature_map".to_string()));
    // println!("temperature-to-humidity map: {:?}", processes.get(&"temperature_to_humidity_map".to_string()));
    // println!("humidity-to-location map: {:?}", processes.get(&"humidity_to_location_map".to_string()));

    let process_order = vec![
        "seed_to_soil_map".to_string(),
        "soil_to_fertilizer_map".to_string(),
        "fertilize_to_water_map".to_string(),
        "water_to_light_map".to_string(),
        "light_to_temperature_map".to_string(),
        "temperature_to_humidity_map".to_string(),
        "humidity_to_location_map".to_string(),
    ];

    let mut min_location: u32 = u32::MAX;
    // Part 1
    for seed in old_seeds {
        let mut current_value: u32 = seed;
        for process_step in &process_order {
            let destination_map = processes.get(process_step).unwrap();
            current_value = get_next_value(current_value, &destination_map);
        }
        if current_value < min_location {
            min_location = current_value;
        }
    }
    println!("Min location for part 1: {}", min_location);
    // Part 2
    let result = seeds.par_iter().map(|seed| -> u32 {
        let mut current_value: u32 = *seed;
        for process_step in &process_order {
            let destination_map = processes.get(process_step).unwrap();
            current_value = get_next_value(current_value, &destination_map);
        }
        return current_value;
    }).collect::<Vec<_>>();
    println!("Min location for part 2: {:?}", result.iter().min().unwrap());
}