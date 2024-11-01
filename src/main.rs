use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

struct Stats{
    min: f32, 
    max: f32, 
    sum: f32, 
    count: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            min: f32::MAX,
            max: f32::MIN, 
            sum: 0.0, 
            count: 0,
        }
    }
}

impl Stats {
    fn update(&mut self, temperature: f32) {
        self.min = self.min.min(temperature);
        self.max = self.max.max(temperature);
        self.sum += temperature;
        self.count += 1;
    }

    fn average(&self) -> f32 {
        self.sum / self.count as f32
    }
}
    
const DEFAULT_MEASUREMENTS_FILE: &str = "res/measurements.txt";
fn main() {
    // Determine file path from arguments or default
    let file_path = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_MEASUREMENTS_FILE.to_owned());

    // Open file and set up buffered reader
    let file = File::open(&file_path).expect("Could not open file");
    let buf_reader = BufReader::new(file);

    let mut stats_map: HashMap<String, Stats> = HashMap::new();

    // Process each line in the file
    for line in buf_reader.lines().filter_map(Result::ok) {
        if let Some((location, temp_str)) = line.split_once(";") {
            if let Ok(temperature) = temp_str.trim().parse::<f32>() {
                stats_map.entry(location.to_owned()).or_default().update(temperature);
            }
        }
    }

    // Output results
    for (location, stats) in &stats_map {
        println!("{};{}/{}/{}", location, stats.min, stats.average(), stats.max);
    }
}
