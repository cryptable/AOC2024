use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, Debug)]
struct FilePart {
    id: u32,
    pos: usize,
    length: usize,
}

const EMPTY_FILE: u32 = u32::MAX; 

fn find_file_part(file_map: &HashMap<u32,VecDeque<FilePart>>, position: usize) -> Option<FilePart> {
    for file_parts in file_map.values() {
        for file_part in file_parts {
            if file_part.pos == position {
                return Some(FilePart {
                    id: file_part.id,
                    pos: file_part.pos,
                    length: file_part.length,
                })
            }    
        }
    }
    None
}

fn day9_1(filename: &str) {
    let var_name = HashMap::new();
    let mut file_map: HashMap<u32,VecDeque<FilePart>> = var_name;
    let mut file_id = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut position = 0;
            for (index, kar) in line.chars().enumerate() {
                let file_len = kar.to_string().parse::<usize>().unwrap();
                if index % 2 == 1 {
                    if !file_map.contains_key(&EMPTY_FILE) {
                        file_map.insert(EMPTY_FILE, VecDeque::new());
                    }
                    if file_len != 0 {
                        file_map.get_mut(&EMPTY_FILE).unwrap().push_back(
                            FilePart {
                                id: EMPTY_FILE,
                                pos: position,
                                length: file_len
                            }
                        )    
                    }
                }
                else {
                    if file_len != 0 {
                        file_map.insert(file_id, VecDeque::new());
                        file_map.get_mut(&file_id).unwrap().push_back(
                            FilePart {
                                id: file_id,
                                pos: position,
                                length: file_len
                            }
                        );    
                    }
                    file_id += 1;
                }
                position += file_len;
            }
        }
    }
//    println!("{}: Original {:?}",filename, file_map);
    let mut id = file_map.keys().len() - 2;
    let mut empty_file_locs = file_map.get(&EMPTY_FILE).unwrap().clone();
    let mut empty_part = empty_file_locs.pop_front().unwrap();
    println!("{} max file id {}",filename, id);
    loop {
//        println!("{} processing id {}",filename, id);
        let file_part = file_map.get_mut(&(id as u32)).unwrap().pop_back().unwrap();
        if file_part.pos < empty_part.pos {
            file_map.get_mut(&(id as u32)).unwrap().push_back(FilePart {
                id: file_part.id,
                pos: file_part.pos,
                length: file_part.length,
            });
            break;
        }
        if empty_part.length >= file_part.length {
            file_map.get_mut(&(id as u32)).unwrap().push_back(FilePart {
                id: file_part.id,
                pos: empty_part.pos,
                length: file_part.length,
            });
            empty_part.length -= file_part.length;
            empty_part.pos += file_part.length;
            if empty_part.length == 0 {
                if empty_file_locs.len() == 0 {
                    break;
                }
                empty_part = empty_file_locs.pop_front().unwrap();
            }
            id -= 1;
        }
        else {
            file_map.get_mut(&(id as u32)).unwrap().push_back(FilePart {
                id: file_part.id,
                pos: empty_part.pos,
                length: empty_part.length,
            });
            file_map.get_mut(&(id as u32)).unwrap().push_back(FilePart {
                id: file_part.id,
                pos: file_part.pos,
                length: file_part.length - empty_part.length,
            });
            if empty_file_locs.len() == 0 {
                break;
            }
            empty_part = empty_file_locs.pop_front().unwrap();
        }
    }
    // println!("{} finished at {}",filename, id);
    file_map.remove(&EMPTY_FILE);
    let mut file_positions: Vec<FilePart> = Vec::new();
    for value in file_map.values() {
        for part in value {
            file_positions.push(*part);
        }
    }
    file_positions.sort_by(compate_file_positions);
    let mut result: u64 = 0;
    for file_pos in &file_positions {
        for i in file_pos.pos..(file_pos.pos + file_pos.length) {
            result += ((i as u32) *file_pos.id) as u64;
        }
    }
//    println!("{} Positions {:?}",filename, file_positions);
    println!("{} Result {:?}", filename, result);
}

fn compare_file_id(a: &FilePart, b: &FilePart) -> std::cmp::Ordering {
    if a.id < b.id {
        return std::cmp::Ordering::Greater;
    }
    if a.id == b.id {
        return std::cmp::Ordering::Equal;
    }
    return std::cmp::Ordering::Less;
}

fn compate_file_positions(a: &FilePart, b: &FilePart) -> std::cmp::Ordering {
    if a.pos < b.pos {
        return std::cmp::Ordering::Less;
    }
    if a.pos == b.pos {
        return std::cmp::Ordering::Equal;
    }
    return std::cmp::Ordering::Greater;
}

fn day9_2(filename: &str) {
    let mut file_map: HashMap<u32,VecDeque<FilePart>> = HashMap::new();
    let mut file_id = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut position = 0;
            for (index, kar) in line.chars().enumerate() {
                let file_len = kar.to_string().parse::<usize>().unwrap();
                if index % 2 == 1 {
                    if !file_map.contains_key(&EMPTY_FILE) {
                        file_map.insert(EMPTY_FILE, VecDeque::new());
                    }
                    if file_len != 0 {
                        file_map.get_mut(&EMPTY_FILE).unwrap().push_back(
                            FilePart {
                                id: EMPTY_FILE,
                                pos: position,
                                length: file_len
                            }
                        )    
                    }
                }
                else {
                    if file_len != 0 {
                        file_map.insert(file_id, VecDeque::new());
                        file_map.get_mut(&file_id).unwrap().push_back(
                            FilePart {
                                id: file_id,
                                pos: position,
                                length: file_len
                            }
                        );    
                    }
                    file_id += 1;
                }
                position += file_len;
            }
        }
    }
    // println!("{}: Original {:?}",filename, file_map);
    let mut empty_file_locs = file_map.remove(&EMPTY_FILE).unwrap();
    let mut empty_part = empty_file_locs.pop_front().unwrap();
    let mut to_move_files: Vec<FilePart> = Vec::new();
    for value in file_map.values() {
        to_move_files.push(value[0]);
    }
    to_move_files.sort_by(compare_file_id);
    loop {
        //        println!("{} processing id {}",filename, id);
        let mut remove_id = 0;
        let mut moved_file = false;
        for id in 0..to_move_files.len() {
            // println!("{} Testing part {:?} to {:?}", filename, to_move_files[id], empty_part);
            if to_move_files[id].length <= empty_part.length {
                if to_move_files[id].pos >= empty_part.pos {
                    // println!("{} Moving part {:?} to {:?}", filename, to_move_files[id], empty_part);
                    file_map.get_mut(&to_move_files[id].id).unwrap()[0].pos = empty_part.pos;
                    remove_id = id;
                    moved_file = true;
                    if empty_part.length > to_move_files[id].length {
                        empty_part.pos += to_move_files[id].length;
                        empty_part.length -= to_move_files[id].length;
                    }
                    else {
                        empty_part.length = 0;
                    }
                    break;
                }
            }
        }
        if moved_file {
            to_move_files.remove(remove_id);
        }
        else {
            if empty_file_locs.len() == 0 {
                break;
            }
            empty_part = empty_file_locs.pop_front().unwrap();
        }
    }
//    println!("{}: Result {:?}",filename, file_map);
    let mut file_positions: Vec<FilePart> = Vec::new();
    for value in file_map.values() {
        for part in value {
            file_positions.push(*part);
        }
    }
    file_positions.sort_by(compate_file_positions);

    let mut result: u64 = 0;
    for file_pos in &file_positions {
        for i in file_pos.pos..(file_pos.pos + file_pos.length) {
            result += ((i as u32) *file_pos.id) as u64;
        }
    }
//    println!("{} Positions {:?}",filename, file_positions);
    println!("{} Result {:?}", filename, result);
}

fn main() {
    day9_1("test1.txt");
    day9_1("test2.txt");
    day9_1("input.txt");
    day9_2("test1.txt");
    day9_2("test2.txt");
    day9_2("input.txt");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}