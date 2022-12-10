use std::collections::HashSet;
use std::fs;
use std::ops::Index;

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

    let mut tails:Vec<(i32, i32)> = [(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)].to_vec();
    for d in data {
        if d == 'R' {
            tails[0].0 += 1;
        } else if d == 'L' {
            tails[0].0 -= 1;
        } else if d == 'U' {
            tails[0].1 += 1;
        } else if d == 'D' {
            tails[0].1 -= 1;
        }
        for t in 1..tails.len() {
            if tails[t-1].0 >= tails[t].0 + 2 {
                tails[t].0 += 1;
                if tails[t-1].1 >= tails[t].1 + 1 {
                    tails[t].1 += 1;
                }
                if tails[t-1].1 <= tails[t].1 - 1 {
                    tails[t].1 -= 1;
                }
            }
            if tails[t-1].0 <= tails[t].0 - 2 {
                tails[t].0 -= 1;
                if tails[t-1].1 >= tails[t].1 + 1 {
                    tails[t].1 += 1;
                }
                if tails[t-1].1 <= tails[t].1 - 1 {
                    tails[t].1 -= 1;
                }
            }
            if tails[t-1].1 >= tails[t].1 + 2 {
                tails[t].1 += 1;
                if tails[t-1].0 >= tails[t].0 + 1 {
                    tails[t].0 += 1;
                }
                if tails[t-1].0 <= tails[t].0 - 1 {
                    tails[t].0 -= 1;
                }
            }
            if tails[t-1].1 <= tails[t].1 - 2 {
                tails[t].1 -= 1;
                if tails[t-1].0 >= tails[t].0 + 1 {
                    tails[t].0 += 1;
                }
                if tails[t-1].0 <= tails[t].0 - 1 {
                    tails[t].0 -= 1;
                }
            }
        }
        // for y in (0..5).rev() {
        //     for x in 0..6 {
        //         if tails.contains(&(x, y)) {
        //             print!("{}", tails.iter().position(|&r| r == (x, y)).unwrap());
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     print!("\n");
        // }
        // print!("\n");
        positions.insert(tails[9]);
    }
    // !2604

    println!("{:?}", positions.len());
}
