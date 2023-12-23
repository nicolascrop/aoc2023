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
    let mut sum_in = 0;
    for i in 0..map.len() {
        let mut is_in = false;
        for j in 0..map[i].len() {
            let mut next_is_in = is_in;
            if map[i][j].is_some() && (j == map[i].len() - 1 || !map[i][j + 1].is_some()) {

                // No more # at right
                let mut sum_rest_down = 0;
                let mut sum_rest_up = 0;
                let mut sum_rest_both = 0;
                let mut previous_dir = "up";
                for k in j+1..map[i].len() {
                    if map[i][k].is_some() && (k == map[i].len() - 1 || !map[i][k + 1].is_some()) {
                        if k == map[i].len() - 1 {
                            sum_rest_both += 1;
                            next_is_in = !next_is_in;
                        } else {
                            if i > 0 && map[i - 1][k].is_some() {
                                if i < map.len() - 1 && map[i + 1][k].is_some() {
                                    sum_rest_both += 1;
                                    next_is_in = !next_is_in;
                                } else {
                                    sum_rest_up += 1;
                                    if !next_is_in {
                                        next_is_in = true;
                                    }
                                }
                            } else {
                                sum_rest_down += 1;
                                if next_is_in {
                                    next_is_in = false;
                                }
                            }
                        }
                    }
                }
                if is_in {
                    is_in = next_is_in == false
                } else {
                    is_in = next_is_in == true
                }
                // println!("{} {} both {} up {} down {} (next {})", i, j, sum_rest_both, sum_rest_up, sum_rest_down, next_is_in);
                // is_in = next_is_in == true;
                // is_in = min(sum_rest_down + sum_rest_both, sum_rest_up + sum_rest_both) % 2 != 0;
            }

            if map[i][j].is_some() || is_in {
                sum_in += 1;
            }
            if map[i][j].is_some() {
                if next_is_in {
                    print!("{}", "#".cyan().bold())
                } else {
                    print!("{}", "#".red().bold())
                }
            } else {
                if is_in {
                    print!("{}", ".".green().bold());
                } else {
                    print!(".");
                }
            }
        }
        print!("\n");
    }
    println!("Sum in {}", sum_in);
}

pub fn resolve(input: &String) {
    let mut map: Map = vec![vec![]];
    let mut nb_columns = 0;
    let mut current_point: (usize, usize) = (0, 0);

    let mut is_init = true;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').filter(|x| x.len() > 0).collect();
        let direction = get_direction_from_part(parts[0]);
        let amount: usize = parts[1].parse().unwrap();
        let color: &str = parts[2];

        if direction == Direction::UP {
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