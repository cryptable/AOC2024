use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_valid_delta(number1: i32, number2: i32) -> bool {
    let delta = (number1 - number2).abs();
    return delta >= 1 && delta <= 3
}

fn is_safe_1(numbers: Vec<&str>) -> bool {
    if numbers.len() < 2 {
        return false
    }
    if numbers.len() == 2 {
        return is_valid_delta(numbers[0].parse::<i32>().unwrap(),numbers[1].parse::<i32>().unwrap())
    }
    let mut prev_nbr = numbers[0].parse::<i32>().unwrap();
    let cur_nbr = numbers[1].parse::<i32>().unwrap();
    println!("Checking {} vs {}", cur_nbr, prev_nbr);
    if !is_valid_delta(cur_nbr, prev_nbr) {
        println!("invalid delta");
        return false
    }
    let increasing = cur_nbr > prev_nbr;
    prev_nbr = cur_nbr;
    for i in 2..numbers.len() {
        let cur_nbr = numbers[i].parse::<i32>().unwrap();
        println!("Checking {} vs {}", cur_nbr, prev_nbr);
        if increasing {
            if cur_nbr < prev_nbr {
                println!("stop increasing");
                return false
            }
        }
        else {
            if cur_nbr > prev_nbr {
                println!("stop decreasing");
                return false
            }
        }
        if !is_valid_delta(cur_nbr, prev_nbr) {
            println!("invalid delta");
            return false
        }
        prev_nbr = cur_nbr;
    }

    true
}

fn day2_1(filename: &str) {
    let mut cnt = 0;
    let mut total = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            println!("line={}", line);
            let numbers = line.split_whitespace().collect();
            if is_safe_1(numbers) {
                cnt += 1;
            }
            total +=1;
        }
    }
    println!("Count = {} / {}", cnt, total);
}

fn day2_2(filename: &str) {
    let mut cnt = 0;
    let mut total = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            println!("line={}", line);
            let numbers: Vec<&str> = line.split_whitespace().collect();
            if is_safe_1(numbers.clone()) {
                cnt += 1;
            }
            else {
                for i in 0..numbers.len() {
                    let mut altered_numbers = numbers.clone();
                    altered_numbers.remove(i);
                    if is_safe_1(altered_numbers) {
                        cnt += 1;
                        break;
                    }
                }
            }
            total +=1;
        }
    }
    println!("Count = {} / {}", cnt, total);
}


fn main() {
    day2_1("test_1.txt");
    day2_1("input.txt");
    day2_2("test_1.txt");
    day2_2("input.txt");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
