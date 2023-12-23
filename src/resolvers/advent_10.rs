use std::cmp::min;
use std::collections::HashSet;

type Map = Vec<Vec<char>>;
#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM
}
#[derive(Debug)]
struct Point {
    row: usize,
    col: usize
}

fn print_map(map: &Map) {
    println!("Map:");
    for line in map {
        for char in line {
            print!("{}", char);
        }
        print!("\n");
    }
}



fn get_next_point(map: &Map, current: &Point, previous: &Point, allow_start: bool) -> Point {
    // println!("Current {} {:?}, previous {} {:?}", map[current.row][current.col], current, map[previous.row][previous.col], previous);
    let next_directions: (Direction, Direction) = match map[current.row][current.col] {
        '|' => (
            Direction::TOP,
            Direction::BOTTOM
        ),
        '-' => (
            Direction::LEFT,
            Direction::RIGHT
        ),
        'F' => (
            Direction::RIGHT,
            Direction::BOTTOM
        ),
        'L' => (
            Direction::TOP,
            Direction::RIGHT
        ),
        'J' => (
            Direction::TOP,
            Direction::LEFT
        ),
        '7' => (
            Direction::LEFT,
            Direction::BOTTOM
        ),
        _ => {
            panic!("Next point invalid");
        }
    };
    for next_direction in [next_directions.0, next_directions.1].iter() {
        if path_is_valid(map, current, *next_direction) {
            let next_point = get_next_point_from_direction(current, *next_direction);
            if !allow_start && map[next_point.row][next_point.col] == 'S' {
                continue;
            }
            if !(next_point.row == previous.row && next_point.col == previous.col) {
                return next_point;
            }
        }
    }
    panic!("No direction found current: {:?} next directions ({:?}, {:?}). Previous {:?}", current, next_directions.0, next_directions.1, previous);
}

fn get_next_point_from_direction(current: &Point, direction: Direction) -> Point {
    match direction {
        Direction::LEFT => Point { row: current.row, col: current.col - 1 },
        Direction::RIGHT => Point { row: current.row, col: current.col + 1 },
        Direction::BOTTOM => Point { row: current.row + 1, col: current.col },
        Direction::TOP => Point { row: current.row - 1, col: current.col }
    }
}

fn path_is_valid(map: &Map, start_point: &Point, direction: Direction) -> bool {
    if direction == Direction::LEFT {
        if start_point.col == 0 {
            return false;
        }
        if !"SFL-".contains(map[start_point.row][start_point.col - 1]) {
            return false;
        }
    }
    if direction == Direction::RIGHT {
        if start_point.col + 1 >= map[start_point.row].len() {
            return false;
        }
        if !"S7J-".contains(map[start_point.row][start_point.col + 1]) {
            return false;
        }
    }
    if direction == Direction::TOP {
        if start_point.row == 0 {
            return false;
        }
        if !"SF|7".contains(map[start_point.row - 1][start_point.col]) {
            return false;
        }
    }
    if direction == Direction::BOTTOM {
        if start_point.row + 1 >= map.len() {
            return false;
        }
        if !"S|JL".contains(map[start_point.row + 1][start_point.col]) {
            return false;
        }
    }
    return true;
}

fn get_utf8_char(map_char: char) -> char {
    match map_char {
        'F' => '┌',
        '7' => '┐',
        'L' => '└',
        'J' => '┘',
        _ => map_char
    }
}


fn is_inside_loop(map: &Map, row_index: usize, col_index: usize, set: &HashSet<String>) -> bool {
    let mut down: u32 = 0;
    let mut up: u32 = 0;
    for i in col_index+1..map[row_index].len() {
        if set.contains(&format!("{}_{}", row_index, i)) {
            if "JL|".contains(map[row_index][i]) {
                up += 1;
            }
            if "|7F".contains(map[row_index][i]) {
                down += 1;
            }
        }
    }
    return min(down, up) % 2 != 0;
}

fn get_max_distance_from_start(map: &Map, start: &Point) -> usize {
    let mut path_possible: Vec<Point> = Vec::new();
    if path_is_valid(map, start, Direction::TOP) {
        path_possible.push(get_next_point_from_direction(start, Direction::TOP));
    }
    if path_is_valid(map, start, Direction::BOTTOM) {
        path_possible.push(get_next_point_from_direction(start, Direction::BOTTOM));
    }
    if path_is_valid(map, start, Direction::LEFT) {
        path_possible.push(get_next_point_from_direction(start, Direction::LEFT));
    }
    if path_is_valid(map, start, Direction::RIGHT) {
        path_possible.push(get_next_point_from_direction(start, Direction::RIGHT));
    }
    if path_possible.len() != 2 {
        panic!("Found {} path possibles", path_possible.len());
    }

    let mut current: Point = Point {row: path_possible[0].row, col: path_possible[0].col};
    let mut previous: Point = Point {row: start.row, col: start.col};
    println!("at start Current {:?} Previous {:?}", current, previous);
    let mut loop_parts: HashSet<String> = HashSet::new();
    loop_parts.insert(format!("{}_{}", start.row, start.col));
    while map[current.row][current.col] != 'S' {
        let tmp_previous = Point {row: current.row, col: current.col};
        loop_parts.insert(format!("{}_{}", current.row, current.col));
        current = get_next_point(map, &current, &previous, loop_parts.len() > 2);
        previous = tmp_previous;
    }
    println!("Full loop length: {}", loop_parts.len());


    println!("Map:");
    for row_index in 0..map.len() {
        for col_index in 0..map[row_index].len() {
            if loop_parts.contains(&format!("{}_{}", row_index, col_index)) {
                print!("\x1b[91m{}\x1b[0m", map[row_index][col_index]);
            } else {
                print!("{}", map[row_index][col_index]);
            }
        }
        print!("\n");
    }

    println!("Map:");
    let mut nb_part_of_loop = 0;
    for row_index in 0..map.len() {
        for col_index in 0..map[row_index].len() {
            if !loop_parts.contains(&format!("{}_{}", row_index, col_index)) {
                let enclosed = is_inside_loop(map, row_index, col_index, &loop_parts);
                if enclosed {
                    print!("\x1b[92m{}\x1b[0m", get_utf8_char(map[row_index][col_index]));
                    nb_part_of_loop += 1;
                } else {
                    print!("\x1b[93m{}\x1b[0m", get_utf8_char(map[row_index][col_index]));
                }
            } else {
                print!("\x1b[95m{}\x1b[0m", get_utf8_char(map[row_index][col_index]));
            }
        }
        print!("\n");
    }

    println!("Found {} inside loop", nb_part_of_loop);

    return loop_parts.len() / 2;
}

pub fn resolve(input: &String) {
    let mut map: Map = Vec::new();
    let mut start: Option<Point> = None;
    let mut i = 0;
    for line in input.lines() {
        let mut _line: Vec<char> = Vec::new();
        let mut j = 0;
        for _char in line.chars() {
            _line.push(_char);
            if _char == 'S' {
                start = Some(Point {
                    col: j,
                    row: i
                })
            }
            j += 1;
        }
        map.push(_line);
        i += 1;
    }

    print_map(&map);
    if !start.is_some() {
        panic!("No starting point found");
    }
    println!("Max distance: {}", get_max_distance_from_start(&map, &start.unwrap()));
}