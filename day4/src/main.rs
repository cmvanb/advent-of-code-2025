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

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

struct Cell {
    has_paper: bool,
    reachable: bool,
}

struct Warehouse {
    grid: Vec<Vec<Cell>>,
}

impl Warehouse {
    fn from_input(input: String) -> Result<Self, Box<dyn Error>> {
        let grid =
            input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '@' => Ok(Cell { has_paper: true, reachable: false }),
                        '.' => Ok(Cell { has_paper: false, reachable: false }),
                        _ => Err(format!("Invalid character in input: {}", c)),
                    })
                    .collect::<Result<Vec<Cell>, _>>()
            })
            .collect::<Result<Vec<Vec<Cell>>, _>>()
            .map_err(|e: String| -> Box<dyn Error> { e.into() })?;

        Ok(Self { grid })
    }

    fn valid_position(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.grid[0].len() && (y as usize) < self.grid.len()
    }

    fn has_paper_at(&self, x: usize, y: usize) -> bool {
        if y >= self.grid.len() || x >= self.grid[y].len() {
            false
        } else {
            self.grid[y][x].has_paper
        }
    }

    fn count_adjacent_paper(&self, x: usize, y: usize) -> usize {
        DIRECTIONS
            .iter()
            .filter_map(|(dx, dy)| {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
                if self.valid_position(new_x, new_y) {
                    Some(self.has_paper_at(new_x as usize, new_y as usize))
                } else {
                    None
                }
            })
            .filter(|&has_paper| has_paper)
            .count()
    }

    fn get_reachable_positions(&self) -> Vec<(usize, usize)> {
        let mut reachable_positions = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if self.has_paper_at(x, y) && self.count_adjacent_paper(x, y) < 4 {
                    reachable_positions.push((x, y));
                }
            }
        }
        reachable_positions
    }
}

fn main() {
    let input = read_stdin();
    let mut warehouse = Warehouse::from_input(input).expect("Failed to parse input");

    let reachable_count = warehouse.get_reachable_positions().len();
    println!("Initially reachable: {}", reachable_count);

    let mut total_removable = 0;
    loop {
        let reachable_positions = warehouse.get_reachable_positions();
        if reachable_positions.is_empty() {
            break;
        }

        for (x, y) in &reachable_positions {
            warehouse.grid[*y][*x].reachable = true;
            warehouse.grid[*y][*x].has_paper = false;
        }

        total_removable += reachable_positions.len();
    }
    println!("Total removable: {}", total_removable);

    println!("Final grid state:");
    for row in &warehouse.grid {
        for cell in row {
            if cell.has_paper {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
