use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::sync::mpsc;

fn count_stones(level: u32, stone: u64, result: &mut u128) {
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
    let mut result = start_stones.len() as u128;
    for stone in start_stones {
        println!("Count stone {}", stone);
        count_stones(blinks, stone, &mut result);
    }
    println!("Number of stones {}", result);
}
const CACHE_LEVEL: u32 = 37;

fn count_stones_cache(level: u32, stone: u64, result: &mut u128, cache: &HashMap<u64, u128>) {
    if level == CACHE_LEVEL && cache.contains_key(&stone) {
//        println!("Get from cache {} result {}", stone, cache.get(&stone).unwrap());
        *result += cache.get(&stone).unwrap();
        return
    }
    if level == 0 {
        return
    }
    if stone == 0 {
        count_stones_cache(level-1, 1, result, cache);
        return
    }
    let digits:u64 = (stone.checked_ilog10().unwrap_or(0) + 1).into();
    if (digits % 2) == 0 {
        *result += 1; 
        let divider:u64 = 10_i32.pow((digits/2).try_into().unwrap()).try_into().unwrap();
        let new_stone_1 = stone / divider;
        count_stones_cache(level-1, new_stone_1, result, cache);
        let new_stone_2 = stone % divider;
        count_stones_cache(level-1, new_stone_2, result, cache);
        return
    }
    let new_stone = stone * 2024;
    count_stones_cache(level-1, new_stone, result, cache);
}

fn day11_2(filename: &str, blinks: u32, nbr_threads: usize) {
    let mut start_stones: Vec<u64> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            for number_str in line.split_whitespace() {
                start_stones.push(number_str.parse::<u64>().unwrap())
            }
        }
    }
    let mut total: u128 = start_stones.len() as u128;
    let mut cache: HashMap<u64, u128> = HashMap::new();
    let (tx, rx) = mpsc::channel();
    for t in 0..nbr_threads {
        let local_t = t.clone();
        let local_tx = tx.clone();
        thread::spawn(move || {
            let mut local_cache: HashMap<u64, u128> = HashMap::new();
            for i in (local_t..80960).step_by(nbr_threads) {
                let mut result = 0;
//                println!("{}", i);
                count_stones(CACHE_LEVEL, i.try_into().unwrap(), &mut result);
                local_cache.insert(i.try_into().unwrap(), result);
            }
            println!("Finished cache {}", local_t);
            local_tx.send(local_cache).unwrap();
        });    
    }
    for _ in 0..nbr_threads {
        cache.extend(rx.recv().unwrap());
        println!("Got: cache data");
    }

    println!("Cache created");
    let (stone_tx, stone_rx) = mpsc::channel();
    for stone in start_stones.clone() {
//        let local_stone = stone.clone();
        let local_tx = stone_tx.clone();
        let local_cache = cache.clone();
        let mut result: u128 = 0;
        thread::spawn(move || {
            println!("Count stone {}", stone);
            count_stones_cache(blinks, stone, &mut result, &local_cache);
            local_tx.send(result).unwrap();
        });
    }
    for _ in 0..start_stones.len() {
        total += stone_rx.recv().unwrap();
        println!("Got: result {}", total);
    }
    println!("Number of stones {}", total);
}

fn main() {
    // day11_1("test1.txt", 6);
    // day11_1("test1.txt", 25);
    // day11_1("input.txt", 25);
    // day11_2("test1.txt", 25, 8);
    // day11_2("input.txt", 25, 8);
    day11_2("input.txt", 75, 24);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}