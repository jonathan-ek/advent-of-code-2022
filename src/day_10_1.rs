use std::fs;

pub fn main() {
    let file_path = "inputs/10.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let data: Vec<&str> = contents.split("\n").collect();
    let mut i = 0;
    let mut a = 1;
    let mut sum = 0;
    for d in data {

        i += 1;
        if (i + 20) % 40 == 0 {
            println!("{}, {}", i, a);
            sum += i*a;
        }

        if d.contains("addx") {
            let val = d.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

            i += 1;
            if (i + 20) % 40 == 0 {
                println!("{}, {}", i, a);
                sum += i*a;
            }
            a += val;
        }
        // println!("{}, {}", i, a);
    }
    println!("{}", sum);
}
