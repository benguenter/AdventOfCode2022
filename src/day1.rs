use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

#[allow(dead_code)]
pub fn main() {
    if let Ok(contents) = read_file("./input.txt") {
        let mut calorie_groups: Vec<i32> = contents
            .split("\r\n\r\n")
            .map(|line| {
                line.split("\r\n")
                    .filter_map(|chunk| str::parse::<i32>(chunk).ok())
                    .sum()
            })
            .collect();
        calorie_groups.sort();
        let max_calorie_groups: i32 = calorie_groups[calorie_groups.len() - 3..].iter().sum();
        println!("{:?}", max_calorie_groups);
    } else {
        println!("Could not find file ./input.txt");
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
