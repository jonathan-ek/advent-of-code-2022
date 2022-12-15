use std::{cmp, fs};
use std::collections::HashSet;

pub fn main() {
    let file_path = "inputs/15.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let data: Vec<((i64, i64), (i64, i64), i64)> = contents
        .split("\n")
        .map(|x| x
            .split(":")
            .map(|y| y
                .split("=")
                .collect()
            )
            .map(|y: Vec<&str>| ((&y[1][..(y[1].len() - 3)]).parse::<i64>().unwrap(), y[2].parse::<i64>().unwrap()))
            .collect()
        ).map(|d: Vec<(i64, i64)>| (d[0], d[1], (d[0].0 - d[1].0).abs() + (d[0].1 - d[1].1).abs())).collect();
    let mut beacons: HashSet<(i64, i64)> = HashSet::new();
    for d in &data {
        beacons.insert(d.1);
    }
    let mut res: Option<(i64, i64)> = None;
    for line in 0..4000000 as i64 {
        let mut r: Vec<(i64, i64)> = Vec::new();
        for d in &data {
            let distance;
            if d.0.1 < line {
                distance = line - d.0.1;
            } else {
                distance = d.0.1 - line;
            }
            if distance <= d.2 {
                let nr = d.2 - distance;
                r.push((d.0.0 - nr, d.0.0 + nr));
            }
        }
        r.sort();
        if r[0].0 > 0 && !beacons.contains(&(0, line)) {
            res = Some((0, line));
            break;
        }
        let mut end = r[0].1;
        for x in 0..r.len() - 1 {
            if end < r[x + 1].0 && !beacons.contains(&(end + 1, line)) {
                res = Some((end + 1, line));
                break;
            }
            end = cmp::max(end, r[x + 1].1);
        }
        if res.is_some() {
            break;
        }
        if end < 4000000 && !beacons.contains(&(4000000, line)) {
            res = Some((4000000, line));
            break;
        }
    }
    let result = res.unwrap();
    println!("{}", result.0 * 4000000 + result.1);
}

