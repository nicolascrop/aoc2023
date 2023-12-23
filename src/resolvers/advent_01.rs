const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const DIGITS_AS_WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

struct Matcher {
    temp_matchers: Vec<String>
}

impl Matcher {
    pub fn new() -> Self {
        Self {
            temp_matchers: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        let _ = &self.temp_matchers.clear();
    }

    pub fn digit_match(&mut self, next_key: char) -> Option<char> {
        if DIGITS.contains(&next_key) {
            let _ = &self.temp_matchers.clear();
            return Some(next_key);
        }
        let mut to_return: Option<char> = None;
        let mut new_matchers: Vec<String> = Vec::new();
        let _ = &self.temp_matchers.push(String::new());
        for temp_matcher in &mut self.temp_matchers {
            temp_matcher.push(next_key);
            let mut i = 0;
            let mut has_matched = false;
            for digit_as_word in DIGITS_AS_WORDS {
                if digit_as_word == temp_matcher {
                    to_return = Some(DIGITS[i + 1]);
                }

                if !has_matched && digit_as_word.starts_with(temp_matcher.as_str()) {
                    has_matched = true;
                }
                i = i + 1;
            }
            if has_matched {
                new_matchers.push(temp_matcher.to_string());
            }
        }
        let _ = &self.temp_matchers.clear();
        for new_matcher in new_matchers {
            let _ = &self.temp_matchers.push(new_matcher);
        }
        return to_return;
    }
}

pub fn resolve(input: &String) {
    let mut matcher = Matcher::new();
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        let mut str = String::new();
        let mut digit_b: Option<char> = None;
        for char in line.chars() {
            let digit = matcher.digit_match(char);
            if digit.is_some() {
                if str.len() == 0 {
                    str.push(digit.unwrap());
                }
                digit_b = digit;
            }
        }
        matcher.clear();
        if digit_b.is_some() {
            str.push(digit_b.unwrap());
        } else {
            str.push(str.chars().nth(0).unwrap());
        }
        // println!("{} -> {}", line, str);
        let nb: i64 = str.parse().unwrap();
        result += nb;
    }
    println!("Result {}", result);
}