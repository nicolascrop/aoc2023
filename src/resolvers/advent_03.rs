const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn is_symbol(str: char) -> bool {
    return !DIGITS.contains(&str) && str != '.';
}
fn has_symbol(map: &Vec<Vec<char>>, row_index: usize, start: i32, end: usize) -> bool {
    // Check above
    let range_start: usize;
    if start < 0 {
       range_start = 0;
    } else {
        range_start = start as usize;
    }
    if row_index > 0 {
        for i in range_start..=end {
            if end > map[row_index - 1].len() - 1 {
                continue;
            }
            if is_symbol(map[row_index - 1][i]) {
                return true;
            }
        }
    }
    // Check left
    if start >= 0 {
        if is_symbol(map[row_index][start as usize]) {
            return true;
        }
    }

    // Check right
    if end < map[row_index].len() - 1 {
        if is_symbol(map[row_index][end]) {
            return true;
        }
    }

    // Check below
    if row_index < map.len() - 1 {
        for i in range_start..=end {
            if end > map[row_index + 1].len() - 1 {
                continue;
            }
            if is_symbol(map[row_index + 1][i]) {
                return true;
            }
        }
    }
    return false;
}

#[derive(Debug)]
struct PartNumber {
    value: u32,
    row: usize,
    start_x: usize,
    end_x: usize,
}

fn get_gear_part_number_result(part_numbers: &Vec<PartNumber>, gear_row: usize, gear_col: usize) -> Option<u32> {
    let mut part_numbers_around_gear: Vec<&PartNumber> = Vec::new();
    for part_number in part_numbers {
        println!("Testing part number {} start_x {} end_x {} row {}", part_number.value, part_number.start_x, part_number.end_x, part_number.row);
        if gear_row > 0 && part_number.row >= gear_row - 1 && part_number.row <= gear_row + 1 {
            let start: usize;
            if part_number.start_x > 0 {
                start = part_number.start_x - 1;
            } else {
                start = 0;
            }
            if gear_col > 0 && start <= gear_col && part_number.end_x >= gear_col {
                println!("Part Number {} is valid", part_number.value);
                part_numbers_around_gear.push(part_number);
            }
        }
    }
    if part_numbers_around_gear.len() == 2 {
        return Some(part_numbers_around_gear[0].value * part_numbers_around_gear[1].value);
    }
    return None;
}

pub fn resolve(input: &String) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    for row in input.lines() {
        map.push(row.chars().collect::<Vec<_>>())
    }
    let mut result = 0;
    let mut row_index = 0;
    for row in &map {
        let mut current_number = String::new();
        let mut col_index = 0;
        for col in row {
            if DIGITS.contains(&col) {
                current_number.push(col.clone());
            }
            if col_index == row.len() - 1 || !DIGITS.contains(&col) {
                if current_number.len() > 0 {
                    let current_number_value: u32 = current_number.parse().unwrap();
                    println!("col index {}, current nb {}", col_index, current_number);
                    let col_start: i32 = i32::from((col_index - current_number.len()) as u16) - 1;
                    let col_end: usize = col_index;
                    if has_symbol(&map, row_index, col_start, col_end) {
                        result = result + current_number_value;
                        println!("has symbol, number: {}, result {}", current_number_value, result);
                    }
                    part_numbers.push(PartNumber {
                        value: current_number_value,
                        row: row_index,
                        start_x: (col_index - current_number.len()),
                        end_x: col_index
                    });
                }
                current_number.clear();
            }
            col_index = col_index + 1;
        }
        row_index = row_index + 1;
    }
    let mut result_sum_gears = 0;
    // println!("PartNumbers {:?}", part_numbers);
    row_index = 0;
    for row in &map {
        let mut col_index = 0;
        for _ in row {
            if map[row_index][col_index] == '*' {
                println!("Found gear at x {} y {}", row_index, col_index);
                let result = get_gear_part_number_result(&part_numbers, row_index, col_index);
                if result.is_some() {
                    println!("Gear is valid {}", result.unwrap());
                    result_sum_gears = result_sum_gears + result.unwrap();
                }
            }
            col_index = col_index + 1;
        }
        row_index = row_index + 1;
    }
    println!("Resul sum gears {}", result_sum_gears);
}