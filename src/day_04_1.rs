use std::fs;

fn expand(section: &str) -> Vec<i32> {
    let range_endpoints: Vec<&str> = section.split('-').collect();
    let start_point = range_endpoints[0].parse::<i32>().unwrap();
    let end_point = range_endpoints[1].parse::<i32>().unwrap() + 1;
    return (start_point..end_point).collect();
}

pub fn main() {
    let file_path = "inputs/04.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let section_pairs = contents.split("\n");
    let mut sum = 0;
    for section_pair in section_pairs {
        let sections: Vec<&str> = section_pair.split(',').collect();
        let section_1 = expand(sections[0]);
        let section_2 = expand(sections[1]);
        if section_1.len() < section_2.len() {
             if section_1.iter().all(|&x| section_2.contains(&x)) {
                 sum += 1;
             }
        } else {
             if section_2.iter().all(|&x| section_1.contains(&x)) {
                 sum += 1;
             }
        }
        // println!("{:?}", section_1);
        // println!("{:?}", section_2);
    }
    println!("{}", sum);
}
