use std::fs;

pub fn main() {
    let file_path = "inputs/01.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let elfs = contents.split("\n\n");
    let mut elfsum = Vec::new();
    for elf in elfs {
        let rows = elf.split("\n");
        let mut sum = 0;
        for row in rows {
            sum += row.parse::<i32>().unwrap();
        }
        elfsum.push(sum);
    }
    elfsum.sort_by(|a, b| b.cmp(a));

    let result: i32 = elfsum.get(0..3).unwrap().iter().sum();
    println!("{}", result);
}
