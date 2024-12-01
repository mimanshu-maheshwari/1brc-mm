use std::{
    collections::BTreeMap,
    fmt::Display,
    fs::File,
    io,
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
};

use memmap2::{Mmap, MmapOptions};

const MIN_TEMP: i16 = -999;
const MAX_TEMP: i16 = 999;
const FILE_PATH: &str = "./res/measurements.txt";
// const SEGMENT_SIZE: usize = 1 << 21;
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

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.1}/{:.1}/{:.1}",
            (self.min as f32) / 10.0_f32,
            self.average(),
            (self.max as f32) / 10.0f32
        )
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
fn next_new_line(mmap: &Mmap, pos: usize) -> usize {
    let mut pos = pos;
    while pos < mmap.len() && mmap[pos] != b'\n' {
        pos += 1;
    }
    pos + 1 // Move to the start of the next line
}

#[inline]
fn parse_segment(
    mmap: &Mmap,
    start_pos: usize,
    end_pos: usize,
    results: &mut BTreeMap<String, Stats>,
) {
    let mut pos = start_pos;
    while pos < end_pos {
        let end_of_line = mmap[pos..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|p| pos + p + 1)
            .unwrap_or(end_pos);

        let line = &mmap[pos..end_of_line];
        if let Ok(buffer) = std::str::from_utf8(line) {
            if let Some((city_name, temp)) = buffer.trim().split_once(";") {
                let city_name = city_name.to_string();
                if let Ok(temp) = temp.parse::<f32>() {
                    let entry = results.entry(city_name).or_default();
                    entry.update((temp * 10.0) as i16);
                }
            }
        }
        pos = end_of_line;
    }
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
    let file = File::options()
        .read(true)
        .create(false)
        .write(false)
        .open(file_path)?;
    let file_size = file.metadata()?.len() as usize;
    // println!("file size: {file_size}");

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    // println!("num of threads: {num_threads}");
    let mmap = Arc::new(unsafe { MmapOptions::new().len(file_size).map(&file)? });
    let segment_size = file_size / num_threads;
    // let segment_size = SEGMENT_SIZE.min(file_size / num_threads);
    let cursor = Arc::new(AtomicUsize::new(0));
    let result_sets = Arc::new(Mutex::new(Vec::with_capacity(num_threads)));
    let mut handles = vec![];

    for _ in 0..num_threads {
        let cursor = Arc::clone(&cursor);
        let result_sets = Arc::clone(&result_sets);
        let mmap_ = Arc::clone(&mmap);

        let handle = thread::spawn(move || {
            let mut local_results = BTreeMap::new();
            let start_pos = cursor.fetch_add(segment_size, Ordering::SeqCst);
            if start_pos >= mmap_.len() {
                return;
            }

            let end_pos = (start_pos + segment_size).min(mmap_.len());
            let start_line = if start_pos == 0 {
                0 // Start directly from beginning for the first chunk
            } else {
                next_new_line(&mmap_, start_pos)
            };
            // let start_line = next_new_line(&mmap_, start_pos);
            let end_line = next_new_line(&mmap_, end_pos);

            parse_segment(&mmap_, start_line, end_line, &mut local_results);

            result_sets.lock().unwrap().push(local_results);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let all_results = Arc::try_unwrap(result_sets).unwrap().into_inner().unwrap();
    let final_results = accumulate_results(all_results);

    for (city_name, stats) in final_results {
        println!("{};{}", city_name, stats);
    }

    Ok(())
}
