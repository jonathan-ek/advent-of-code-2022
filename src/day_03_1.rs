use std::fs;

pub fn main() {
    let file_path = "inputs/03.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let rucksacks = contents.split("\n");
    let mut sum = 0;
    for rucksack in rucksacks {
        let items: Vec<char> = rucksack.chars().collect();
        let middle:usize = (items.len() / 2) as usize;
        let compartment_1:Vec<char> = items[..middle].to_vec();
        let compartment_2:Vec<char> = items[middle..].to_vec();
        for i in compartment_1 {
            if compartment_2.contains(&i) {
                let v1 = i as i32 - 'a' as i32 + 1;
                let v2 = i as i32 - 'A' as i32 + 1;
                if v2 > 28 {
                    // println!("{}", v1);
                    sum += v1;
                } else {
                    // println!("{}", v2+26);
                    sum += v2+26;
                }
                // println!("{}\n", i);
                break
            }
        }
    }
    println!("{}", sum);
}
