use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use std::fs;

pub fn main() {
    let file_path = "inputs/07.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let rows = contents.split("\n");
    let mut path = VecDeque::new();
    let mut dir_sizes: HashMap<String, i32> = HashMap::new();
    for row in rows {
        let parts: Vec<&str> = row.split(" ").collect();
        let row_type = parts[0];
        let content = parts[1];
        if row_type == "$" {
            if content == "cd" {
                let dir = parts[2];
                if dir == ".." {
                    path.pop_back();
                } else {
                    path.push_back(dir);
                }
                // println!("command: {} {}", content, dir)
            } else {
                // println!("command: {}", content)
            }
        } else if row_type == "dir" {
            // println!("dir: {:?}", content)
        } else {
            let size = row_type.parse::<i32>().unwrap();
            let mut cp = "".to_string();
            for p in &path {
                cp = format!("{}/{}", cp, p);
                let tmp = cp.to_owned();
                // println!("{}", cp);
                *dir_sizes.entry(tmp).or_insert(0) += size
            }
            // println!("size: {}, {:?}", content, size)
        }
    }
    let mut current_smallest = 70000000;
    let min_dir_size = 30000000 - (70000000 - dir_sizes["//"]);
    // println!("{}", min_dir_size);
    for (_key, value) in dir_sizes {
        if value > min_dir_size {
            current_smallest = min(value, current_smallest);
        }
    }
    println!("{}", current_smallest);
}
