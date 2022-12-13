use std::{fs};
use json;
use json::{array, JsonValue, Null};

fn compare(left: &JsonValue, right: &JsonValue) -> Option<bool> {
    let mut i = 0;
    loop {
        if left[i].is_number() && right[i].is_number() {
            if left[i].as_i32() < right[i].as_i32() {
                return Some(true);
            } else if left[i].as_i32() > right[i].as_i32() {
                return Some(false);
            }
        }
        if left[i].is_array() && right[i].is_array() {
            let res = compare(&left[i], &right[i]);
            if res.is_some() {
                return res;
            }
        }
        if left[i].is_array() && right[i].is_number() {
            let new_right = array![right[i].as_i32()];
            let res = compare(&left[i], &new_right);
            if res.is_some() {
                return res;
            }
        }
        if left[i].is_number() && right[i].is_array() {
            let new_left = array![left[i].as_i32()];
            let res = compare(&new_left, &right[i]);
            if res.is_some() {
                return res;
            }
        }
        if left[i] == Null && right[i] != Null {
            return Some(true);
        } else if left[i] != Null && right[i] == Null {
            return Some(false);
        }
        if left[i] == Null && right[i] == Null {
            return None;
        }
        i += 1;
    }
}

pub fn main() {
    let file_path = "inputs/13.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let data: Vec<Vec<&str>> = contents.split("\n\n").map(|x| x.split("\n").collect()).collect();
    let mut i = 0;
    let mut sum = 0;
    for d in data {
        let left = json::parse(d[0]).unwrap();
        let right = json::parse(d[1]).unwrap();
        i += 1;
        if compare(&left, &right).unwrap() {
            sum += i;
        }
    }
    println!("{}", sum);
}

