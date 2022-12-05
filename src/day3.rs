use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

const PRIORITIES: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn find_priority(group: &[&str]) -> Option<usize> {
    if let Some(first_line) = group.first() {
        for n in 0..first_line.len() {
            if let Some(c) = first_line.get(n..n + 1) {
                if group.iter().all(|g| g.contains(c)) {
                    return PRIORITIES.find(c);
                }
            }
        }
    }
    None
}

pub fn main() {
    if let Ok(contents) = read_file("./input.txt") {
        let sum: usize = contents
            .split("\r\n")
            .collect::<Vec<&str>>()
            .chunks(3)
            .filter_map(find_priority)
            .sum();
        println!("{:?}", sum);
    }
}

fn read_file<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut reader = io::BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}
