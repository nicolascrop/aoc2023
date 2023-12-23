use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

type MapData = Vec<Vec<u8>>;
#[derive(Hash)]
struct Map {
    map_data: MapData
}
fn my_hash<T>(obj: T) -> u64
    where
        T: std::hash::Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug)]
enum Direction {
    WEST,
    EAST,
    NORTH,
    SOUTH
}

fn parse_map_case(ch: char) -> u8 {
    match ch {
        '.' => b'.',
        '#' => b'#',
        'O' => b'O',
        _ => panic!("Wrong map ch {}", ch)
    }
}

fn display_map(map: &Map) {
    for i in 0..map.map_data.len() {
        for j in 0..map.map_data[i].len() {
            print!("{}", map.map_data[i][j] as char);
        }
        print!("\n");
    }
    println!("Hash: {}", my_hash(map));
}

fn move_rock(map: &mut Map, direction: &Direction, i: usize, j: usize) -> bool {
    let c2: (usize, usize) = match direction {
        Direction::NORTH => (i - 1, j),
        Direction::SOUTH => (i + 1, j),
        Direction::EAST => (i, j + 1),
        Direction::WEST => (i, j - 1),
    };
    if map.map_data[i][j] == b'O' {
        if map.map_data[c2.0][c2.1] == b'.' {
            map.map_data[c2.0][c2.1] = b'O';
            map.map_data[i][j] = b'.';
            return true;
        }
    }
    return false;
}

fn compute_map_weight(map: &Map) -> usize {
    let mut sum: usize = 0;
    for i in 0..map.map_data.len() {
        for j in 0..map.map_data[i].len() {
            if map.map_data[i][j] == b'O' {
                sum += map.map_data.len() - i;
            }
        }
    }
    return sum;
}

fn move_map(map: &mut Map, direction: Direction) {
    loop {
        let mut has_moved = false;

        match direction {
            Direction::NORTH => {
                for i in 1..map.map_data.len() {
                    for j in 0..map.map_data[i].len() {
                        if move_rock(map, &direction, i, j) {
                            has_moved = true;
                        }
                    }
                }
            },
            Direction::SOUTH => {
                for i in (0..(map.map_data.len() - 1)).rev() {
                    for j in 0..map.map_data[i].len() {
                        if move_rock(map, &direction, i, j) {
                            has_moved = true;
                        }
                    }
                }
            },
            Direction::WEST => {
                for j in 1..map.map_data[0].len() {
                    for i in 0..map.map_data.len() {
                        if move_rock(map, &direction, i, j) {
                            has_moved = true;
                        }
                    }
                }
            },
            Direction::EAST => {
                for j in (0..map.map_data[0].len() - 1).rev() {
                    for i in 0..map.map_data.len() {
                        if move_rock(map, &direction, i, j) {
                            has_moved = true;
                        }
                    }
                }
            }
        }

        if !has_moved {
            break;
        }
    }

}

pub fn resolve(input: &String) {
    let mut map_data: MapData = Vec::new();
    for line in input.lines() {
        let mut map_line: Vec<u8> = Vec::new();
        for ch in line.chars() {
            map_line.push(parse_map_case(ch));
        }
        map_data.push(map_line);
    }

    let mut map = Map { map_data };

    println!("Map");
    display_map(&map);

    // part1
    // move_map(map, Direction::NORTH);

    let mut max_cycles = 1000000000;
    let mut nb_cycles = 0;
    let mut cycle_found = false;
    let mut hash_found = 0;
    let mut hash_map: HashMap<u64, usize> = HashMap::new();
    let initial_hash = my_hash(&map);
    hash_map.insert(initial_hash, 0);

    loop {
        move_map(&mut map, Direction::NORTH);
        move_map(&mut map, Direction::WEST);
        move_map(&mut map, Direction::SOUTH);
        move_map(&mut map, Direction::EAST);
        nb_cycles += 1;
        let current_hash = my_hash(&map);
        if !cycle_found {
            if hash_map.contains_key(&current_hash) {
                cycle_found = true;
                hash_found = current_hash;
                println!("Found loop at {} cycle content {:?}", nb_cycles, hash_map.get(&current_hash));
                let start_loop_at = hash_map.get(&current_hash).unwrap();
                let loop_size = nb_cycles - start_loop_at;
                let remaining_cycles = (max_cycles - nb_cycles) % loop_size;
                println!("Remaining cycles to match 1000 000 000 cycles {} loop size {}, start_loop_at {}", remaining_cycles, loop_size, start_loop_at);
                max_cycles = remaining_cycles + nb_cycles;
                continue
            }
            hash_map.insert(current_hash, nb_cycles);
        } else {
            if hash_found == current_hash {
                println!("Found res at {} weight: {}", nb_cycles, compute_map_weight(&map));
            }
            if compute_map_weight(&map) == 93736 {
                println!("Found weight {} {}", nb_cycles, max_cycles);
            }
        }
        if nb_cycles == max_cycles {
            break;
        }
    }

    // println!("Map after cycles");
    // display_map(&map);


    println!("Weight: {}", compute_map_weight(&map));
}

