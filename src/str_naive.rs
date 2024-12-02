use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

use super::measurement::Measurement;

type HashMap<K, V> = FxHashMap<K, V>;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// hashtable to lookup the u8
lazy_static! {
    static ref TEMP_VALUES: FxHashMap<Vec<u8>, f32> = {
        let mut map = FxHashMap::default();
        for int in -1000..=1000 {
            for dec in -9..=9 {
                if dec == 0 {
                    let key = format!("{}", int);
                    map.insert(key.as_bytes().to_vec(), int as f32);
                    let key = format!("{}.0", int);
                    map.insert(key.as_bytes().to_vec(), int as f32);
                } else {
                    let val = int as f32 + 0.1 * (dec as f32);
                    let key = format!("{}", val);
                    map.insert(key.as_bytes().to_vec(), val);
                }
            }
        }
        map.insert("-0".as_bytes().to_vec(), 0.0);
        map
    };
}

#[inline(always)]
pub fn solve(file_name: &str) {
    let file = File::options()
        .read(true)
        .write(false)
        .append(false)
        .open(file_name)
        .expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut buf = Vec::with_capacity(200);
    let mut records: HashMap<_, Measurement> = HashMap::default();
    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        if n == 0 {
            break;
        }
        let line = &buf[..n - 1];
        if let Some((city, value)) = split_at(line) {
            // println!("{city:?}|{value:?}");
            let value = *TEMP_VALUES
                .get(value)
                .ok_or_else(|| {
                    format!(
                        "can't find value  {}",
                        String::from_utf8(value.to_vec()).unwrap()
                    )
                })
                .unwrap();
            if let Some(val) = records.get_mut(city) {
                val.update(value);
            } else {
                records.insert(city.to_owned(), Measurement::new(value));
            }
        }
        buf.clear();
    }
    display(records);
}

#[inline(always)]
fn split_at(line: &[u8]) -> Option<(&[u8], &[u8])> {
    // println!("{line:?}");
    for (i, b) in line.iter().enumerate() {
        if b == &b';' {
            return Some((&line[0..i], &line[i + 1..]));
        }
    }
    None
}

#[inline(always)]
fn _parse_temp(bytes: &[u8]) -> f32 {
    let mut neg = false;
    let mut value = 0.0;
    let mut multiplier = 1.0;
    for &byte in bytes {
        match byte {
            b'-' => neg = true,
            b'.' => multiplier = 0.1,
            s if s.is_ascii_digit() => {
                value = (value * if multiplier == 0.1 { 1.0 } else { 10.0 })
                    + ((s - b'0') as f32 * multiplier)
            }
            s => panic!("Unexpected value ({s}) while parsing temp"),
        }
    }
    if neg {
        value *= -1.0;
    }
    value
}

#[inline(always)]
fn display(result: HashMap<Vec<u8>, Measurement>) {
    let mut result: Vec<_> = result
        .iter()
        .map(|(city, m)| (String::from_utf8(city.to_vec()).unwrap(), m))
        .collect();
    result.sort_unstable_by_key(|v| v.0.clone());
    for (city, measurement) in result {
        println!("{city};{measurement}");
    }
}
