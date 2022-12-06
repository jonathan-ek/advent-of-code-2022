use std::collections::{HashSet, VecDeque};
use std::fs;
use std::hash::Hash;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn main() {
    let file_path = "inputs/06.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut rolling_chars = VecDeque::new();
    let mut i = 0;
    for c in contents.chars() {
        rolling_chars.push_front(c);
        if rolling_chars.len() > 14 {
            rolling_chars.pop_back();
        }
        i += 1;
        if rolling_chars.len() == 14 && has_unique_elements(rolling_chars.iter()) {
            print!("{}", i);
            break
        }
    }
    print!("\n")
}
