use std::collections::VecDeque;
use std::fs;

pub struct ContainerCollection {
    stacks: Vec<VecDeque<char>>,
}

impl ContainerCollection {
    fn new() -> ContainerCollection {
        let stacks = Vec::new();
        return ContainerCollection {
            stacks,
        };
    }
}

trait ParseStacks {
    fn parse(&mut self, inp: &str);
}

trait MoveStacks {
    fn moves(&mut self, inp: &str);
}

impl ParseStacks for ContainerCollection {
    fn parse(&mut self, inp: &str) {
        let mut rows: Vec<&str> = inp.split("\n").collect();
        let row_nrs: &str = rows.pop().unwrap();

        for i in 0..(row_nrs.len() as usize) {
            if row_nrs.chars().nth(i).unwrap() != ' ' {
                let mut current_stack: VecDeque<char> = VecDeque::new();
                for row in &rows {
                    if row.len() >= i {
                        let c = row.chars().nth(i).unwrap();
                        if c != ' ' {
                            current_stack.push_back(c);
                        }
                    }
                }
                self.stacks.push(current_stack);
            }
        }
        // println!("{:?}", self.stacks);
    }
}

impl MoveStacks for ContainerCollection {
    fn moves(&mut self, inp: &str) {
        let mut rows = inp.split("\n");

        for row in rows {
            let parts: Vec<&str> = row.split(' ').collect();
            let nr = parts[1].parse::<i32>().unwrap();
            let from = parts[3].parse::<usize>().unwrap() - 1;
            let to = parts[5].parse::<usize>().unwrap() - 1;
            let mut tmp = VecDeque::new();
            for _ in 0..nr {
                let val = self.stacks[from].pop_front().unwrap();
                tmp.push_front(val);
            }
            for _ in 0..nr {
                let val = tmp.pop_front().unwrap();
                self.stacks[to].push_front(val);
            }
        }
        // println!("{:?}", self.stacks);
    }
}

pub fn main() {
    let file_path = "inputs/05.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let section_pairs: Vec<&str> = contents.split("\n\n").collect();
    let mut cc = ContainerCollection::new();
    cc.parse(section_pairs[0]);
    cc.moves(section_pairs[1]);
    for s in cc.stacks {
        print!("{}", s[0])
    }
    print!("\n")
}
