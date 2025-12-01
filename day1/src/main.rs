use std::io::{self, BufRead};

const DIAL_MAX: i32 = 99;
const DIAL_ORIGIN: i32 = 50;

#[derive(Debug, Clone, Copy)]
struct Dial {
    current_tick: i32,
}

impl Dial {
    pub fn rotate(&mut self, ticks: i32) -> i32 {
        let zeroes = if ticks >= 0 {
            (self.current_tick + ticks) / (DIAL_MAX + 1)
        } else {
            let base_zeroes = ((-ticks >= self.current_tick) as i32) * (1 + (-ticks - self.current_tick) / (DIAL_MAX + 1));
            if self.current_tick == 0 {
                base_zeroes - 1
            } else {
                base_zeroes
            }
        };

        self.current_tick = (self.current_tick + ticks).rem_euclid(DIAL_MAX + 1);
        zeroes
    }
}


fn main() {
    let stdin = io::stdin();
    let input = stdin.lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("\n");

    let rotations = parse_input(&input);

    let mut dial = Dial { current_tick: DIAL_ORIGIN };
    let mut zero_counter = 0;

    for rotation in rotations {
        let original_position = dial.current_tick;
        let zeroes = dial.rotate(rotation);
        zero_counter += zeroes;
        println!("Rotate {} from {} to {}, zeroes: {}", rotation, original_position, dial.current_tick, zeroes);
    }

    println!("Zero count: {}", zero_counter);
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let direction = line.chars().next()?;
            let number = line[1..].parse::<i32>().ok()?;

            match direction {
                'L' => Some(-number),
                'R' => Some(number),
                _ => None,
            }
        })
        .collect()
}
