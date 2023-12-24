use std::collections::HashMap;

const RANGE_MIN: usize = 1;
const RANGE_MAX: usize = 4000;

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
enum Category {
    X,
    M,
    A,
    S
}

fn parse_category(ch: char) -> Category {
    match ch {
        'x' => Category::X,
        'm' => Category::M,
        'a' => Category::A,
        's' => Category::S,
        _ => panic!("Category {} non existent", ch)
    }
}

#[derive(Copy, Clone, Debug)]
enum Operator {
    LOWER,
    GREATER
}

fn parse_operator(ch: char) -> Operator {
    match ch {
        '<' => Operator::LOWER,
        '>' => Operator::GREATER,
        _ => panic!("Operator {} non existent", ch)
    }
}

fn compare(value_a: usize, value_b: usize, op: &Operator) -> bool {
    return match op {
        Operator::GREATER => value_a > value_b,
        Operator::LOWER => value_a < value_b,
    }
}

fn get_range_from_operator(value: usize, op: &Operator) -> (usize, usize) {
    return match op {
        Operator::GREATER => (value, RANGE_MAX),
        Operator::LOWER => (RANGE_MIN, value),
    }
}

#[derive(Debug)]
struct Action {
    category: Option<Category>,
    operator: Option<Operator>,
    value_ref: Option<usize>,
    output: String
}

impl Action {
    pub fn new(txt: &str) -> Self {
        let parts: Vec<&str> = txt.split(':').collect();
        return if parts.len() == 2 {
            Self {
                category: Some(parse_category(parts[0].chars().nth(0).unwrap())),
                operator: Some(parse_operator(parts[0].chars().nth(1).unwrap())),
                value_ref: Some(parts[0][2..].to_string().parse().unwrap()),
                output: String::from(parts[1])
            }
        } else {
            Self {
                category: None,
                operator: None,
                value_ref: None,
                output: String::from(parts[0])
            }
        }
    }

    pub fn process(&self, values: &Vec<usize>) -> Option<&String> {
        if !self.category.is_some() {
            return Some(&self.output);
        }
        let index = match self.category.unwrap() {
            Category::X => 0,
            Category::M => 1,
            Category::A => 2,
            Category::S => 3,
        };
        if compare(values[index], self.value_ref.unwrap(), &self.operator.unwrap()) {
            return Some(&self.output);
        }
        return None;
    }

    pub fn process_range(&self, ranges: &RangeMap) -> (&String, RangeMap, Option<RangeMap>) {
        if !self.category.is_some() {
            return (&self.output, ranges.clone(), None);
        }
        let category = self.category.unwrap();
        let mut next_range_valid = ranges.clone();
        let mut next_range_invalid = ranges.clone();
        return match self.operator.unwrap() {
            Operator::GREATER => {
                let range_value = ranges.get(&category).unwrap();
                next_range_valid.insert(category, (self.value_ref.unwrap() + 1, range_value.1));
                next_range_invalid.insert(category, (range_value.0, self.value_ref.unwrap()));
                (&self.output, next_range_valid.clone(), Some(next_range_invalid.clone()))
            },
            Operator::LOWER => {
                let range_value = ranges.get(&category).unwrap();
                next_range_valid.insert(category, (range_value.0, self.value_ref.unwrap() - 1));
                next_range_invalid.insert(category, (self.value_ref.unwrap(), range_value.1));
                (&self.output, next_range_valid.clone(), Some(next_range_invalid.clone()))
            }
        }
    }
}

fn do_workflow(rules: &HashMap<String, Vec<Action>>, values: &Vec<usize>) -> bool {
    let mut current = String::from("in");
    loop {
        for action in rules.get(&current).unwrap() {
            let result = action.process(values);
            if result.is_some() {
                current = result.unwrap().clone();
                break;
            }
        }

        if current == "A" {
            return true;
        }
        if current == "R" {
            return false;
        }
    }
}

type RangeMap = HashMap<Category, (usize, usize)>;

fn compute_range_possibilities(range: &RangeMap) -> usize {
    let mut result = 0;
    for r in range {
        let value = if r.1.0 < r.1.1 { (r.1.1 - r.1.0) + 1 } else { 0 };
        if result == 0 {
            result = value;
        } else {
            result *= value;
        }
    }
    return result;
}

fn explore_workflows(rules: &HashMap<String, Vec<Action>>) {
    let mut sum = 0;
    let mut currents: Vec<(String, RangeMap)> = vec![(String::from("in"), HashMap::from([
        (Category::X, (1, 4000)),
        (Category::M, (1, 4000)),
        (Category::A, (1, 4000)),
        (Category::S, (1, 4000))
    ]))];
    loop {
        let mut next_currents:Vec<(String, RangeMap)> = Vec::new();
        for current in currents {
            let mut current_range = current.1.clone();
            for action in rules.get(&current.0).unwrap() {
                println!("{} Action {:?}", current.0, action);
                let result = action.process_range(&current_range);
                println!("Result {:?}", result);
                if result.0 == "R" {
                    if result.2.is_some() {
                        current_range = result.2.unwrap().clone();
                    }
                    continue;
                }
                if result.0 == "A" {
                    println!("\nFound range!!! action {:?} range: {:?}", action, result.1);
                    sum += compute_range_possibilities(&result.1);
                    if result.2.is_some() {
                        current_range = result.2.unwrap().clone();
                    }
                    continue;
                }
                next_currents.push((result.0.clone(), result.1.clone())); // Happy path
                if result.2.is_some() {
                    current_range = result.2.unwrap().clone();
                }
            }
        }
        currents = next_currents;
        if currents.len() == 0 {
            break;
        }
    }
    println!("Sum {}", sum);
    println!("Max 256000000000000");
    println!("She 167409079868000");

    // Test
    /*
    X 1 => 1416 | 2662 => 4000    1416 + 1338
    M 1 => 4000                   4000
    A 1 => 2006                   2006
    S 1 => 1351 | 2770 => 4000    1351 + 1230
    max: 256000000000000
    res: 167409079868000
     */
}

pub fn resolve(input: &String) {
    let mut rules: HashMap<String, Vec<Action>> = HashMap::new();
    let mut parse_rules = true;
    let mut sum = 0;
    for line in input.lines() {
        if line.len() == 0 {
            parse_rules = false;
            continue;
        }
        if parse_rules {
            let parts: Vec<&str> = line.split("{").map(|x| {
                x.trim()
            }).collect();
            rules.insert(String::from(parts[0]), parts[1][..parts[1].len() - 1].split(",").map(|a| -> Action {
                return Action::new(a);
            }).collect());
            continue;
        }

        let parts: Vec<usize> = line[1..line.len() - 1].split(",").map(|part| -> usize {
            return part[2..].parse().unwrap()
        }).collect();
        println!("Parts: {:?}", parts);
        let result = do_workflow(&rules, &parts);
        println!("do workflow res: {}", result);
        if result {
            for part in parts {
                sum += part;
            }
        }
    }
    println!("Result {}", sum);

    println!("Explore workflows");
    explore_workflows(&rules);

}