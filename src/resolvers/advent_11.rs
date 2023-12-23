use colored::*;
use rayon::prelude::*;

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

struct Galaxy {
    map: Vec<Vec<char>>,
    galaxy_points: Vec<Point>,
    expansion_size: usize,
    expanded_rows_index: Vec<usize>,
    expanded_cols_index: Vec<usize>
}

impl Galaxy {
    pub fn new(input: &String, expansion_size: usize) -> Self {
        let mut col_expanded_index: Vec<usize> = Vec::new();
        let mut row_expanded_index: Vec<usize> = Vec::new();
        let mut map: Vec<Vec<char>> = Vec::new();

        let lines: Vec<&str> = input.lines().collect();
        if lines.len() == 0 {
            panic!("Lines empty");
        }

        for j in 0..lines[0].len() {
            let mut galaxy_found = false;
            for i in 0..lines.len() {
                let chars: Vec<char> = lines[i].chars().collect();
                if chars[j] == '#' {
                    galaxy_found = true;
                }
            }
            if !galaxy_found {
                col_expanded_index.push(j);
            }
        }

        for i in 0..lines.len() {
            let mut map_line: Vec<char> = Vec::new();
            for char in lines[i].chars() {
                map_line.push(char);
            }
            if !map_line.contains(&'#') {
                row_expanded_index.push(i);
            }
            map.push(map_line);
        }

        let mut galaxy_points: Vec<Point> = Vec::new();
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == '#' {
                    galaxy_points.push(Point {
                        row: i,
                        col: j
                    });
                }
            }
        }

        Self {
            map,
            galaxy_points,
            expanded_rows_index: row_expanded_index,
            expanded_cols_index: col_expanded_index,
            expansion_size
        }
    }
    pub fn display_map(&self) {
        println!("Map:");
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                let mut galaxy_point_found = false;
                for galaxy_point in &self.galaxy_points {
                    if galaxy_point.row == i && galaxy_point.col == j {
                        galaxy_point_found = true;
                        break;
                    }
                }

                if galaxy_point_found {
                    print!("{}", "#".cyan().bold());
                } else {
                    print!("{}", self.map[i][j]);
                }
            }
            print!("\n");
        }

        println!("Expanded rows index {:?}", self.expanded_rows_index);
        println!("Expanded cols index {:?}", self.expanded_cols_index);
    }

    pub fn get_distance_between_2_galaxy_points(&self, point_a: &Point, point_b: &Point) -> usize {
        let search_galaxy = SearchGalaxy::new(self);
        return search_galaxy.smart_search(point_a, point_b);
    }
}

struct SearchGalaxy<'a> {
    galaxy: &'a Galaxy
}

impl<'a> SearchGalaxy<'a> {
    pub fn new(galaxy: &'a Galaxy) -> SearchGalaxy<'a> {
        Self {
            galaxy
        }
    }

    pub fn smart_search(&self, start: &Point, target: &Point) -> usize {
        let mut distance = 0;

        // rows
        let min_row_index = std::cmp::min(start.row, target.row);
        let max_row_index = std::cmp::max(start.row, target.row);
        let mut nb_expansion_row_between = 0;
        for expansion_row_index in &self.galaxy.expanded_rows_index {
            if min_row_index < *expansion_row_index && max_row_index > *expansion_row_index {
                nb_expansion_row_between += 1;
            }
        }
        distance += max_row_index - min_row_index + (nb_expansion_row_between * self.galaxy.expansion_size);

        // cols
        let min_col_index = std::cmp::min(start.col, target.col);
        let max_col_index = std::cmp::max(start.col, target.col);
        let mut nb_expansion_col_between = 0;
        for expansion_col_index in &self.galaxy.expanded_cols_index {
            if min_col_index < *expansion_col_index && max_col_index > *expansion_col_index {
                nb_expansion_col_between += 1;
            }
        }
        distance += max_col_index - min_col_index + (nb_expansion_col_between * self.galaxy.expansion_size);

        return distance;
    }

    // pub fn search(&self, start: &Point, target: &Point) -> usize {
    //     let mut shortest_path = usize::MAX;
//
    //     let mut explored_list: HashSet<String> = HashSet::new();
    //     let mut to_explore: Vec<(Point, usize)> = Vec::new();
    //     to_explore.push((Point {row: start.row, col: start.col}, 0));
    //     // println!("Target: {:?}", target);
    //     let mut i = 0;
    //     while to_explore.len() > 0 {
    //         let current = &to_explore.remove(0);
    //         let current_point = Point { row: current.0.row, col: current.0.col };
    //         let current_path_size = current.1;
    //         if explored_list.contains(&format!("{}_{}", current_point.row, current_point.col)) {
    //             continue;
    //         }
    //         explored_list.insert(format!("{}_{}", current_point.row, current_point.col));
    //         if &current_point == target {
    //             if current_path_size < shortest_path {
    //                 shortest_path = current_path_size;
    //             }
    //             continue;
    //         }
    //         if current_path_size == shortest_path {
    //             continue;
    //         }
//
    //         let mut next_points: Vec<Point> = Vec::new();
    //         if current_point.row > 0 {
    //             next_points.push(Point { row: current_point.row - 1, col: current_point.col });
    //         }
    //         if current_point.row < self.galaxy.map.len() - 1 {
    //             next_points.push(Point { row: current_point.row + 1, col: current_point.col });
    //         }
    //         if current_point.col < self.galaxy.map[0].len() - 1 {
    //             next_points.push(Point { row: current_point.row, col: current_point.col + 1 });
    //         }
    //         if current_point.col > 0 {
    //             next_points.push(Point { row: current_point.row, col: current_point.col - 1});
    //         }
    //         for next_point in next_points {
    //             if !explored_list.contains(&format!("{}_{}", next_point.row, next_point.col)) {
    //                 to_explore.push((Point { row: next_point.row, col: next_point.col }, current_path_size + 1));
    //             }
    //         }
    //         // println!("To explore after:");
    //         // for el in &to_explore {
    //         //     println!("{:?}", el);
    //         // }
    //         // i += 1;
    //         // if i == 100 {
    //         //     // break;
    //         // }
    //     }
    //     return shortest_path;
    // }

}

// part 1 result: 9795148
pub fn resolve(input: &String) {
    let galaxy = Galaxy::new(&input, 1000000 - 1);
    println!("Map built");
    galaxy.display_map();
    let mut galaxy_points_pairs: Vec<(&Point, &Point)> = Vec::new();

    for i in 0..galaxy.galaxy_points.len() - 1 {
        for j in (i + 1)..galaxy.galaxy_points.len() {
            galaxy_points_pairs.push((&galaxy.galaxy_points[i], &galaxy.galaxy_points[j]));
        }
    }

    println!("Nb pairs: {}", galaxy_points_pairs.len());
    let mut result = 0;
    let parallel_results: Vec<usize> = galaxy_points_pairs.par_iter().map(|galaxy_points_pair| {
        return galaxy.get_distance_between_2_galaxy_points(galaxy_points_pair.0, galaxy_points_pair.1);
    }).collect();
    for distance in parallel_results {
        result += distance;
    }
    // for galaxy_points_pair in &galaxy_points_pairs {
    //     let distance = galaxy.get_distance_between_2_galaxy_points(galaxy_points_pair.0, galaxy_points_pair.1);
    //     // println!(
    //     //     "(r: {}, c: {}) - (r: {}, c: {}) Distance: {}",
    //     //     galaxy_points_pair.0.row,
    //     //     galaxy_points_pair.0.col,
    //     //     galaxy_points_pair.1.row,
    //     //     galaxy_points_pair.1.col,
    //     //     distance
    //     // );
    //     result += distance;
    // }

    println!("Result: {}", result);
}