use std::error::Error;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

fn read_stdin() -> String {
    io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("\n")
}

fn parse_input() -> Result<(Vec<RangeInclusive<u64>>, Vec<u64>), Box<dyn Error>> {
    let input = read_stdin();
    let mut lines = input.lines();

    let mut ranges = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid range format".into());
        }
        let start: u64 = parts[0].parse()?;
        let end: u64 = parts[1].parse()?;
        ranges.push(start..=end);
    }

    let mut integers = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        integers.push(line.parse()?);
    }

    Ok((ranges, integers))
}

fn merge_ranges(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return Vec::new();
    }

    let mut ranges: Vec<_> = ranges.into_iter().map(|r| (*r.start(), *r.end())).collect();
    ranges.sort_by_key(|r| r.0);

    let mut merged = Vec::new();
    let (mut current_start, mut current_end) = ranges[0];

    for (start, end) in ranges.into_iter().skip(1) {
        if start <= current_end + 1 {
            current_end = current_end.max(end);

        } else {
            merged.push(current_start..=current_end);
            current_start = start;
            current_end = end;
        }
    }
    merged.push(current_start..=current_end);

    merged
}

fn main() {
    let (fresh_ingredient_ranges, ingredients) = parse_input().expect("Failed to parse input");

    let fresh_count =
        ingredients
        .iter()
        .filter_map(|&ingredient| {
            if fresh_ingredient_ranges.iter().any(|range| range.contains(&ingredient)) {
                Some(ingredient)

            } else {
                None
            }
        })
        .count();
    println!("{}", fresh_count);

    let merged_ranges = merge_ranges(fresh_ingredient_ranges);
    let total_fresh_ids: u64 = merged_ranges.iter().map(|r| r.end() - r.start() + 1).sum();

    println!("{}", total_fresh_ids);
}
