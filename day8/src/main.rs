use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn create_anti_node(pnt1: &Point, pnt2: &Point) -> Point {
    return Point {
        x: (pnt2.x + pnt2.x - pnt1.x),
        y: (pnt2.y + pnt2.y - pnt1.y)
    };
}

fn determine_anti_nodes_1(pnt1: Point, pnt2: Point, max_x: i64, max_y: i64) -> Vec<Point> {
    let mut nodes: Vec<Point> = Vec::new();
    let anti_node1 = create_anti_node(&pnt1, &pnt2);
    let anti_node2 = create_anti_node(&pnt2, &pnt1);
    if anti_node1.x >= 0 && anti_node1.x < max_x &&
       anti_node1.y >= 0 && anti_node1.y < max_y {
        nodes.push(anti_node1);
    }
    if anti_node2.x >= 0 && anti_node2.x < max_x &&
       anti_node2.y >= 0 && anti_node2.y < max_y {
        nodes.push(anti_node2);
    }
    nodes
}

fn day8_1(filename: &str) {
    let mut nodes: HashMap<char, Vec<Point>> = HashMap::new();
    if let Ok(lines) = read_lines(filename) {
        let mut y: i64 = 0;
        let mut max_x: i64 = 0;
        let mut max_y: i64 = 0;
        for line in lines.flatten() {
            max_x = 0;
            for (index, kar) in line.chars().enumerate() {
                if kar != '.' {
                    if !nodes.contains_key(&kar) {
                        nodes.insert(kar, Vec::new());                        
                    }
                    nodes.get_mut(&kar).unwrap().push(Point{x:index as i64, y});
                }
                max_x += 1;
            }
            max_y += 1;
            y += 1;
        }
//        println!("{:?}", nodes);
        let mut antinodes: Vec<Point> = Vec::new();
        for kar in nodes.keys() {
            for i in 0..nodes[kar].len()-1 {
                for j in (i+1)..nodes[kar].len() {
                    let possible_nodes = determine_anti_nodes_1(nodes[kar][i], nodes[kar][j], max_x, max_y);
                    for possible_node in possible_nodes {
                        if !antinodes.contains(&possible_node) {
                            antinodes.push(possible_node);
                        }
                    }
                }
            }
        }
//        println!("{:?}", antinodes);
        println!("Number of antinodes = {}", antinodes.len());
    }
}

fn valid_node(pnt: Point, max_x: i64, max_y: i64) -> bool {
    if pnt.x >= 0 && pnt.x < max_x &&
       pnt.y >= 0 && pnt.y < max_y {
        return true
    }
    return false
}

fn create_nodes(pnt1: Point, pnt2: Point, max_x: i64, max_y: i64) -> Vec<Point> {
    let delta = ((pnt2.x - pnt1.x),(pnt2.y-pnt1.y));
    let mut new_node = Point {x:pnt2.x + delta.0, y:pnt2.y + delta.1};
    let mut nodes: Vec<Point> = Vec::new();

    while valid_node(new_node, max_x, max_y) {
        nodes.push(new_node);
        new_node = Point {x:new_node.x + delta.0, y:new_node.y + delta.1};
    }

    return nodes;
}

fn determine_anti_nodes_2(pnt1: Point, pnt2: Point, max_x: i64, max_y: i64) -> Vec<Point> {
    let mut nodes: Vec<Point> = Vec::new();
    nodes.push(pnt1);
    nodes.push(pnt2);
    nodes.append(&mut create_nodes(pnt1, pnt2, max_x, max_y));
    nodes.append(&mut create_nodes(pnt2, pnt1, max_x, max_y));
    
    nodes
}

fn day8_2(filename: &str) {
    let mut nodes: HashMap<char, Vec<Point>> = HashMap::new();
    if let Ok(lines) = read_lines(filename) {
        let mut y: i64 = 0;
        let mut max_x: i64 = 0;
        let mut max_y: i64 = 0;
        for line in lines.flatten() {
            max_x = 0;
            for (index, kar) in line.chars().enumerate() {
                if kar != '.' {
                    if !nodes.contains_key(&kar) {
                        nodes.insert(kar, Vec::new());                        
                    }
                    nodes.get_mut(&kar).unwrap().push(Point{x:index as i64, y});
                }
                max_x += 1;
            }
            max_y += 1;
            y += 1;
        }
//        println!("{:?}", nodes);
        let mut antinodes: Vec<Point> = Vec::new();
        for kar in nodes.keys() {
            for i in 0..nodes[kar].len()-1 {
                for j in (i+1)..nodes[kar].len() {
                    let possible_nodes = determine_anti_nodes_2(nodes[kar][i], nodes[kar][j], max_x, max_y);
                    for possible_node in possible_nodes {
                        if !antinodes.contains(&possible_node) {
                            antinodes.push(possible_node);
                        }
                    }
                }
            }
        }
//        println!("{:?}", antinodes);
        println!("Number of antinodes = {}", antinodes.len());
    }
}

fn main() {
    day8_1("test1.txt");
    day8_1("input.txt");
    day8_2("test1.txt");
    day8_2("test2.txt");
    day8_2("input.txt");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}