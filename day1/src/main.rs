use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn day1_1(filename: &str) {
    let mut vec_left: Vec<i32> = Vec::new();
    let mut vec_right: Vec<i32> = Vec::new();
    
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
//            println!("{}", line);
            let mut nbrs = line.split_whitespace();
            vec_left.push(nbrs.next().unwrap().parse::<i32>().unwrap());
            vec_right.push(nbrs.next().unwrap().parse::<i32>().unwrap());
        }
        vec_left.sort();
        vec_right.sort();
        let mut result = 0;
        println!("length: {}", vec_right.len());
        for i in 0..vec_left.len() {
            result += (vec_left[i] - vec_right[i]).abs()
        }
        println!("{}", result);
    }
    else {
        println!("File not found");
    }
}

fn day1_2(filename: &str) {
    let mut vec_left: Vec<i32> = Vec::new();
    let mut vec_right: Vec<i32> = Vec::new();
    
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
//            println!("{}", line);
            let mut nbrs = line.split_whitespace();
            vec_left.push(nbrs.next().unwrap().parse::<i32>().unwrap());
            vec_right.push(nbrs.next().unwrap().parse::<i32>().unwrap());
        }
//        vec_left.sort();
//        vec_right.sort();
        let mut result = 0;
        println!("length: {}", vec_right.len());
        for i in 0..vec_left.len() {
            let mut cnt = 0;
            for j in 0..vec_right.len() {
                if vec_left[i] == vec_right[j] {
                    cnt += 1;
                }
            }
            result += vec_left[i] * cnt;
        }
        println!("{}", result);
    }
    else {
        println!("File not found");
    }
}

 fn main() {
    day1_1("test_1.txt");
    day1_1("input_1.txt");
    day1_2("test_2.txt");
    day1_2("input_1.txt");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

