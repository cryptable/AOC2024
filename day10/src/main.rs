use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
struct Location {
    x: usize,
    y: usize,
}

fn char_to_i8(kar: char) -> i8 {
    match kar {
        '0' => return 0,
        '1' => return 1,
        '2' => return 2,
        '3' => return 3,
        '4' => return 4,
        '5' => return 5,
        '6' => return 6,
        '7' => return 7,
        '8' => return 8,
        '9' => return 9,
        _ => return -1,
    }
}

fn find_start_locations(map: &Vec<Vec<i8>>, queue: &mut VecDeque<Location>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                queue.push_back(Location {
                    x,
                    y,
                });
            }
        }
    }
}

fn find_next_locations_1(map: &Vec<Vec<i8>>, location: &Location, queue: &mut VecDeque<Location>, result: &mut Vec<Location>) {
    let next_location_value = map[location.y][location.x] + 1;
    
    // Check up
    if location.y > 0 {
        if map[location.y-1][location.x] == next_location_value {
            let next_location = Location {
                y:location.y-1,
                x:location.x,
            };
            if next_location_value == 9 && !result.contains(&next_location) {
                result.push(next_location);
            }
            else {
                queue.push_back(next_location)
            }
        }
    } 
    // Check right
    if location.x < (map[0].len()-1) {
        if map[location.y][location.x+1] == next_location_value {
            let next_location = Location {
                y:location.y,
                x:location.x+1,
            };
            if next_location_value == 9 && !result.contains(&next_location) {
                result.push(next_location);
            }
            else {
                queue.push_back(next_location)
            }
        }
    } 
    // Check down
    if location.y < map.len()-1 {
        if map[location.y+1][location.x] == next_location_value {
            let next_location = Location {
                y:location.y+1,
                x:location.x,
            };
            if next_location_value == 9 && !result.contains(&next_location) {
                result.push(next_location);
            }
            else {
                queue.push_back(next_location)
            }
        }
    } 
    // Check left
    if location.x > 0 {
        if map[location.y][location.x-1] == next_location_value {
            let next_location = Location {
                y:location.y,
                x:location.x-1,
            };
            if next_location_value == 9 && !result.contains(&next_location) {
                result.push(next_location);
            }
            else {
                queue.push_back(next_location)
            }
        }
    } 
}

fn find_next_locations_2(map: &Vec<Vec<i8>>, location: &Location, queue: &mut VecDeque<Location>, result: &mut u32) {
    let next_location_value = map[location.y][location.x] + 1;
    
    // Check up
    if location.y > 0 {
        if map[location.y-1][location.x] == next_location_value {
            let next_location = Location {
                y:location.y-1,
                x:location.x,
            };
            if next_location_value == 9 {
                *result += 1; 
            }
            else {
                queue.push_back(next_location)
            }
        }
    } 
    // Check right
    if location.x < (map[0].len()-1) {
        if map[location.y][location.x+1] == next_location_value {
            let next_location = Location {
                y:location.y,
                x:location.x+1,
            };
            if next_location_value == 9 {
                *result += 1;
            }
            else {
                queue.push_back(next_location)
            }
        }
    } 
    // Check down
    if location.y < map.len()-1 {
        if map[location.y+1][location.x] == next_location_value {
            let next_location = Location {
                y:location.y+1,
                x:location.x,
            };
            if next_location_value == 9 {
                *result += 1;
            }
            else {
                queue.push_back(next_location)
            }
        }
    } 
    // Check left
    if location.x > 0 {
        if map[location.y][location.x-1] == next_location_value {
            let next_location = Location {
                y:location.y,
                x:location.x-1,
            };
            if next_location_value == 9 {
                *result += 1;
            }
            else {
                queue.push_back(next_location)
            }
        }
    } 
}

fn day10_1(filename: &str) {
    let mut map: Vec<Vec<i8>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        let mut row = 0;
        for line in lines.flatten() {
            map.push(Vec::new());
            for kar in line.chars() {
                map[row].push(char_to_i8(kar));
            }
            row += 1;
        }
    }
    let mut start_queue: VecDeque<Location> = VecDeque::new();
    let mut total = 0;
    find_start_locations(&map, &mut start_queue);
    while let Some(location) = start_queue.pop_back() {
        let mut queue: VecDeque<Location> = VecDeque::new();
        let mut result: Vec<Location> = Vec::new();
//        println!("Start location {:?}", location);
        queue.push_back(location);
        while let Some(location) = queue.pop_back() {
            find_next_locations_1(&map, &location, &mut queue, &mut result);
        }
        total += result.len() as u32;
//        println!("Result {:?}", result);
    }
    println!("Total result {}", total);
}

fn day10_2(filename: &str) {
    let mut map: Vec<Vec<i8>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        let mut row = 0;
        for line in lines.flatten() {
            map.push(Vec::new());
            for kar in line.chars() {
                map[row].push(char_to_i8(kar));
            }
            row += 1;
        }
    }
    let mut start_queue: VecDeque<Location> = VecDeque::new();
    let mut total = 0;
    find_start_locations(&map, &mut start_queue);
    while let Some(location) = start_queue.pop_back() {
        let mut queue: VecDeque<Location> = VecDeque::new();
        let mut result: u32 = 0;
        queue.push_back(location);
        while let Some(location) = queue.pop_back() {
            find_next_locations_2(&map, &location, &mut queue, &mut result);
        }
        total += result;
    }
    println!("Total result {}", total);
}

fn main() {
    day10_1("test1.txt");
    day10_1("test2.txt");
    day10_1("test3.txt");
    day10_1("test4.txt");
    day10_1("test5.txt");
    day10_1("input.txt");
    day10_2("test6.txt");
    day10_2("test7.txt");
    day10_2("test8.txt");
    day10_2("test9.txt");
    day10_2("input.txt");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}