use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::result;

fn next
fn length_of_fence(begin: &(usize, usize), kar: char, map: &Vec<Vec<char>>) {
    let cur = *begin;
    while ()
}

fn day12_1(filename: &str) {
    let mut plant_types: HashMap<char, Vec<(usize,usize)>> = HashMap::new();
    let mut plant_map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for (i, line) in lines.enumerate() {
            match line {
                Ok(line) => {
                    if (i==0) {
                        plant_map.push(Vec::new());
                        for f in 0..(line.len()+2) {
                            plant_map[0].push('*');
                        }
                    }
                    plant_map.push(Vec::new());
                    plant_map[i+1].push('*');
                    for (j, kar) in line.chars().enumerate() {
                        if !plant_types.contains_key(&kar) {
                            plant_types.insert(kar, Vec::new());
                        }
                        plant_types.get_mut(&kar).unwrap().push((i+1,j+1));
                        plant_map[i+1].push(kar);
                    }        
                    plant_map[i+1].push('*');
                },
                Err(e) => panic!("{}", e)
            }
        }
        plant_map.push(Vec::new());
        for l in 0..plant_map[0].len() {
            plant_map[plant_map.len()].push('*');
        }
    }
    let mut result = 0;
    for kar in plant_types.key() {
        result += length_of_fence(&plant_types[kar], kar, &plant_map);
    }
}

fn main() {
    day12_1("test1.txt");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}