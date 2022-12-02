use std::fs;
use std::collections::HashMap;


pub fn main() {
    let file_path = "inputs/02.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let rounds = contents.split("\n");
    let mut sum = 0;
    let opponent_mapping = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);
    let player_mapping = HashMap::from([
        ('X', HashMap::from([('A', 3), ('B', 1), ('C', 2)])),
        ('Y', HashMap::from([('A', 1), ('B', 2), ('C', 3)])),
        ('Z', HashMap::from([('A', 2), ('B', 3), ('C', 1)])),
    ]);

    for round in rounds {
        let round_list: Vec<char> = round.chars().collect();
        let o = *opponent_mapping.get(&round_list[0]).unwrap();
        let p = *player_mapping.get(&round_list[2]).unwrap().get(&round_list[0]).unwrap();
        sum += p;
        if (o == 1 && p == 2) || (o == 2 && p == 3) || (o == 3 && p == 1) {
            sum += 6;
        } else if o == p {
            sum +=3;
        }
    }
    println!("{}", sum);
}
