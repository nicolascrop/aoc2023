use std::collections::{HashMap};

type Map = Vec<Vec<u32>>;

#[derive(Debug, Clone)]
struct Position {
    row_index: usize,
    col_index: usize,
    previous_cell_index: Option<(usize, usize)>,
    path: Vec<String>,
    previous_movements_in_same_direction: u8,
    total_heat: usize
}

const MIN_MOVE_BEFORE_TURN: u8 = 4;
const MAX_MOVE_BEFORE_TURN: u8 = 10;

fn get_next_positions(map: &Map, current: &Position) -> Vec<Position> {
    let mut results: Vec<Position> = Vec::new();

    if current.previous_cell_index.is_some() {
        // println!("current {:?}", current);
        let position_heat = map[current.row_index][current.col_index] as usize;
        let previous_cell = current.previous_cell_index.unwrap();
        let mut new_path: Vec<String> = current.path[0..].to_vec();
        new_path.push(format!("{}_{}", current.row_index, current.col_index));
        let mut next_positions: Vec<(usize, usize, bool)> = Vec::new();
        if current.row_index < map.len() - 1 {
            next_positions.push((current.row_index + 1, current.col_index, previous_cell.1 == current.col_index));
        }
        if current.row_index > 0 {
            next_positions.push((current.row_index - 1, current.col_index, previous_cell.1 == current.col_index));
        }
        if current.col_index < map[0].len() - 1 {
            next_positions.push((current.row_index, current.col_index + 1, previous_cell.0 == current.row_index));
        }
        if current.col_index > 0 {
            next_positions.push((current.row_index, current.col_index - 1, previous_cell.0 == current.row_index));
        }

        for next_position in next_positions {
            let previous_movements_in_same_direction = if next_position.2 { current.previous_movements_in_same_direction + 1 } else { 0 };
            if current.path.contains(&format!("{}_{}", next_position.0, next_position.1)) {
                continue;
            }

            if current.previous_movements_in_same_direction + 1 < MIN_MOVE_BEFORE_TURN {
                if previous_movements_in_same_direction != current.previous_movements_in_same_direction + 1 {
                    continue;
                }
            } else {
                if previous_movements_in_same_direction >= MAX_MOVE_BEFORE_TURN {
                    continue;
                }
            }
            results.push(Position {
                row_index: next_position.0,
                col_index: next_position.1,
                previous_cell_index: Some((current.row_index, current.col_index)),
                previous_movements_in_same_direction,
                total_heat: current.total_heat + position_heat,
                path: new_path.clone()
            });
        }
    } else {
        results.push(Position {
            row_index: current.row_index + 1,
            col_index: current.col_index,
            previous_cell_index: Some((current.row_index, current.col_index)),
            previous_movements_in_same_direction: 1,
            total_heat: 0,
            path: vec![String::from("0_0")]
        });
        results.push(Position {
            row_index: current.row_index,
            col_index: current.col_index + 1,
            previous_cell_index: Some((current.row_index, current.col_index)),
            previous_movements_in_same_direction: 1,
            total_heat: 0,
            path: vec![String::from("0_0")]
        })
    }
    return results;
}

fn get_id_from_position(position: &Position) -> String {
    return format!("{}_{}_{:?}_{}", position.row_index, position.col_index, position.previous_cell_index, position.previous_movements_in_same_direction);
}

fn compute_heat_loss(map: &Map, start: &Position) -> usize {
    let mut heat_loss: usize = usize::MAX;
    let mut best_path: Option<Position> = None;
    let mut best_score: HashMap<String, usize> = HashMap::new();
    let mut currents = get_next_positions(map, start);
    let last_heat_value = map[map.len() - 1][map[0].len() - 1] as usize;
    loop {
        // println!("\nIn Loop {}:", nb_loop);
        // for current in &currents {
        //     println!("Current: {:?}", current);
        // }
        if currents.len() == 0 {
            break;
        }
        let mut next_currents = Vec::new();
        for i in 0..currents.len() {
            if currents[i].row_index == map.len() - 1 && currents[i].col_index == map[0].len() - 1 {
                if currents[i].total_heat + last_heat_value < heat_loss &&
                    currents[i].previous_movements_in_same_direction + 1 >= MIN_MOVE_BEFORE_TURN &&
                    currents[i].previous_movements_in_same_direction + 1 <= MAX_MOVE_BEFORE_TURN {
                    heat_loss = currents[i].total_heat + last_heat_value;
                    best_path = Some(currents[i].clone());
                }
                continue;
            }
            if currents[i].total_heat + last_heat_value >= heat_loss {
                continue;
            }

            let next_positions = get_next_positions(map, &currents[i]);
            for next_position in next_positions {
                let score_id = get_id_from_position(&next_position);
                let found = best_score.get(&score_id);
                // println!("Score id {} found {:?} current_score {}", score_id, found, next_position.total_heat);
                if found.is_some() {
                    let best_score_value = found.unwrap();
                    if best_score_value > &next_position.total_heat {
                        best_score.insert(score_id, *(&next_position.total_heat));
                        next_currents.push(next_position.clone());
                    }
                } else {
                    best_score.insert(score_id, *(&next_position.total_heat));
                    next_currents.push(next_position.clone());
                }
            }
        }
        currents.clear();
        for next_current in next_currents {
            if next_current.total_heat <= *best_score.get(&get_id_from_position(&next_current)).unwrap() {
                currents.push(next_current);
            }
        }
    }
    println!("Best path {:?}", best_path);
    return heat_loss;
}


fn display_map(map: &Map) {
    println!("Map:");
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]);
        }
        print!("\n");
    }
}

pub fn resolve(input: &String) {
    let mut map: Map = Vec::new();
    for line in input.lines() {
        let mut map_line: Vec<u32> = Vec::new();
        for char in line.chars() {
            map_line.push(char.to_digit(10).unwrap());
        }
        map.push(map_line);
    }

    display_map(&map);
    println!("Min loss: {}", compute_heat_loss(&map, &Position {
        row_index: 0,
        col_index: 0,
        previous_cell_index: None,
        previous_movements_in_same_direction: 0,
        total_heat: 0,
        path: Vec::new()
    }))
}