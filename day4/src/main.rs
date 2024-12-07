use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Direction {
    Up = 0,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
    Right,
    UpRight,
}

fn test_pattern(r: usize, c: usize, dir: Direction, xmas_table: &Vec<Vec<char>>, pattern: &Vec<char>) -> i32 {
    let mut row = r as i32;
    let mut col = c as i32;

    for kar in pattern {
        if row < 0 {
            return 0;
        }
        if col < 0 {
            return 0;
        }
        if row >= xmas_table.len() as i32 {
            return 0;
        }
        if col >= xmas_table[0].len() as i32 {
            return 0;
        }
        if xmas_table[row as usize][col as usize] != *kar {
            return 0
        }
        match dir {
            Direction::Up => row -=1,
            Direction::UpLeft => {
                row -=1;
                col +=1;
            },
            Direction::Left => col += 1,
            Direction::DownLeft => {
                row +=1;
                col +=1;
            },
            Direction::Down => row +=1,
            Direction::DownRight => {
                row +=1;
                col -=1;
            },
            Direction::Right => col -=1,
            Direction::UpRight =>  {
                row -=1;
                col -=1;
            }
        }
    }
    return 1
}

fn test_up(r: usize, c: usize, xmas_table: &Vec<Vec<char>>, pattern: &Vec<char>) -> i32 {
    return test_pattern(r, c, Direction::Up, xmas_table, pattern)
}

fn test_up_left(r: usize, c: usize, xmas_table: &Vec<Vec<char>>, pattern: &Vec<char>) -> i32 {
    return test_pattern(r, c, Direction::UpLeft, xmas_table, pattern)
}

fn test_left(r: usize, c: usize, xmas_table: &Vec<Vec<char>>, pattern: &Vec<char>) -> i32 {
    return test_pattern(r, c, Direction::Left, xmas_table, pattern)
}

fn test_down_left(r: usize, c: usize, xmas_table: &Vec<Vec<char>>, pattern: &Vec<char>) -> i32 {
    return test_pattern(r, c, Direction::DownLeft, xmas_table, pattern)
}

fn test_down(r: usize, c: usize, xmas_table: &Vec<Vec<char>>, pattern: &Vec<char>) -> i32 {
    return test_pattern(r, c, Direction::Down, xmas_table, pattern)
}

fn test_down_right(r: usize, c: usize, xmas_table: &Vec<Vec<char>>, pattern: &Vec<char>) -> i32 {
    return test_pattern(r, c, Direction::DownRight, xmas_table, pattern)
}

fn test_right(r: usize, c: usize, xmas_table: &Vec<Vec<char>>, pattern: &Vec<char>) -> i32 {
    return test_pattern(r, c, Direction::Right, xmas_table, pattern)
}

fn test_up_right(r: usize, c: usize, xmas_table: &Vec<Vec<char>>, pattern: &Vec<char>) -> i32 {
    return test_pattern(r, c, Direction::UpRight, xmas_table, pattern)
}

fn day4_1(filename: &str) {
    let mut xmas_table: Vec<Vec<char>> = Vec::new();
    let pattern = vec!['X','M','A','S'];

    // Load Table
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let row: Vec<char> = line.chars().collect();
            xmas_table.push(row);
        }
    }
    
    // Test the XMAS table
    let mut cnt = 0;
    for r in 0..xmas_table.len() {
        for c in 0..xmas_table[0].len() {
            cnt += test_up(r, c, &xmas_table, &pattern);
            cnt += test_up_left(r, c, &xmas_table, &pattern);
            cnt += test_left(r, c, &xmas_table, &pattern);
            cnt += test_down_left(r, c, &xmas_table, &pattern);
            cnt += test_down(r, c, &xmas_table, &pattern);
            cnt += test_down_right(r, c, &xmas_table, &pattern);
            cnt += test_right(r, c, &xmas_table, &pattern);
            cnt += test_up_right(r, c, &xmas_table, &pattern);
        }
    }
    println!("Count {}", cnt);
}

fn test_x_pattern(r: usize, c: usize, xmas_table: &Vec<Vec<char>>, pattern: &Vec<Vec<char>>) -> i32 {
    for row in 0..pattern.len() {
        for col in 0..pattern[0].len() {
            if pattern[row][col] != '.' {
                if xmas_table[r + row][c + col] != pattern[row][col] {
                    return 0
                }    
            }
        }    
    }

//    println!("Found {}:{} {:?}", r, c, pattern);

    return 1
}

fn day4_2(filename: &str) {
    let mut xmas_table: Vec<Vec<char>> = Vec::new();
    let pattern1 = vec![
        vec!['M','.','M'],
        vec!['.','A','.'],
        vec!['S','.','S']];
    let pattern2 = vec![
        vec!['M','.','S'],
        vec!['.','A','.'],
        vec!['M','.','S']];
    let pattern3 = vec![
        vec!['S','.','S'],
        vec!['.','A','.'],
        vec!['M','.','M']];
    let pattern4 = vec![
        vec!['S','.','M'],
        vec!['.','A','.'],
        vec!['S','.','M']];
                    
    // Load Table
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let row: Vec<char> = line.chars().collect();
            xmas_table.push(row);
        }
    }
    
    // Test the XMAS table
    let mut cnt = 0;
    for r in 0..(xmas_table.len() - pattern1.len() + 1)  {
        for c in 0..(xmas_table[0].len() - pattern1[0].len() + 1) {
            cnt += test_x_pattern(r, c, &xmas_table, &pattern1);
            cnt += test_x_pattern(r, c, &xmas_table, &pattern2);
            cnt += test_x_pattern(r, c, &xmas_table, &pattern3);
            cnt += test_x_pattern(r, c, &xmas_table, &pattern4);
        }
    }
    println!("Count {}", cnt);
}

fn main() {
    day4_1("test1.txt");
    day4_1("input.txt");
    day4_2("test1.txt");
    day4_2("test2.txt");
    day4_2("input.txt");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}