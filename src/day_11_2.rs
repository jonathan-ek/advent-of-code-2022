use std::{fmt, fs};

pub struct Monkey {
    nr: usize,
    items: Vec<i64>,
    operation: String,
    condition: i32,
    throw_to_if_true: usize,
    throw_to_if_false: usize,
}


impl Monkey {
    fn new(str_monkey:&str) -> Monkey {
        let parts: Vec<&str> = str_monkey.split("\n").collect();
        let items = parts[1].split(": ").collect::<Vec<&str>>()[1].split(", ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let nr= parts[0].split(" ").collect::<Vec<&str>>()[1].strip_suffix(":").unwrap().parse::<usize>().unwrap();
        let operation = parts[2].split(" = ").collect::<Vec<&str>>()[1];
        let condition = parts[3].split(" by ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let throw_to_if_true = parts[4].split(" ").last().unwrap().parse::<usize>().unwrap();
        let throw_to_if_false = parts[5].split(" ").last().unwrap().parse::<usize>().unwrap();
        return Monkey {
            nr,
            items,
            operation: operation.to_string(),
            condition,
            throw_to_if_true,
            throw_to_if_false,
        };
    }
}



impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monkey {}, [{:?}]", self.nr, self.items)
    }
}

pub fn main() {
    let file_path = "inputs/11.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut monkeys: Vec<Monkey> = contents.split("\n\n").map(|x| Monkey::new(x)).collect();
    let mut inspections: Vec<i64> = Vec::new();
    let mut divisor: i64 = 1;
    for monkey in 0..monkeys.len() {
        inspections.push(0);
        divisor *= monkeys[monkey].condition as i64;
    }
    // println!("{:?}", monkeys);
    for _round in 0..10000 {
        for monkey in 0..monkeys.len() {
            for i in 0..monkeys[monkey].items.len() {
                inspections[monkey] += 1;
                let mut dest = monkeys[monkey].throw_to_if_true;
                let operation: Vec<&str> = monkeys[monkey].operation.split(" ").collect();
                let r: i64;
                let l: i64;
                let mut new_val: i64 = 0;
                if operation[0] == "old" {
                    l = monkeys[monkey].items[i];
                } else {
                    l = operation[0].parse::<i64>().unwrap();
                }
                if operation[2] == "old" {
                    r = monkeys[monkey].items[i];
                } else {
                    r = operation[2].parse::<i64>().unwrap();
                }
                if operation[1] == "+" {
                    new_val = r + l;
                } else if operation[1] == "*" {
                    new_val = r * l;
                }
                new_val = new_val % divisor;
                if new_val % monkeys[monkey].condition as i64 != 0 {
                    dest = monkeys[monkey].throw_to_if_false;
                }
                monkeys[dest].items.push(new_val);
                // println!("{}", new_val);
            }
            monkeys[monkey].items = Vec::new()
        }
        // println!("{:?}", monkeys);
    }
    inspections.sort();
    let mut tmp = inspections.iter().rev();
    println!("{:?}", tmp.nth(0).unwrap() * tmp.nth(0).unwrap());
}
