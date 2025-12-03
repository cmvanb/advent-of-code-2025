use std::{
    error::Error,
    io::{
        self,
        BufRead,
    },
};

fn read_stdin() -> String {
    io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("\n")
}

fn parse_input() -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    read_stdin()
        .lines()
        .map(|s| s.chars()
            .map(|c| c.to_digit(10)
                .map(|d| d as u8)
                .ok_or_else(|| "Invalid character".into()))
            .collect())
        .collect()
}

fn get_highest_joltage_combo(bank: &[u8]) -> String {
    let mut highest_index = 0;
    let mut second_index = 1;
    for i in 1..bank.len() {
        if bank[i] > bank[highest_index] && i < bank.len() - 1 {
            highest_index = i;
            second_index = i + 1;
        }
        if bank[i] > bank[second_index] && i != highest_index {
            second_index = i;
        }
    }
    format!("{}{}", bank[highest_index], bank[second_index])
}

fn main() {
    let banks = parse_input().expect("Failed to parse input");
    let combos = banks.iter()
        .map(|bank| get_highest_joltage_combo(bank))
        .collect::<Vec<String>>();
    println!("{:?}", combos);
    let total_joltage = combos.iter()
        .map(|combo| combo.parse::<u32>().unwrap_or(0))
        .sum::<u32>();
    println!("Total joltage: {}", total_joltage);
}
