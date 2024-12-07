use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn verify_rules(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<i32>) -> i32 {
    for i in 0..(updates.len()-1) {
        for j in (i+1)..updates.len() {
            if !rules.contains_key(&updates[i]) {
                return 0;
            }
            if !rules[&updates[i]].contains(&updates[j]){
                return 0;
            }
        }
    }
    return updates[updates.len()/2];
}

fn day5_1(filename: &str) {
    let mut result = 0;
    let mut rules = HashMap::<i32,Vec<i32>>::new();
    if let Ok(mut lines) = read_lines(filename) {
        // Load Rules
        while let Some(line) = lines.next() {
            let rule_line = line.unwrap();
            if rule_line == "" {
                break;
            }
            let rule: Vec<i32> = rule_line.split("|").map(|nbr| nbr.parse::<i32>().unwrap()).collect();
            if !rules.contains_key(&rule[0]) {
                rules.insert(rule[0], Vec::new());
            }
            rules.get_mut(&rule[0]).unwrap().push(rule[1]);
        }

        // Load Updates
        while let Some(line) = lines.next() {
            let updates_line = line.unwrap();
            let updates: Vec<i32> = updates_line
                .split(",")
                .map(|nbr| nbr.parse::<i32>().unwrap())
                .collect();
            let ret = verify_rules(&rules, &updates);
            if ret > 0 {
                println!("Updates correct {:?}", updates)
            }
            result += ret;
        }
        println!("Result {}", result);
    }

}

fn verify_update_rules(rules: &HashMap<i32, Vec<i32>>, updates: &mut Vec<i32>) -> i32 {
    let mut update_needed = false;

    for i in 0..(updates.len()-1) {
        for j in (i+1)..updates.len() {
            if rules.contains_key(&updates[i]) {
                if rules[&updates[i]].contains(&updates[j]) {
                    continue;
                }
                let old = updates[j];
                updates[j] = updates[i];
                updates[i] = old;
                update_needed = true;
            }
            else {
                let old = updates[j];
                updates[j] = updates[i];
                updates[i] = old;
                update_needed = true;
            }
        }
    }
    if update_needed {
        return updates[updates.len()/2];
    }

    0
}

fn day5_2(filename: &str) {
    let mut result = 0;
    let mut rules = HashMap::<i32,Vec<i32>>::new();
    if let Ok(mut lines) = read_lines(filename) {
        // Load Rules
        while let Some(line) = lines.next() {
            let rule_line = line.unwrap();
            if rule_line == "" {
                break;
            }
            let rule: Vec<i32> = rule_line.split("|").map(|nbr| nbr.parse::<i32>().unwrap()).collect();
            if !rules.contains_key(&rule[0]) {
                rules.insert(rule[0], Vec::new());
            }
            rules.get_mut(&rule[0]).unwrap().push(rule[1]);
        }

        // Load Updates
        while let Some(line) = lines.next() {
            let updates_line = line.unwrap();
            let mut updates: Vec<i32> = updates_line
                .split(",")
                .map(|nbr| nbr.parse::<i32>().unwrap())
                .collect();
            let ret = verify_update_rules(&rules, &mut updates);
            if ret > 0 {
                println!("Updates needed {:?}", updates)
            }
            result += ret;
        }
        println!("Result {}", result);
    }

}

fn main() {
    day5_1("test1.txt");
    day5_1("input.txt");
    day5_2("test1.txt");
    day5_2("input.txt");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}