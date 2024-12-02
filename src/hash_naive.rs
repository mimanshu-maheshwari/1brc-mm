use super::hashmap::HashMap;
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub struct Measurement {
    pub min: f32,
    pub max: f32,
    pub sum: f32,
    pub count: usize,
}
impl Measurement {
    #[inline(always)]
    fn new(temp: f32) -> Self {
        Self {
            min: temp,
            max: temp,
            sum: temp,
            count: 1,
        }
    }
    #[inline(always)]
    fn update(&mut self, temp: f32) {
        self.min = temp.min(self.min);
        self.max = temp.max(self.max);
        self.sum += temp;
        self.count += 1;
    }
}

impl Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.1}/{:.1}/{:.1}",
            self.min,
            (self.sum / self.count as f32),
            self.max
        )
    }
}

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
