use std::fs;

pub fn main() {
    let file_path = "inputs/08.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let data:Vec<Vec<i32>> = contents.split("\n").map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect()).collect();
    let mut i = 0;
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if x == 0 || y == 0 || x == data[0].len() - 1 || y == data.len() - 1 {
                i += 1;
                continue;
            }
            if (0..x).all(|x0| data[y][x] > data[y][x0]) ||
                ((x + 1)..data[0].len()).all(|x0| data[y][x] > data[y][x0]) ||
                (0..y).all(|y0| data[y][x] > data[y0][x]) ||
                ((y + 1)..data.len()).all(|y0| data[y][x] > data[y0][x]) {
                i += 1;
            }
        }
    }
    println!("{:?}", i);
}
