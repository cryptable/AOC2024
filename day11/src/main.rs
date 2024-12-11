use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn count_stones(level: u32, stone: u64, result: &mut u64) {
    if level == 0 {
        return
    }
    if stone == 0 {
        count_stones(level-1, 1, result);
        return
    }
    let digits:u64 = (stone.checked_ilog10().unwrap_or(0) + 1).into();
    if (digits % 2) == 0 {
        *result += 1; 
        let divider:u64 = 10_i32.pow((digits/2).try_into().unwrap()).try_into().unwrap();
        let new_stone_1 = stone / divider;
        count_stones(level-1, new_stone_1, result);
        let new_stone_2 = stone % divider;
        count_stones(level-1, new_stone_2, result);
        return
    }
    let new_stone = stone * 2024;
    count_stones(level-1, new_stone, result);
}

fn day11_1(filename: &str, blinks: u32) {
    let mut start_stones: Vec<u64> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            for number_str in line.split_whitespace() {
                start_stones.push(number_str.parse::<u64>().unwrap())
            }
        }
    }
    let mut result = start_stones.len() as u64;
    for stone in start_stones {
        println!("Count stone {}", stone);
        count_stones(blinks, stone, &mut result);
    }
    println!("Number of stones {}", result);
}

fn main() {
    day11_1("test1.txt", 6);
    day11_1("test1.txt", 25);
    day11_1("input.txt", 25);
    day11_1("input.txt", 75);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}