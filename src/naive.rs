use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
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
    fn add(&mut self, temp: f32) {
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
            "{}/{}/{}",
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
        .fold(HashMap::<String, Measurement>::new(), |mut acc, e| {
            acc.entry(e.0.to_string())
                .and_modify(|v| v.add(e.1))
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
