use std::fs;

pub fn main() {
    let file_path = "inputs/03.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let rucksacks:Vec<&str> = contents.split("\n").collect();
    let mut sum = 0;
    for team in 0..(rucksacks.len()/3) {
        let items_1: Vec<char> = rucksacks[(3*team)].chars().collect();
        let items_2: Vec<char> = rucksacks[(3*team)+1].chars().collect();
        let items_3: Vec<char> = rucksacks[(3*team)+2].chars().collect();
        for i in items_1 {
            if items_2.contains(&i) && items_3.contains(&i) {
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
