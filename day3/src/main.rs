use regex::Regex;
use std::fs;

fn day3_1(multiplier_string: &str) {
    let re = Regex::new(r"mul\((?<m1>[0-9]{1,3}),(?<m2>[0-9]{1,3})\)").unwrap();
    let multipliers: Vec<(i32,i32)> = re.captures_iter(multiplier_string).map(|caps| {
        let m1 = caps.name("m1").unwrap().as_str().parse::<i32>().unwrap();
        let m2 = caps.name("m2").unwrap().as_str().parse::<i32>().unwrap();
        (m1,m2)
    }).collect();

    let mut result = 0;
    for multiplier in multipliers {
        result += multiplier.0 * multiplier.1;
    }
    println!("Result = {}", result);
}

fn day3_2(multiplier_string: &str) {
    let re = Regex::new(r"mul\((?<m1>[0-9]{1,3}),(?<m2>[0-9]{1,3})\)|(?<dont>don't)|(?<do>do[^n])").unwrap();
    let mut no_multiplier = false;
    let multipliers: Vec<(i32,i32)> = re.captures_iter(multiplier_string).map(|caps| {
        match caps.name("dont") {
            Some(_) => {
                no_multiplier = true;
                return (0,0)
            },
            None => ()
        }
        match caps.name("do") {
            Some(_) => {
                no_multiplier = false;
                return (0,0)
            },
            None => ()
        }
        let m1 = caps.name("m1").unwrap().as_str().parse::<i32>().unwrap();
        let m2 = caps.name("m2").unwrap().as_str().parse::<i32>().unwrap();
        if no_multiplier {
            return (0,0)
        }
        (m1,m2)
    }).collect();
    
    println!("multipliers length {}", multipliers.len());

    let mut result = 0;
    for multiplier in multipliers {
        result += multiplier.0 * multiplier.1;
    }
    println!("Result = {}", result);
}


fn main() {
    day3_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    let contents = fs::read_to_string("input.txt").unwrap();
    day3_1(&contents);
    day3_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    day3_2(&contents);
}
 
