use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct TestLine {
    total: u64,
    numbers: Vec<u64>,
}

impl TestLine {
    fn from(line: &str) -> TestLine {
        let split1 : Vec<&str> = line.split(":").collect();
        let total = split1[0].parse::<u64>().unwrap();
        let split2 = split1[1].split_whitespace();
        let mut numbers: Vec<u64> = Vec::new();
        for nbr in split2 {
            numbers.push(nbr.parse::<u64>().unwrap());
        }
        TestLine {
            total: total,
            numbers: numbers
        }
    }
}

fn number_concat(number1: u64, number2: u64) -> u64 {
    let mut exp: u64 = 10;
    while exp < number2 {
        exp *= 10;
    }
    number1*exp + number2
}

fn verify_line_1(total: u64, subtotal: u64, index: usize, numbers: &Vec<u64>) -> bool {
//    println!("INFO: {}, {}, {}, {:?}",total, subtotal, index, numbers);
    if total == subtotal {
        return true
    }
    if index >= numbers.len() {
        return false
    }
    if subtotal > total {
        return false
    }
    return verify_line_1(total, subtotal * numbers[index], index + 1, numbers) ||
        verify_line_1(total, subtotal + numbers[index], index + 1, numbers);
}

fn verify_line_2(total: u64, subtotal: u64, index: usize, numbers: &Vec<u64>) -> bool {
    //    println!("INFO: {}, {}, {}, {:?}",total, subtotal, index, numbers);
        if total == subtotal {
            return true
        }
        if index >= numbers.len() {
            return false
        }
        if subtotal > total {
            return false
        }
        return verify_line_2(total, subtotal * numbers[index], index + 1, numbers) ||
            verify_line_2(total, subtotal + numbers[index], index + 1, numbers) ||
            verify_line_2(total, number_concat(subtotal, numbers[index]), index + 1, numbers);
}

fn day7_1(filename: &str) {
    if let Ok(lines) = read_lines(filename) {
        let mut result = 0;
        for line in lines.flatten() {
            let test = TestLine::from(&line);
            // print!("Test line {:?}", test);
            if verify_line_1(test.total, 0, 0, &test.numbers) {
                result += test.total;
                // println!("--> Good!")
            }
            else {
                // println!("--> Bad!")    
            }
        }
        println!("Result = {}", result);
    }
}   

fn day7_2(filename: &str) {
    if let Ok(lines) = read_lines(filename) {
        let mut result = 0;
        for line in lines.flatten() {
            let test = TestLine::from(&line);
            // print!("Test line {:?}", test);
            if verify_line_2(test.total, 0, 0, &test.numbers) {
                result += test.total;
                // println!("--> Good!")
            }
            else {
                // println!("--> Bad!")    
            }
        }
        println!("Result = {}", result);
    }
}   

fn main() {
    day7_1("test1.txt");
    day7_1("input.txt");
    let test_concat = number_concat(56, 9087);
    println!("number concat {}", test_concat);
    day7_2("test1.txt");
    day7_2("input.txt");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}