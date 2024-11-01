use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom},
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
};

const MIN_TEMP: i16 = -999;
const MAX_TEMP: i16 = 999;
const FILE_PATH: &str = "./res/measurements.txt";
const SEGMENT_SIZE: usize = 1 << 21;
// const HASH_TABLE_SIZE: usize = 1 << 17;
// const MAX_NAME_LENGTH: usize = 100;
// const MAX_CITIES: usize = 10000;

#[derive(Debug)]
struct Stats {
    min: i16,
    max: i16,
    sum: i64,
    count: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            min: MAX_TEMP,
            max: MIN_TEMP,
            sum: 0,
            count: 0,
        }
    }
}

impl Stats {

    #[inline(always)]
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn update(&mut self, temperature: i16) {
        self.min = self.min.min(temperature);
        self.max = self.max.max(temperature);
        self.sum += temperature as i64;
        self.count += 1;
    }

    #[inline]
    fn accumulate(&mut self, other: &Stats) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.count += other.count;
        self.sum += other.sum;
    }

    #[inline(always)]
    fn average(&self) -> f32 {
        (self.sum / self.count as i64) as f32 / 10.0_f32
    }
}

#[inline]
fn next_new_line(file: &mut BufReader<File>, pos: usize) -> io::Result<usize> {
    let mut buf = [0; 1];
    file.seek(SeekFrom::Start(pos as u64))?;
    let mut pos = pos;
    while file.read(&mut buf)? > 0 {
        if buf[0] == b'\n' {
            pos += 1;
        }
    }
    Ok(pos)
}

#[inline]
fn parse_segment(
    file: &mut BufReader<File>,
    start_pos: usize,
    end_pos: usize,
    results: &mut BTreeMap<String, Stats>,
) -> io::Result<()> {
    // let mut file = BufReader::new(File::open(file_path)?);
    file.seek(SeekFrom::Start(start_pos as u64))?;
    let mut buffer = String::new();
    while file.read_line(&mut buffer)? > 0 && (file.stream_position()? as usize) < end_pos {
        let parts = buffer.trim().split_once(";");
        if let Some((city_name, temp)) = parts {
            let city_name = city_name.to_string();
            if let Ok(temp) = temp.parse::<f32>() {
                let entry = results.entry(city_name).or_default(); // or_insert_with(Stats::new);
                entry.update((temp * 10.0) as i16);
            }
        }
        buffer.clear();
    }
    Ok(())
}

#[inline]
fn accumulate_results(all_results: Vec<BTreeMap<String, Stats>>) -> BTreeMap<String, Stats> {
    let mut final_results = BTreeMap::new();
    for result_map in all_results {
        for (city_name, result) in result_map {
            final_results
                .entry(city_name)
                .or_insert_with(Stats::new)
                .accumulate(&result);
        }
    }
    final_results
}
fn main() -> io::Result<()> {
    let file_path = Path::new(FILE_PATH);
    let display = file_path.display();
    let file_size = File::open(&file_path)
        .expect(format!("Unable to find path {}", display).as_str())
        .metadata()?
        .len() as usize;
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let segment_size = SEGMENT_SIZE.min(file_size / num_threads);
    let cursor = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    let result_sets = Arc::new(Mutex::new(Vec::with_capacity(num_threads)));
    for _ in 0..num_threads {
        let cursor = Arc::clone(&cursor);
        let result_sets = Arc::clone(&result_sets);
        let handle = thread::spawn(move || {
            let mut local_results = BTreeMap::new();
            let start_pos = cursor.fetch_add(segment_size, Ordering::SeqCst);
            if start_pos >= file_size {
                return;
            }
            let end_pos = (start_pos + segment_size).min(file_size);
            let mut file = BufReader::new(File::open(&file_path).unwrap());
            let start_line = next_new_line(&mut file, start_pos).unwrap();
            let end_line = next_new_line(&mut file, end_pos).unwrap();
            if parse_segment(&mut file, start_line, end_line, &mut local_results).is_ok() {
                result_sets.lock().unwrap().push(local_results);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let all_results = Arc::try_unwrap(result_sets).unwrap().into_inner().unwrap();
    let final_results = accumulate_results(all_results);

    for (city_name, result) in final_results {
        println!(
            "{}: min = {}, avg = {:.1}, max = {}",
            city_name,
            result.min,
            result.average(),
            result.max
        );
    }

    Ok(())
}
