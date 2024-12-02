use super::{hashmap::HashMap, measurement::Measurement};

use std::{
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
        .fold(
            HashMap::<Measurement>::with_capacity(1_000_000),
            |mut acc, e| {
                if acc.contains_key(&e.0) {
                    if let Some(val) = acc.get_mut(&e.0) {
                        val.update(e.1);
                    }
                } else {
                    acc.insert(e.0, Measurement::new(e.1));
                }
                acc
            },
        );
    display(val);
}

#[inline(always)]
fn display(result: HashMap<Measurement>) {
    for (city, measurement) in result.iter() {
        println!("{city};{measurement}");
    }
}
