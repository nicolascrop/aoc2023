#[derive(PartialEq, Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal: usize
}

#[derive(Debug)]
struct Boxes<'b> {
    content: Vec<Vec<Lens<'b>>>
}

impl Boxes<'_> {
    pub fn new() -> Self {
        Self {
            content: vec![vec![]; 256]
        }
    }

    pub fn print(&self) {
        for i in 0..self.content.len() {
            if self.content[i].len() > 0 {
                println!("Box {} => {:?}", i, self.content[i]);
            }
        }
    }

    pub fn focusing_power(&self) -> usize {
        let mut sum: usize = 0;
        for i in 0..self.content.len() {
            for j in 0..self.content[i].len() {
                sum += (i + 1) * (j + 1) * self.content[i][j].focal;
            }
        }
        return sum;
    }
}

fn do_hash(str: &str) -> usize {
    let mut hash: usize = 0;
    for char in str.chars() {
        let ascii_code = char as u32;
        hash += ascii_code as usize;
        hash = hash * 17;
        hash = hash % 256;
    }
    return hash;
}
fn part_1(input: &String) {
    println!("Sum value {}", input.split(",").map(do_hash).sum::<usize>());
}

fn part_2(input: &String) {
    let mut boxes = Boxes::new();
    for step in input.split(",") {
        let parts: Vec<&str> = step.split(|c| c == '-' || c == '=').filter(|x| x.len() > 0).collect();
        let label = parts[0];
        let box_index = do_hash(&label);
        println!("parts: {:?} label: {} box_index {}", parts, label, box_index);
        let box_found = boxes.content[box_index].iter().position(|v| v.label == label);
        if parts.len() == 1 {
            // Dash operation
            if box_found.is_some() {
                let found_at = box_found.unwrap();
                println!("Found at {}", found_at);
                boxes.content[box_index].remove(found_at);
            }
        } else {
            if box_found.is_some() {
                let found_at = box_found.unwrap();
                boxes.content[box_index][found_at].focal = parts[1].parse().unwrap();
            } else {
                boxes.content[box_index].push( Lens { label, focal: parts[1].parse().unwrap() });
            }
        }
    }
    boxes.print();
    eprintln!("Focusing power {}", boxes.focusing_power());
}

pub fn resolve(input: &String) {
    part_1(input);
    part_2(input)
}