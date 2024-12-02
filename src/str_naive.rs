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
    let mut reader = BufReader::new(file);
    let mut buf = Vec::with_capacity(200);
    let mut records = HashMap::<Measurement>::with_capacity(1_000_000);
    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        if n == 0 {
            break;
        }
        let line = &buf[..n - 1];
        if let Some((city, value)) = split_at(line) {
            let value = parse_temp(value);
            if let Some(val) = records.get_mut_(city) {
                val.update(value);
            } else {
                records.insert(
                    String::from_utf8(city.to_owned()).unwrap(),
                    Measurement::new(value),
                );
            }
        }
        buf.clear();
    }
    display(records);
}

#[inline(always)]
fn split_at(line: &[u8]) -> Option<(&[u8], &[u8])> {
    for (i, b) in line.iter().enumerate() {
        if b == &b';' {
            return Some((&line[0..i], &line[i + 1..]));
        }
    }
    None
}

#[inline(always)]
fn parse_temp(bytes: &[u8]) -> f32 {
    let mut neg = false;
    let mut value = 0.0;
    let mut multiplier = 1.0;
    for &byte in bytes {
        match byte {
            b'-' => neg = true,
            b'.' => multiplier = 0.1,
            s if (b'0'..=b'9').contains(&s) => {
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
fn display(result: HashMap<Measurement>) {
    let mut result: Vec<_> = result.iter().collect();
    result.sort_by_key(|k| k.0);
    for (city, measurement) in result {
        println!("{city};{measurement}");
    }
}
