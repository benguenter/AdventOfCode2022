use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

#[derive(Debug)]
enum Input {
    Rock(),
    Paper(),
    Scissors(),
}

#[derive(Debug)]
enum Strategy {
    Lose(),
    Draw(),
    Win(),
}

#[derive(Debug)]
enum Response {
    Rock(),
    Paper(),
    Scissors(),
}

fn parse_line(line: &str) -> Result<(Input, Strategy), String> {
    let input = match &line[0..1] {
        "A" => Ok(Input::Rock()),
        "B" => Ok(Input::Paper()),
        "C" => Ok(Input::Scissors()),
        _ => Err(format!("Unexpected input {line}")),
    }?;
    let strategy = match &line[2..3] {
        "X" => Ok(Strategy::Lose()),
        "Y" => Ok(Strategy::Draw()),
        "Z" => Ok(Strategy::Win()),
        _ => Err(format!("Unexpected response {line}")),
    }?;
    Ok((input, strategy))
}

fn formulate_response(t: (Input, Strategy)) -> (Input, Response) {
    match t {
        (Input::Rock(), Strategy::Lose()) => (t.0, Response::Scissors()),
        (Input::Rock(), Strategy::Draw()) => (t.0, Response::Rock()),
        (Input::Rock(), Strategy::Win()) => (t.0, Response::Paper()),
        (Input::Paper(), Strategy::Lose()) => (t.0, Response::Rock()),
        (Input::Paper(), Strategy::Draw()) => (t.0, Response::Paper()),
        (Input::Paper(), Strategy::Win()) => (t.0, Response::Scissors()),
        (Input::Scissors(), Strategy::Lose()) => (t.0, Response::Paper()),
        (Input::Scissors(), Strategy::Draw()) => (t.0, Response::Scissors()),
        (Input::Scissors(), Strategy::Win()) => (t.0, Response::Rock()),
    }
}

fn calculate_points(t: (Input, Response)) -> i32 {
    match t {
        (Input::Rock(), Response::Paper()) => 2 + 6,
        (Input::Rock(), Response::Rock()) => 1 + 3,
        (Input::Rock(), Response::Scissors()) => 3 + 0,
        (Input::Paper(), Response::Rock()) => 1 + 0,
        (Input::Paper(), Response::Paper()) => 2 + 3,
        (Input::Paper(), Response::Scissors()) => 3 + 6,
        (Input::Scissors(), Response::Scissors()) => 3 + 3,
        (Input::Scissors(), Response::Paper()) => 2 + 0,
        (Input::Scissors(), Response::Rock()) => 1 + 6,
    }
}

#[allow(dead_code)]
pub fn main() {
    if let Ok(contents) = read_file("./input.txt") {
        let sum = contents
            .split("\r\n")
            .filter_map(|line| parse_line(line).ok())
            .map(formulate_response)
            .map(calculate_points)
            .sum::<i32>();
        println!("{sum}");
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
