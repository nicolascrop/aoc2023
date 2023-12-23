use std::collections::HashMap;

fn sequence_is_valid_for_spring(spring: &Vec<char>, is_full: bool, sequences: &Vec<usize>) -> bool {
    let mut nb_same_char: usize = 1;
    let mut current_seq_index = 0;
    for i in 1..spring.len() {
        if spring[i] != spring[i - 1] {
            if spring[i - 1] == '#' {
                if current_seq_index == sequences.len() {
                    return false;
                }
                if spring[i] == '?' {
                    if nb_same_char > sequences[current_seq_index] {
                        return false;
                    }
                    return true;
                }
                if nb_same_char != sequences[current_seq_index] {
                    return false;
                }
                current_seq_index += 1;
            }
            nb_same_char = 0;
        }
        nb_same_char += 1;
    }
    if spring[spring.len() - 1] == '#' {
        if current_seq_index == sequences.len() {
            return false;
        }
        if is_full && nb_same_char != sequences[current_seq_index] {
            return false;
        }
        current_seq_index += 1;
    }
    if is_full && current_seq_index != sequences.len() {
        return false;
    }
    return true;
}

fn copy_and_add_to_vec(current: &Vec<char>, new_char: char) -> Vec<char> {
    let mut current_slice: Vec<char> = Vec::new();
    for char in current {
        current_slice.push(*char);
    }
    current_slice.push(new_char);
    return current_slice;
}

fn explore(current: &Vec<char>, rest: &[char], sequences: &Vec<usize>, cache: &mut HashMap<String, usize>) -> usize {
    // println!("Exploring {:?} {:?}, {:?}", current, rest, sequences);
    let id = format!("{:?}", current);
    let found_in_map = cache.get(&id);
    if found_in_map.is_some() {
        return *found_in_map.unwrap();
    }

    if current.len() > 0 && !sequence_is_valid_for_spring(current, rest.len() == 0, sequences) {
        cache.insert(id, 0);
        return 0;
    }
    if rest.len() == 0 {
        cache.insert(id, 1);
        return 1;
    }

    if rest[0] == '?' {
        let mut sum = 0;
        sum += explore(&copy_and_add_to_vec(current, '.'), &rest[1..], sequences, cache);
        sum += explore(&copy_and_add_to_vec(current, '#'), &rest[1..], sequences, cache);
        cache.insert(id, sum);
        return sum;
    }

    let mut next = copy_and_add_to_vec(current, rest[0]);
    let mut rest_index = 1;
    for i in 1..rest.len() {
        if rest[i] == '?' {
            break;
        }
        next.push(rest[i]);
        rest_index += 1;
    }

    let result = explore(&next, &rest[rest_index..], sequences, cache);
    cache.insert(id, result);
    return result;
}

fn compute_possibilities(springs: &Vec<char>, sequences: &Vec<usize>) -> usize {
    return explore(&Vec::new(), &springs[0..], sequences, &mut HashMap::new());
}

pub fn resolve(input: &String) {
    let mut sum = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let springs: Vec<char> = parts[0].split("").map(|c| c.trim()).filter(|c| c.len() == 1).map(|c| c.chars().nth(0).unwrap()).collect();
        let sequences: Vec<usize> = parts[1].split(",").map(|seq| -> usize {
            return seq.trim().parse().unwrap();
        }).collect();
        let mut unfolded_springs: Vec<char> = Vec::new();
        let mut unfolded_sequences: Vec<usize> = Vec::new();
        for i in 0..5 {
            for char in &springs {
                unfolded_springs.push(*char);
            }
            if i != 4 {
                unfolded_springs.push('?');
            }
            for el in &sequences {
                unfolded_sequences.push(*el);
            }
        }
        // println!("Line {}. springs: {:?} sequences: {:?}", line, springs, sequences);
        println!("Unfolded springs: {:?} unfolded sequences: {:?}", unfolded_springs, unfolded_sequences);
        let now = std::time::SystemTime::now();
        // Part 1:
        //let possibilities = compute_possibilities(&springs, &sequences);
        // Part 2:
        let possibilities = compute_possibilities(&unfolded_springs, &unfolded_sequences);
        println!("Possibilities {} in {}ms", possibilities, now.elapsed().unwrap().as_millis());
        sum += possibilities;
    }
    println!("Result: {}", sum);
}