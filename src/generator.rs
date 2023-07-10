use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

pub fn create_random(length: usize, path: &str, max: u32) -> Result<(), Error> {
    let mut buffer = File::create(path)?;
    let mut random = Vec::new();

    for _ in 0..length {
        let random_number = rand::thread_rng().gen_range(1..max);
        random.push(random_number);
    }

    let random_string = random
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>();

    write!(buffer, "{}", random_string.join(" "))
}

pub fn read_random(path: &str) -> Result<Vec<u32>, Error> {
    let mut buffer = File::open(path)?;
    let mut contents = String::new();
    buffer.read_to_string(&mut contents)?;

    let mut random: Vec<u32> = Vec::new();
    for c in contents.split(' ') {
        match c.parse::<u32>() {
            Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidData, e)),
            Ok(n) => random.push(n),
        };
    }

    Ok(random)
}
