use std::cmp::min;
use colored::Colorize;

#[derive(PartialEq, Debug)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    DOWN
}

#[derive(Clone, Debug)]
struct Point {
    color: String
}

type Map = Vec<Vec<Option<Point>>>;

fn get_direction_from_part(part: &str) -> Direction {
    match part {
        "R" => Direction::RIGHT,
        "U" => Direction::UP,
        "D" => Direction::DOWN,
        "L" => Direction::LEFT,
        _ => panic!("Wrong direction {}", part)
    }
}

fn display_map(map: &Map) {
    println!("Map:");
    for i in 0..map.len() {
        for j in 0..map[i].len() {

            if map[i][j].is_some() {
                print!("{}", "#".red().bold())
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

pub fn resolve(input: &String) {
    let mut map: Map = vec![vec![]];
    let mut nb_columns = 0;
    let mut current_point: (usize, usize) = (0, 0);

    let mut is_init = true;
    let mut polygon_vertices: Vec<(usize, usize)> = vec![(0, 0)];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').filter(|x| x.len() > 0).collect();
        let direction = get_direction_from_part(parts[0]);
        let amount: usize = parts[1].parse().unwrap();
        let color: &str = parts[2];
        let last_vertex = polygon_vertices[polygon_vertices.len() - 1];

        if direction == Direction::UP {
            polygon_vertices.push((last_vertex.0 + amount, last_vertex.1));
            for _ in 0..amount {
                if current_point.0 == 0 {
                    map.insert(0, vec![None; nb_columns]);
                } else {
                    current_point.0 -= 1;
                }
                println!("{:?} Current point {:?} nb_columns {} map len {}", direction, current_point, nb_columns, map.len());
                map[current_point.0][current_point.1] = Some(Point { color: String::from(color) });
            }
        }
        if direction == Direction::DOWN {
            polygon_vertices.push((last_vertex.0 - amount, last_vertex.1));
            for _ in 0..amount {
                if current_point.0 == map.len() - 1 {
                    map.push(vec![None; nb_columns]);
                }
                println!("{:?} Current point {:?} nb_columns {} map len {}", direction, current_point, nb_columns, map.len());
                if !is_init {
                    current_point.0 += 1;
                }
                map[current_point.0][current_point.1] = Some(Point { color: String::from(color) });
            }
        }
        if direction == Direction::LEFT {
            polygon_vertices.push((last_vertex.0, last_vertex.1 - amount));
            for _ in 0..amount {
                if current_point.1 == 0 {
                    for row_index in 0..map.len() {
                        map[row_index].insert(0, None);
                    }
                    nb_columns += 1;
                } else {
                    current_point.1 -= 1;
                }
                println!("{:?} Current point {:?} nb_columns {} map len {}", direction, current_point, nb_columns, map.len());
                map[current_point.0][current_point.1] = Some(Point { color: String::from(color) });
            }
        }
        if direction == Direction::RIGHT {
            polygon_vertices.push((last_vertex.0, last_vertex.1 + amount));
            for _ in 0..amount {
                println!("HH {} {} {}", current_point.1, nb_columns, map.len());
                if is_init || current_point.1 + 1 == nb_columns {
                    for row_index in 0..map.len() {
                        map[row_index].push(None);
                    }
                    nb_columns += 1;
                }
                if !is_init {
                    current_point.1 += 1;
                } else {
                    is_init = false;
                }
                println!("{:?} Current point {:?} nb_columns {} map len {}", direction, current_point, nb_columns, map.len());
                map[current_point.0][current_point.1] = Some(Point { color: String::from(color) });
            }
        }
    }
    display_map(&map);
}