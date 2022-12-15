use std::{fs};
use std::collections::HashSet;

pub fn main() {
    let file_path = "inputs/15.txt";
    let line = 2000000;
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let data: Vec<Vec<(i32, i32)>> = contents
        .split("\n")
        .map(|x| x
            .split(":")
            .map(|y| y
                .split("=")
                .collect()
            )
            .map(|y: Vec<&str>| ((&y[1][..(y[1].len()-3)]).parse::<i32>().unwrap(), y[2].parse::<i32>().unwrap()))
            .collect()
        ).collect();
    let mut coords: HashSet<i32> = HashSet::new();
    let mut beacons: HashSet<i32> = HashSet::new();
    for d in &data {
        if d[1].1 == line {
            beacons.insert(d[1].0);
        }
    }
    for d in &data {
        let m_distance = (d[0].0 - d[1].0).abs() + (d[0].1 - d[1].1).abs();
        let distance;
        if d[0].1 < line {
            distance = line - d[0].1;
        } else {
            distance = d[0].1 - line;
        }
        if distance <= m_distance {
            let nr = m_distance - distance;
            // print!("{:?}, {}; ", d[0], m_distance);
            for i in 0..(nr+1) {
                if !beacons.contains(&(d[0].0 + i)) {
                    coords.insert(d[0].0 + i);
                    // print!("{}, ", d[0].0 + i);
                }
                if !beacons.contains(&(d[0].0 - i))   {
                    coords.insert(d[0].0 - i);
                    // print!("{}, ", d[0].0 - i);
                }
            }
            // print!("\n");
        }
    }
    // !5746540
    println!("{}", coords.len());
}

