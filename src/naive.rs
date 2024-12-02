use super::measurement::Measurement;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[inline(always)]
pub fn solve(file_name: &str) {
    let file = File::options()
        .read(true)
        .write(false)
        .append(false)
        .open(file_name)
        .expect("Unable to open file");
    let reader = BufReader::new(file);
    let val = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            if let Some((city, temp)) = line.split_once(";") {
                Some((city.to_string(), temp.parse::<f32>().unwrap()))
            } else {
                None
            }
        })
        .fold(HashMap::<String, Measurement>::new(), |mut acc, e| {
            acc.entry(e.0.to_string())
                .and_modify(|v| v.update(e.1))
                .or_insert(Measurement::new(e.1));
            acc
        });
    display(val);
}

#[inline(always)]
fn display(result: HashMap<String, Measurement>) {
    for (city, measurement) in result {
        println!("{city};{measurement}");
    }
}
