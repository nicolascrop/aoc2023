use std::ops::Add;

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

struct GameParser {
    result: u32,
    sum_of_game_power: u64
}

impl GameParser {
    pub fn new() -> Self {
        Self {
            result: 0,
            sum_of_game_power: 0
        }
    }

    fn get_game_id(game_str: &str) -> u32 {
        let parts = game_str.split(" ").collect::<Vec<_>>();
        return parts[1].parse().unwrap();
    }

    // Return r, g, b
    fn parse_round(round: &str) -> (u32, u32, u32) {
        let parts = round.split(",").collect::<Vec<_>>();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for part in parts {
            let color = part.trim().split(" ").collect::<Vec<_>>();
            let value: u32 = color[0].parse().unwrap();
            let color_name =  color[1];
            match color_name {
                "red" => r = value,
                "green" => g = value,
                "blue" => b = value,
                _ => println!("Cannot match round: {}, value: {}, color: {}", round, value, color_name)
            }
        }

        return (r, g, b);
    }

    pub fn parse(&mut self, line: &str) {
        let parts = line.split(":").collect::<Vec<_>>();
        let game_id = GameParser::get_game_id(parts[0]);
        println!("Line {}", line);
        let rounds = parts[1].split(";").collect::<Vec<_>>();
        let mut round_impossible = false;
        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;
        for round in rounds {
            let round_result = GameParser::parse_round(round);
            let r = round_result.0;
            let g = round_result.1;
            let b = round_result.2;
            println!("Game ID {}. R: {}, G: {}, B: {}", game_id, r, g, b);
            if !(r <= RED_CUBES && g <= GREEN_CUBES && b <= BLUE_CUBES) {
                round_impossible = true;
            }
            if r > max_r {
                max_r = r;
            }
            if g > max_g {
                max_g = g;
            }
            if b > max_b {
                max_b = b;
            }
        }
        if !round_impossible {
            self.result = self.result.add(game_id);
            println!("Game is valid, sum {}", &self.result);
        }
        println!("MAX R {}, MAX G {}, MAX B {}, power: {}", max_r, max_g, max_b, max_r * max_g * max_b);
        self.sum_of_game_power = self.sum_of_game_power.add((max_r * max_g * max_b) as u64);
        println!("Sum of game power {}", &self.sum_of_game_power);
    }
}

pub fn resolve(input: &String) {
    let lines = input.lines();
    let mut parser = GameParser::new();
    for line in lines {
        parser.parse(line);
    }
}