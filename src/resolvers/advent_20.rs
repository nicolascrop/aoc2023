use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
#[derive(Clone, Copy, PartialEq, Debug)]
enum Pulse {
    High,
    Low
}
type RecentPulses = HashMap<String, Pulse>;

trait Module: std::fmt::Debug {
    fn receive_pulse(&mut self, pulse: &Pulse, recent_pulses: &RecentPulses) -> Vec<(String, Pulse)>;

    fn label(&self) -> String;
    fn has_destination(&self, destination: &String) -> bool;

    fn is_conjunction_module(&self) -> bool { false }
    fn as_conjunction_module(&mut self) -> Option<&mut ConjunctionModule> { None }
}

#[derive(Debug)]
struct BroadcasterModule {
    label: String,
    destinations: Vec<String>,
}

impl BroadcasterModule {
    fn new(_: &str, out: &str) -> Self {
        return Self {
            label: String::from("broadcaster"),
            destinations: out.split(", ").map(|dest| String::from(dest.trim())).filter(|dest| dest.len() > 0).collect(),
        }
    }
}

impl Module for BroadcasterModule {
    fn receive_pulse(&mut self, pulse: &Pulse, _: &RecentPulses) -> Vec<(String, Pulse)> {
        return self.destinations.iter().map(|destination| -> (String, Pulse) {
            return (destination.clone(), pulse.clone());
        }).collect();
    }

    fn label(&self) -> String {
        return self.label.clone();
    }

    fn has_destination(&self, destination: &String) -> bool {
        return self.destinations.contains(destination);
    }
}


#[derive(Debug)]
struct FlipFlopModule {
    label: String,
    destinations: Vec<String>,
    status: bool
}

impl FlipFlopModule {
    fn new(label: &str, out: &str) -> Self {
        return Self {
            label: String::from(&label[1..]),
            destinations: out.split(", ").map(|dest| String::from(dest.trim())).filter(|dest| dest.len() > 0).collect(),
            status: false
        }
    }
}

impl Module for FlipFlopModule {

    fn receive_pulse(&mut self, pulse: &Pulse, _: &RecentPulses) -> Vec<(String, Pulse)> {
        if *pulse == Pulse::High {
            return Vec::new();
        }
        // println!("On flip flop dest: {:?} status: {}", self.destinations, self.status);

        let out: Vec<(String, Pulse)> = self.destinations.iter().map(|destination| -> (String, Pulse) {
            return (destination.clone(), if !self.status { Pulse::High } else { Pulse::Low });
        }).collect();
        // println!("Out: {:?}", out);
        self.status = !self.status;
        return out;
    }

    fn label(&self) -> String {
        return self.label.clone();
    }

    fn has_destination(&self, destination: &String) -> bool {
        return self.destinations.contains(destination);
    }
}

#[derive(Debug)]
struct ConjunctionModule {
    label: String,
    destinations: Vec<String>,
    inputs_module: Vec<String>
}

impl ConjunctionModule {
    fn new(label: &str, out: &str) -> Self {
        return Self {
            label: String::from(&label[1..]),
            destinations: out.split(", ").map(|dest| String::from(dest.trim())).filter(|dest| dest.len() > 0).collect(),
            inputs_module: Vec::new()
        }
    }

    fn add_input_module(&mut self, label: String) {
        self.inputs_module.push(label);
    }

    fn has_destination(&self, destination: &String) -> bool {
        return self.destinations.contains(destination);
    }
}

impl Module for ConjunctionModule {
    fn receive_pulse(&mut self, _: &Pulse, recent_pulses: &RecentPulses) -> Vec<(String, Pulse)> {
        let mut pulse_to_send = Pulse::Low;
        for input_module in &self.inputs_module {
            let found = recent_pulses.get(input_module);
            if found.is_some() && *found.unwrap() == Pulse::Low {
                pulse_to_send = Pulse::High;
                break;
            }
        }

        return self.destinations.iter().map(|destination| -> (String, Pulse) {
            return (destination.clone(), pulse_to_send);
        }).collect();
    }

    fn label(&self) -> String {
        return self.label.clone();
    }

    fn has_destination(&self, destination: &String) -> bool {
        return self.destinations.contains(destination);
    }

    fn is_conjunction_module(&self) -> bool {
        true
    }
    
    fn as_conjunction_module(&mut self) -> Option<&mut ConjunctionModule> {
        Some(self)
    }
}

fn push_button(
    modules_hashmap: &HashMap<String, usize>,
    modules: &mut Vec<Box<dyn Module>>,
    recent_pulses: &mut HashMap<String, Pulse>,
    nb_times_pressed: usize
) -> (usize, usize, bool) {
    let mut sum_high_pulse = 0;
    let mut sum_low_pulse = 0;
    let mut rx_called_with_low = false;
    let mut actions: Vec<(String, Pulse)> = vec![(String::from("broadcaster"), Pulse::Low)];
    loop {
        if actions.len() == 0 {
            break;
        }
        let mut new_actions: Vec<(String, Pulse)> = Vec::new();
        for action in &actions {
            if action.1 == Pulse::High {
                sum_high_pulse += 1;
            } else {
                sum_low_pulse += 1;
                if action.0 == "rx" {
                    rx_called_with_low = true;
                    println!("RX called with LOW -> {}", nb_times_pressed);
                }
            }
            let modules_index = modules_hashmap.get(&action.0);
            if !modules_index.is_some() {
                continue;
            }
            let module = modules[*modules_index.unwrap()].deref_mut();
            for new_action in module.receive_pulse(&action.1, recent_pulses) {
                recent_pulses.insert(action.0.clone(), new_action.1);
                if module.label() == "lk" && new_action.0 == "nc" && new_action.1 == Pulse::High {
                    // 4003
                    // panic!("lk output HIGH -> {}", nb_times_pressed);
                }
                if module.label() == "fn" && new_action.0 == "nc" && new_action.1 == Pulse::High {
                    // 3847
                    // panic!("fn output HIGH -> {}", nb_times_pressed);
                }
                if module.label() == "fh" && new_action.0 == "nc" && new_action.1 == Pulse::High {
                    // 3851
                    // panic!("fh output HIGH -> {}", nb_times_pressed);
                }
                if module.label() == "hh" && new_action.0 == "nc" && new_action.1 == Pulse::High {
                    // 4027
                    // panic!("hh output HIGH -> {}", nb_times_pressed);
                }
                // if new_action.0.clone() == "fn" {
                //     panic!("fn output HIGH -> {}", nb_times_pressed);
                // }
                // if new_action.0.clone() == "fh" {
                //     panic!("fh output HIGH -> {}", nb_times_pressed);
                // }
                // if new_action.0.clone() == "hh" {
                //     panic!("hh output HIGH -> {}", nb_times_pressed);
                // }
                new_actions.push(new_action);
                // if action.0 == "fn" {
                //     println!("fn called with HIGH -> {}", nb_times_pressed);
                // }
                // if action.0 == "fh" {
                //     println!("fh called with HIGH -> {}", nb_times_pressed);
                // }
                // if action.0 == "hh" {
                //     println!("hh called with HIGH -> {}", nb_times_pressed);
                // }
            }
        }
        actions = new_actions;
    }
    return (sum_high_pulse, sum_low_pulse, rx_called_with_low);
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn lcm(arr: Vec<i32>) -> u64 {
    let mut res: u64 = 1;
    for i in arr {
        res = (res * i as u64) / gcd(res, i as u64);
    }
    res
}

pub fn resolve(input: &String) {
    let mut modules: Vec<Box<dyn Module>> = Vec::new();
    let mut modules_hashmap: HashMap<String, usize> = HashMap::new();
    let mut recent_pulses: RecentPulses = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts[0] == "broadcaster" {
            modules.push(Box::new(BroadcasterModule::new(parts[0], parts[1])));
        } else {
            let first_char = parts[0].chars().nth(0).unwrap();
            if first_char == '%' {
                modules.push(Box::new(FlipFlopModule::new(parts[0], parts[1])));
            } else {
                modules.push(Box::new(ConjunctionModule::new(parts[0], parts[1])));
            }
        }
    }

    println!("Modules:");
    for i in 0..modules.len() {
        println!("{:?}", modules[i]);
        modules_hashmap.insert(modules[i].deref().label(), i);
        recent_pulses.insert(modules[i].deref().label(), Pulse::Low);

        if modules[i].is_conjunction_module() {
            let modules_with_destination: Vec<String> = modules.iter().filter(|m| m.has_destination(&modules[i].label())).map(|m| m.label()).collect();
            let mut conjunction_module = modules[i].deref_mut().as_conjunction_module().unwrap();
            for module_with_destination in &modules_with_destination {
                conjunction_module.add_input_module(module_with_destination.clone());
            }
        }
    }

    println!("\nModules hashmap:");
    for entry in modules_hashmap.iter() {
        println!("Entry {} {} {:?}", entry.0, entry.1, modules[*entry.1]);
    }

    println!("{}", lcm(vec![4003, 3847, 3851, 4027]));

    let mut sum_high = 0;
    let mut sum_low = 0;
    let mut i = 0;
    loop {
        let result = push_button(&modules_hashmap, &mut modules, &mut recent_pulses, i + 1);
        // println!("Sum high {}, sum Low {}", result.0, result.1);
        // println!("\n");
        if result.2 {
            break;
        }
        sum_high += result.0;
        sum_low += result.1;
        i+= 1;
    }

    println!("Final high {}, Low {}, res: {}", sum_high, sum_low, sum_high * sum_low);
}