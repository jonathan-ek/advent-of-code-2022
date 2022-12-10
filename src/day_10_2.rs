use std::fs;

pub fn main() {
    let file_path = "inputs/10.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let data: Vec<&str> = contents.split("\n").collect();
    let mut i = 0;
    let mut a = 1;
    for d in data {
        if [a-1, a, a+1].contains(&(i % 40)) {
            print!("█");
        } else {
            print!(" ");
        }
        if (i % 40) == 39 {
            print!("\n");
        }
        i += 1;

        if d.contains("addx") {
            let val = d.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

            if [a-1, a, a+1].contains(&(i % 40)) {
                print!("█");
            } else {
                print!(" ");
            }
            if (i % 40) == 39 {
                print!("\n");
            }
            i += 1;
            a += val;
        }
        // println!("{}, {}", i, a);
    }
}
