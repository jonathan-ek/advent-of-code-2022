use std::collections::HashSet;
use std::fs;

pub fn main() {
    let file_path = "inputs/09.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let data: Vec<char> = contents
        .split("\n")
        .map(|x| x.split(" ").collect())
        .map(|x: Vec<&str>| x[0].repeat(x[1].parse::<usize>().unwrap()))
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>()
        .into_iter().flatten().collect();
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let mut h_x = 0;
    let mut h_y = 0;
    let mut t_x = 0;
    let mut t_y = 0;
    for d in data {
        if d == 'R' {
            h_x += 1;
        } else if d == 'L' {
            h_x -= 1;
        } else if d == 'U' {
            h_y += 1;
        } else if d == 'D' {
            h_y -= 1;
        }
        if h_x >= t_x + 2 {
            t_x += 1;
            t_y = h_y;
        }
        if h_x <= t_x - 2 {
            t_x -= 1;
            t_y = h_y;
        }
        if h_y >= t_y + 2 {
            t_y += 1;
            t_x = h_x;
        }
        if h_y <= t_y - 2 {
            t_y -= 1;
            t_x = h_x;
        }
        // println!("{:?}, {:?}", t_x, t_y);

        positions.insert((t_x, t_y));
    }
    println!("{:?}", positions.len());
}
