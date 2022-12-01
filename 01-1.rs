use std::fs;

fn main() {
    let file_path = "01.txt";

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
    let result = elfsum.iter().max().unwrap();
    println!("{}", result);
}