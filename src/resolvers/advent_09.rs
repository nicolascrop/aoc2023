use crate::resolvers::Part;

fn predict_next(sequence: &Vec<i32>) -> i32 {
    let mut sequence_of_differences: Vec<Vec<i32>> = Vec::new();
    loop {
        let mut last_sequence: &Vec<i32> = sequence;
        if sequence_of_differences.len() > 0 {
            last_sequence = &sequence_of_differences[sequence_of_differences.len() - 1];
        }
        let mut next_sequence: Vec<i32> = Vec::new();
        let mut has_only_zero = true;
        for i in 0..last_sequence.len() - 1 {
            let difference: i32 = last_sequence[i + 1] - last_sequence[i];
            next_sequence.push(difference);
            if difference != 0 {
                has_only_zero = false;
            }
        }
        sequence_of_differences.push(next_sequence);
        if has_only_zero {
            break;
        }
    }
    sequence_of_differences.reverse();
    let mut previous_value_to_add = 0;
    for i in 1..sequence_of_differences.len() {
        previous_value_to_add = sequence_of_differences[i].last().unwrap() + previous_value_to_add;
    }
    return sequence.last().unwrap() + previous_value_to_add;
}

pub fn resolve(input: &String) {
    let part = Part::Two;
    let lines = input.lines();
    let mut result: i32 = 0;
    for line in lines {
        let mut sequence: Vec<i32> = line
            .split(" ")
            .filter(|v| v.len() > 0)
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        if part == Part::Two {
            sequence.reverse();
        }

        let next_sequence_value = predict_next(&sequence);
        println!("Sequence: {:?}. Next: {}", sequence, next_sequence_value);
        result += next_sequence_value;
    }
    println!("Result: {}", result);
}