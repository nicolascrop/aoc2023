use std::collections::HashMap;

#[derive(Debug)]
struct Path {
    left: String,
    right: String
}

type CamelMap = HashMap<String, Path>;

fn browse_map(path_chars: &Vec<char>, map: &CamelMap, start: &String) -> String {
    let mut current_position: String = start.to_string();
    for char in path_chars {
        let map_value = map.get(&current_position);
        if !map_value.is_some() {
            panic!("Map value not found {}", current_position);
        }
        let paths_possible = map_value.unwrap();
        if *char == 'L' {
            current_position = paths_possible.left.to_string();
        } else {
            current_position = paths_possible.right.to_string();
        }
    }
    return current_position.to_string();
}

fn browse_map_part_one(path: &String, map: &CamelMap, start: String, end: String) -> usize {
    let mut movement: usize = 0;
    let path_chars = path.chars().collect::<Vec<_>>();

    loop {
        let _end = browse_map(&path_chars, &map, &start.to_string());
        movement += path_chars.len();
        if _end == end {
            return movement;
        }
    }
}

fn browse_map_part_two(path: &String, map: &CamelMap) -> usize {
    let path_chars = path.chars().collect::<Vec<_>>();
    let mut start_nodes: Vec<String> = Vec::new();
    let mut current_positions: Vec<String> = Vec::new();

    for entry in map {
        if entry.0.chars().nth(2).unwrap() == 'A' {
            start_nodes.push(entry.0.to_string());
            current_positions.push(entry.0.to_string());
        }
    }

    println!("Start nodes {:?}", start_nodes);
    println!("Current positions nodes {:?}", current_positions);
    let mut steps = 1;
    for i in 0..start_nodes.len() {
        let mut nb_loop = 0;
        let mut end_found_once = false;
        loop {
            nb_loop += 1;
            current_positions[i] = browse_map(&path_chars, &map, &current_positions[i]);
            if current_positions[i].chars().nth(2).unwrap() == 'Z' {
                println!("End found : {}", current_positions[i]);
                if end_found_once {
                    break;
                } else {
                    end_found_once = true;
                }
            }
        }
        println!("found {} loops for start_node {}", nb_loop, start_nodes[i]);
        steps = steps * (nb_loop / 2);
    }
    return steps * path_chars.len();
}


pub fn resolve(input: &String) {
    let lines = input.lines();
    let mut map: CamelMap = HashMap::new();
    let mut path = String::new();
    let mut doing_path = true;
    for line in lines {
        if line == "" {
            doing_path = false;
            continue;
        }
        if doing_path {
            path = path + &line.to_string();
            continue;
        }

        let parts: Vec<&str> = line.split(" = ").collect();
        let paths: Vec<&str> = parts[1].split(", ").collect();
        let left = &paths[0].to_string()[1..];
        let right = &paths[1].chars().take_while(|&ch|ch != ')').collect::<String>();
        map.insert(parts[0].to_string(), Path {
            left: left.to_string(),
            right: right.to_string()
        });
    }
    println!("Path: {}", path);
    println!("Map {:?}", map);

    println!("Nb move part one: {}", browse_map_part_one(&path, &map, "AAA".to_string(), "ZZZ".to_string()));
    println!("Nb move part two: {}", browse_map_part_two(&path, &map));
}