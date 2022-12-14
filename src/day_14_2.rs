use std::{fs};
use std::cmp::{max, min};
use std::collections::HashSet;

fn expand(inp: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut acc = Vec::new();
    let mut prev: Option<(i32, i32)> = None;
    for a in inp {
        if prev.is_none() {
            prev = Some(a);
        } else {
            let p = prev.unwrap();
            for x in min(p.0, a.0)..max(p.0, a.0)+1 {
                for y in min(p.1, a.1)..max(p.1, a.1)+1 {
                    acc.push((x, y));
                }
            }
            prev = Some(a);
        }
    }
    acc.sort();
    acc.dedup();
    return acc
}

pub fn main() {
    let file_path = "inputs/14.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut data: Vec<(i32, i32)> = contents
        .split("\n")
        .map(|x| x
            .split(" -> ")
            .map(|y| y
                .split(",")
                .map(|z| z.parse::<i32>().unwrap())
                .collect()
            )
            .map(|y: Vec<i32>| (y[0], y[1]))
            .collect()
        ).map(|x: Vec<(i32, i32)>| expand(x)).flatten()
        .collect();
    data.sort();
    data.dedup();
    let max_y = data.iter().map(|x| x.1).max().unwrap();
    let mut h_data: HashSet<(i32, i32)> = HashSet::from_iter(data.iter().cloned());
    let mut i = 0;
    loop {
        let mut x = 500;
        let mut y = 0;
        loop {
            if !h_data.contains(&(x, y+1)) {
                y += 1;
            } else if !h_data.contains(&(x-1, y+1)) {
                x -= 1;
                y += 1;
            } else if !h_data.contains(&(x+1, y+1)) {
                x += 1;
                y += 1;
            } else {
                h_data.insert((x, y));
                break
            }
            if y >= max_y + 1 {
                h_data.insert((x, y));
                break
            }
        }
        i += 1;
        if y == 0 {
            break
        }
    }

    println!("{:?}", i);
}

