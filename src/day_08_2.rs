use std::cmp::max;
use std::fs;

pub fn main() {
    let file_path = "inputs/08.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let data: Vec<Vec<i32>> = contents.split("\n").map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect()).collect();
    let mut max_score = 0;
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if x == 0 || y == 0 || x == data[0].len() - 1 || y == data.len() - 1 {
                continue;
            }
            let mut up = 0;
            let mut down = 0;
            let mut left = 0;
            let mut right = 0;
            for x0 in (0..x).rev() {
                left += 1;
                if data[y][x] <= data[y][x0] {
                    break;
                }
            }
            for x0 in (x + 1)..data[0].len() {
                right += 1;
                if data[y][x] <= data[y][x0] {
                    break;
                }
            }
            for y0 in (0..y).rev() {
                up += 1;
                if data[y][x] <= data[y0][x] {
                    break;
                }
            }
            for y0 in (y + 1)..data.len() {
                down += 1;
                if data[y][x] <= data[y0][x] {
                    break;
                }
            }
            max_score = max(max_score,  up * down * left * right);
        }
    }
    println!("{:?}", max_score);
}
