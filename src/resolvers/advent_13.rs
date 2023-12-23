use colored::Colorize;

const MAX_SMUDGES: usize = 1;

#[derive(Debug)]
struct Mirror {
    horizontal_patterns: Vec<Vec<char>>,
    vertical_patterns: Vec<Vec<char>>
}

impl Mirror {
    pub fn new(lines: &Vec<&str>) -> Self {
        let mut horizontal_patterns: Vec<Vec<char>> = Vec::new();
        let mut vertical_patterns: Vec<Vec<char>> = Vec::new();

        for line in lines {
            horizontal_patterns.push(line.chars().collect());
            for i in 0..line.len() {
                if horizontal_patterns.len() == 1 {
                    // First line
                    vertical_patterns.push(Vec::new());
                }
                vertical_patterns[i].push(char::from((*line).as_bytes()[i]));
            }
        }

        Self {
            horizontal_patterns,
            vertical_patterns
        }
    }

    pub fn get_reflections(&self) -> usize {
        println!("Horizontal reflections:");
        let horizontal_reflection = Mirror::get_reflection_from_patterns(&self.horizontal_patterns, MAX_SMUDGES);
        println!("biggest_horizontal_reflection {:?}", horizontal_reflection);

        println!("Vertical reflections:");
        let vertical_reflection = Mirror::get_reflection_from_patterns(&self.vertical_patterns, MAX_SMUDGES);
        println!("biggest_vertical_reflection {:?}", vertical_reflection);

        let x = if (horizontal_reflection.0 > vertical_reflection.0 || horizontal_reflection.1 == 1) && vertical_reflection.1 == 0 {
            horizontal_reflection.0 * 100
        } else {
            vertical_reflection.0
        };
        println!("{}", std::format!("Result {}", x).cyan());

        return x;
    }

    fn nb_diff(p_1: &Vec<char>, p_2: &Vec<char>) -> usize {
        let mut nb_diff = 0;
        if p_1.len() != p_2.len() {
            return usize::MAX;
        }

        for i in 0..p_1.len() {
            if p_1[i] != p_2[i] {
                nb_diff += 1;
            }
        }
        return nb_diff;
    }

    fn get_reflection_from_patterns(patterns: &Vec<Vec<char>>, max_smudges: usize) -> (usize, usize) {
        let mut matching_reflection: (usize, usize) = (0, 0);
        let mut previous_pattern = &patterns[0];
        for i in 1..patterns.len() {
            let mut nb_smudges = Mirror::nb_diff(previous_pattern, &patterns[i]);
            if nb_smudges <= max_smudges {
                let mut current_diff = 1;
                let mut reflection_valid = false;
                loop {
                    if (i + current_diff == patterns.len()) ||
                        (i - 1 < current_diff) {
                        reflection_valid = true;
                        break;
                    }
                    let nb_diff = Mirror::nb_diff(&patterns[i - 1 - current_diff], &patterns[i + current_diff]);
                    nb_smudges += nb_diff;
                    if nb_smudges > max_smudges {
                        break
                    }
                    current_diff += 1;
                }
                if reflection_valid {
                    matching_reflection = (i, nb_smudges);
                    println!("Reflection at {} {}", i, current_diff);
                    if nb_smudges == max_smudges {
                        return matching_reflection;
                    }
                }
            }
            previous_pattern = &patterns[i];
        }
        return matching_reflection;
    }

    pub fn print(&self) {
        for horizontal_pattern in &self.horizontal_patterns {
            for char in horizontal_pattern {
                print!("{}", char);
            }
            print!("\n");
        }
    }
}

pub fn resolve(input: &String) {
    let mut mirror_lines: Vec<&str> = Vec::new();
    let mut sum: usize = 0;
    for line in input.lines() {
        if line.len() == 0 {
            let mirror = Mirror::new(&mirror_lines);
            println!("Mirror");
            mirror.print();
            sum += mirror.get_reflections();
            mirror_lines.clear();
            continue;
        }
        mirror_lines.push(line);
    }
    let mirror = Mirror::new(&mirror_lines);
    println!("Mirror");
    mirror.print();
    sum += mirror.get_reflections();
    println!("Sum {}", sum);
}