use std::{fs};
use std::collections::HashMap;

pub struct Node {
    height: i32,
    coord: (usize, usize),
    connections: Vec<(usize, usize)>,
    value: i32,
    previous_node: (usize, usize),
}

pub fn main() {
    let file_path = "inputs/12.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut nodes: HashMap<(usize, usize), Node> = HashMap::new();
    let mut start: (usize, usize) = (0, 0);
    let mut rows: Vec<Vec<char>> = contents.split("\n").map(|x| x.chars().collect()).collect();
    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            if rows[y][x] == 'S' {
                rows[y][x] = 'a';
            } else if rows[y][x] == 'E' {
                rows[y][x] = 'z';
                start = (x, y);
            }
        }
    }
    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            let height = rows[y][x] as i32;
            let mut connections: Vec<(usize, usize)> = Vec::new();
            if y > 0 && (height - 1) <= (rows[y - 1][x] as i32) {
                connections.push((x, y - 1));
            }
            if y < (rows.len() - 1) && (height - 1) <= (rows[y + 1][x] as i32) {
                connections.push((x, y + 1));
            }
            if x > 0 && (height - 1) <= (rows[y][x - 1] as i32) {
                connections.push((x - 1, y));
            }
            if x < (rows[y].len() - 1) && (height - 1) <= (rows[y][x + 1] as i32) {
                connections.push((x + 1, y));
            }
            nodes.insert((x, y), Node {
                height,
                coord: (x, y),
                connections,
                value: i32::MAX,
                previous_node: (i32::MAX as usize, i32::MAX as usize),
            });
        }
    }

    // println!("{:?}", start);
    nodes.get_mut(&start).unwrap().value = 0;
    let mut unvisited_nodes: Vec<(usize, usize)> = nodes.values().map(|x| x.coord).collect();

    while unvisited_nodes.len() != 0 {
        let mut current_min_node_coord = unvisited_nodes[0];
        for node in &unvisited_nodes {
            let n = nodes.get(&node).unwrap();
            let current_node = nodes.get(&current_min_node_coord).unwrap();
            if n.value < current_node.value {
                current_min_node_coord = *node;
            }
        }
        let current_min_node = nodes.get(&current_min_node_coord).unwrap();
        if current_min_node.value == i32::MAX {
            break;
        }
        let tentative_value = current_min_node.value + 1;
        for neighbor in &(&current_min_node).connections.clone() {
            let mut n = nodes.get_mut(&neighbor).unwrap();
            if tentative_value < n.value {
                n.value = tentative_value;
                n.previous_node = current_min_node_coord;
            }
        }
        let index = unvisited_nodes.iter().position(|x| *x == current_min_node_coord).unwrap();
        unvisited_nodes.remove(index);
    }
    let lowest_node: i32 = *nodes.values().filter(|x| x.height == 'a' as i32).map(|x| x.value).collect::<Vec<i32>>().iter().min().unwrap();

    println!("{:?}", lowest_node);
    // let mut p = nodes.get(&end).unwrap();
    // while p.coord != start {
    //     println!("{:?}, {}, {}, {:?}", p.coord, p.value, p.height, p.connections);
    //     p = nodes.get(&p.previous_node).unwrap();
    // }
}
