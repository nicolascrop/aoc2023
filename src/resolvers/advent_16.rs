use std::collections::{HashSet};
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    TOP,
    LEFT,
    RIGHT,
    BOTTOM
}

#[derive(Debug)]
struct Beam {
    row_index: usize,
    col_index: usize,
    direction: Direction
}

type Map = Vec<Vec<u8>>;
fn get_tile_from_char(char: char) -> u8 {
    match char {
        '.' => b'.',
        '|' => b'|',
        '-' => b'-',
        '/' => b'/',
        '\\' => b'\\',
        _ => panic!("Wrong char {}", char)
    }
}

fn display_map(map: &Map) {
    println!("Map:");
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j] as char);
        }
        print!("\n");
    }
}

// fn display_map_with_beams(map: &Map, _beams: &Vec<Beam>, tile_browsed: &HashMap<String, Vec<Direction>>) {
//     for i in 0..map.len() {
//         for j in 0..map[i].len() {
//             if tile_browsed.contains_key(&format!("{}_{}", i, j)) {
//                 print!("{}", "#".red().bold());
//             } else {
//                 print!("{}", map[i][j] as char);
//             }
//
//         }
//         print!("\n");
//     }
// }

fn get_next_direction(tile: u8, previous_direction: &Direction) -> (Direction, Option<Direction>) {
    match previous_direction {
        Direction::RIGHT => {
            match tile {
                b'\\' => (Direction::BOTTOM, None),
                b'/' => (Direction::TOP, None),
                b'|' => (Direction::TOP, Some(Direction::BOTTOM)),
                _ => (Direction::RIGHT, None)
            }
        },
        Direction::LEFT => {
            match tile {
                b'\\' => (Direction::TOP, None),
                b'/' => (Direction::BOTTOM, None),
                b'|' => (Direction::TOP, Some(Direction::BOTTOM)),
                _ => (Direction::LEFT, None)
            }
        },
        Direction::TOP => {
            match tile {
                b'\\' => (Direction::LEFT, None),
                b'/' => (Direction::RIGHT, None),
                b'-' => (Direction::LEFT, Some(Direction::RIGHT)),
                _ => (Direction::TOP, None)
            }
        },
        Direction::BOTTOM => {
            match tile {
                b'\\' => (Direction::RIGHT, None),
                b'/' => (Direction::LEFT, None),
                b'-' => (Direction::LEFT, Some(Direction::RIGHT)),
                _ => (Direction::BOTTOM, None)
            }
        }
    }
}

fn energize_map(map: &Map, start_beam: &Beam) -> usize {
    let mut beams: Vec<Beam> = Vec::new();
    let mut energize_tiles_with_direction_set: HashSet<String> = HashSet::new();
    let mut energize_tiles: HashSet<String> = HashSet::new();
    let right_limit = map.len();
    let bottom_limit = map[0].len();

    let next_directions = get_next_direction(map[start_beam.row_index][start_beam.col_index], &start_beam.direction);
    beams.push(Beam {row_index: start_beam.row_index, col_index: start_beam.col_index, direction: next_directions.0});
    if next_directions.1.is_some() {
        beams.push(Beam { row_index: start_beam.row_index, col_index: start_beam.col_index, direction: next_directions.1.unwrap() });
    }
    loop {
        if beams.len() == 0 {
            break;
        }
        let mut new_beams: Vec<Beam> = Vec::new();
        for i in 0..beams.len() {
            let beam = &beams[i];
            // println!("Beam {} (row: {} col {} dir {:?})", i, beam.row_index, beam.col_index, beam.direction);
            let id = format!("{}_{}", beam.row_index, beam.col_index);
            let id_direction = format!("{}_{}_{:?}", beam.row_index, beam.col_index, beam.direction);
            if energize_tiles_with_direction_set.contains(&id_direction) {
                continue;
            }
            energize_tiles_with_direction_set.insert(id_direction);
            energize_tiles.insert(id);
            let next_as_option: Option<(usize, usize)> = match beam.direction {
                Direction::RIGHT => {
                    if beam.col_index == right_limit - 1 {
                       Option::None
                    } else {
                        Some((beam.row_index, beam.col_index + 1))
                    }
                },
                Direction::LEFT => {
                    if beam.col_index == 0 {
                        None
                    } else {
                        Some((beam.row_index, beam.col_index - 1))
                    }
                },
                Direction::TOP => {
                    if beam.row_index == 0 {
                        None
                    } else {
                        Some((beam.row_index - 1, beam.col_index))
                    }
                },
                Direction::BOTTOM => {
                    if beam.row_index == bottom_limit - 1 {
                        None
                    } else {
                        Some((beam.row_index + 1, beam.col_index))
                    }
                }
            };
            if !next_as_option.is_some() {
                continue;
            }
            let next = next_as_option.unwrap();
            let next_directions = get_next_direction(map[next.0][next.1], &beam.direction);
            new_beams.push(Beam {row_index: next.0, col_index: next.1, direction: next_directions.0});
            if next_directions.1.is_some() {
                new_beams.push(Beam { row_index: next.0, col_index: next.1, direction: next_directions.1.unwrap() });
            }
        }
        beams = new_beams;
        // clearscreen::clear();
        // display_map_with_beams(map, &beams, &energize_tiles);
        // std::thread::sleep(std::time::Duration::from_millis(30));
    }

    return energize_tiles.len();
}

pub fn resolve(input: &String) {
    let mut map: Map = Vec::new();
    for line in input.lines() {
        let mut map_line: Vec<u8> = Vec::new();
        for ch in line.chars() {
            map_line.push(get_tile_from_char(ch));
        }
        map.push(map_line);
    }

    display_map(&map);

    println!("Energize tiles part 1: {}", energize_map(&map, &Beam { row_index: 0, col_index: 0, direction: Direction::RIGHT }));

    // Part 2
    let mut start_beams: Vec<Beam> = Vec::new();
    for i in 0..map.len() {
        start_beams.push(Beam { row_index: i, col_index: 0, direction: Direction::RIGHT });
        start_beams.push(Beam { row_index: i, col_index: map[0].len() - 1, direction: Direction::LEFT });
    }
    for j in 0..map[0].len() {
        start_beams.push(Beam { row_index: 0, col_index: j, direction: Direction::BOTTOM });
        start_beams.push(Beam { row_index: map.len() - 1, col_index: j, direction: Direction::TOP });
    }
    let result = start_beams.par_iter().map(|beam| -> usize {
        let beam_res = energize_map(&map, &beam);
        // println!("Result for beam {:?} -> {}", beam, beam_res);
        return beam_res;
    }).max();
    println!("Max energized tiles {:?}", result);

}