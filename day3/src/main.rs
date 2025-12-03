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

fn get_highest_n_digits(bank: &[u8], n: usize) -> String {
    if n == 0 || bank.is_empty() {
        return String::new();
    }
    if n > bank.len() {
        return bank.iter().map(|d| d.to_string()).collect();
    }

    let mut result = Vec::with_capacity(n);
    let mut start = 0;

    for digits_left in (1..=n).rev() {
        let end = bank.len() - digits_left + 1;

        let mut max_pos = start;
        let mut max_val = bank[start];
        for i in start + 1..end {
            if bank[i] > max_val {
                max_val = bank[i];
                max_pos = i;
            }
        }

        result.push(bank[max_pos]);
        start = max_pos + 1;
    }

    result.iter().map(|d| d.to_string()).collect()
}

fn main() {
    let banks = parse_input().expect("Failed to parse input");

    let combos_part1 =
        banks.iter()
        .map(|bank| get_highest_joltage_combo(bank))
        .collect::<Vec<String>>();
    println!("{:?}", combos_part1);

    let total_joltage_part1 =
        combos_part1.iter()
        .map(|combo| combo.parse::<u32>().unwrap_or(0))
        .sum::<u32>();
    println!("Total 2-battery joltage: {}", total_joltage_part1);

    let combos_part2 =
        banks.iter()
        .map(|bank| get_highest_n_digits(bank, 12))
        .collect::<Vec<String>>();
    println!("{:?}", combos_part2);

    let total_joltage_part2 =
        combos_part2.iter()
        .map(|combo| combo.parse::<u64>().unwrap_or(0))
        .sum::<u64>();
    println!("Total 12-battery joltage: {}", total_joltage_part2);
}
