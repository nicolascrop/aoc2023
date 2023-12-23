#[derive(Debug)]
struct Record {
    race_duration: usize,
    max_distance: usize
}

fn get_line_values(line: &str) -> Vec<&str> {
    return line
        .split(':').nth(1).unwrap()
        .split(' ')
        .map(|v| v.trim())
        .filter(|v| v.len() > 0)
        .collect();
}

pub fn resolve(input: &String) {
    let mut part_1_records: Vec<Record> = Vec::new();
    let mut part_2_records: Vec<Record> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<&str> = get_line_values(lines[0]);
    let distances: Vec<&str> = get_line_values(lines[1]);

    println!("Times {:?} (len: {})", times, times.len());
    println!("Distances {:?} (len: {})", distances, distances.len());
    if times.len() != distances.len() {
        panic!("Times and Distances len not equals");
    }

    let mut part_2_time = String::new();
    let mut part_2_distance = String::new();
    for i in 0..times.len() {
        part_1_records.push(Record {
            race_duration: times[i].parse().unwrap(),
            max_distance: distances[i].parse().unwrap()
        });
        part_2_time.push_str(times[i]);
        part_2_distance.push_str(distances[i]);
    }
    println!("Records part 1: {:?}", part_1_records);

    part_2_records.push(Record {
        race_duration: part_2_time.parse().unwrap(),
        max_distance: part_2_distance.parse().unwrap(),
    });
    println!("Records part 2: {:?}", part_2_records);

    let mut result = 0;

    for race in part_2_records {
        let mut race_winnable = 0;
        // For each ms on button we gain 1mm/sec speed
        for i in 0..=race.race_duration {
            let boat_distance = usize::from(i * (race.race_duration - i));
            if boat_distance > race.max_distance {
                race_winnable = race_winnable + 1;
            }
        }
        println!("Race is winnable {} times", race_winnable);
        if result == 0 {
            result = race_winnable
        } else {
            result = result * race_winnable;
        }
    }
    println!("Result is {}", result);
}