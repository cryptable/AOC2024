use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Up = b'^',
    Left = b'>',
    Right = b'<',
    Down = b'v',
}

impl Into<char> for Direction {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl Direction {
    fn from(c: char) -> Result<Direction, ()> {
        match c {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Left),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Right),
            _ => Err(())
        }
    }
}

fn find_guard(guard_map: &Vec<Vec<char>>) -> Result<(usize, usize, Direction), &'static str> {
    for r in 0..guard_map.len() {
        for c in 0..guard_map[0].len() {
            match Direction::from(guard_map[r][c]) {
                Ok(dir) => return Ok((r, c, dir)),
                _ => continue,
            }
        }
    }

    Err("No guard found")
}

fn within_frame(map: &Vec<Vec<char>>, r: i32, c: i32) -> bool {
    if r as usize >= map.len() {
        return false
    }
    if r < 0 {
        return false
    }
    if c as usize >= map[0].len() {
        return false
    }
    if c < 0 {
        return false
    }

    return true;
}

fn move_guard(map: &mut Vec<Vec<char>>, pos: (usize, usize, Direction)) -> Result<(usize, usize, Direction), &'static str> {
    map[pos.0][pos.1] = 'X';
    let mut new_r: i32 = pos.0 as i32;
    let mut new_c: i32 = pos.1 as i32;
    let new_d = pos.2;
    match new_d {
        Direction::Up => {
            new_r -= 1;
        },
        Direction::Left => {
            new_c += 1;
        },
        Direction::Down => {
            new_r += 1;
        },
        Direction::Right => {
            new_c -= 1;
        },
    }
    if !within_frame(map, new_r, new_c) {
        return Err("Outside frame");
    }
    if map[new_r as usize][new_c as usize] == '#' {
        let new_d = match new_d {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        };
        map[pos.0][pos.1] = new_d.into();
        return Ok((pos.0, pos.1, new_d))
    }
    Ok((new_r as usize, new_c as usize, new_d))
}

fn count_distinct_positions(map: &Vec<Vec<char>>) -> i32 {
    let mut cnt = 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == 'X' {
                cnt += 1;
            }
        }
    }
    cnt
}

fn day6_1(filename: &str) {
    let mut guard_map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let row: Vec<char> = line.chars().collect();
            guard_map.push(row);
        }
        let mut guard_pos = find_guard(&guard_map).unwrap();
        println!("Found Guard ({},{},{:?})", guard_pos.0, guard_pos.1, guard_pos.2);
        let mut moves = 0;
        while let Ok(new_pos) = move_guard(&mut guard_map, guard_pos) {
//            println!("New position guard ({},{},{:?})", new_pos.0, new_pos.1, new_pos.2);
            if new_pos.0 != guard_pos.0 || new_pos.1 != guard_pos.1 {
                moves += 1;
            }
            guard_pos = new_pos;
        }
        let pos = count_distinct_positions(&guard_map);
        println!("Distinct positions = {} with moves {}", pos, moves);
    }
}

fn mistery_loop_test(map: &Vec<Vec<char>>, start: &(usize, usize, Direction), block_pos: &(usize, usize)) -> bool {
    let mut test_pos = *start;
    let mut test_map = map.clone();

    test_map[block_pos.0][block_pos.1] = '#';
    let mut new_path: Vec<(usize, usize, Direction)> = Vec::new();
    new_path.push(test_pos);
    while let Ok(new_pos) = move_guard(&mut test_map, test_pos) {
        for path_pos in &new_path {
            if new_pos.0 == path_pos.0 && new_pos.1 == path_pos.1 && new_pos.2 == path_pos.2 {
                return true
            }    
        }
        test_pos = new_pos;
        new_path.push(test_pos);
    }
    return false;
}

fn day6_2(filename: &str) {
    let mut guard_map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let row: Vec<char> = line.chars().collect();
            guard_map.push(row);
        }
        let guard_pos = find_guard(&guard_map).unwrap();
        println!("Found Guard ({},{},{:?})", guard_pos.0, guard_pos.1, guard_pos.2);
        let mut moves = 0;
        let mut original_path: Vec<(usize,usize)> = Vec::new();
        let mut old_pos = guard_pos; 
        while let Ok(new_pos) = move_guard(&mut guard_map, old_pos) {
//            println!("New position ({},{},{:?})", new_pos.0, new_pos.1, new_pos.2);
            if new_pos.0 != old_pos.0 || new_pos.1 != old_pos.1 {
                original_path.push((new_pos.0, new_pos.1));
                moves += 1;
            }
            old_pos = new_pos;
        }
        let pos = count_distinct_positions(&guard_map);
        println!("Distinct positions = {} with moves {}", pos, moves);
        let mut mistery_loop_cnt = 0;
        let mut mistery_blocks: Vec<(usize,usize)> = Vec::new();
        for test_pos in original_path {
            if mistery_loop_test(&guard_map, &guard_pos, &test_pos) {
                if !mistery_blocks.contains(&test_pos) {
                    mistery_blocks.push(test_pos);
                    println!("Mistery position {}:{}", test_pos.0, test_pos.1);
                    mistery_loop_cnt += 1
                }
            }
        }
        println!("Real mistery loops = {}", mistery_loop_cnt);
    }
}

fn main() {
    day6_1("test1.txt");
    day6_1("input.txt");
    day6_2("test1.txt");
    day6_2("input.txt");
    // 1836
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}