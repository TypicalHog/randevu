use blake3::hash;
use chrono::{Duration, Utc};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

struct Item {
    name: String,
    value: i32,
}

fn load_items() -> Vec<Item> {
    let file_path = Path::new("RANDEVU.randevu");
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut items = Vec::new();

    for line in reader.lines() {
        if let Ok(item_data) = line {
            let parts: Vec<&str> = item_data.split(' ').collect();
            if parts.len() == 2 {
                if let Ok(value) = parts[1].trim().parse::<i32>() {
                    let sanitized_name = sanitize_string(parts[0].trim());
                    let item = Item {
                        name: sanitized_name,
                        value,
                    };
                    items.push(item);
                }
            }
        }
    }

    items
}

fn sanitize_string(str: &str) -> String {
    let filtered: String = str
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() && !c.is_whitespace())
        .collect();

    filtered.to_uppercase()
}

fn utc_date_string_with_offset(offset: i64) -> String {
    let today = Utc::now();
    let target_date = today + Duration::days(offset);
    target_date.format("%Y-%m-%d").to_string()
}

fn randevu(item: &str, date: &str) -> u32 {
    let item_hash = hash(item.as_bytes());
    let date_hash = hash(date.as_bytes());

    let mut concatenated = [0u8; 64];
    concatenated[..32].copy_from_slice(item_hash.as_bytes());
    concatenated[32..].copy_from_slice(date_hash.as_bytes());
    let final_hash = hash(concatenated.as_slice());

    let mut randevu_level = 0;
    for byte in final_hash.as_bytes() {
        if *byte == 0 {
            randevu_level += 8;
        } else {
            randevu_level += byte.leading_zeros();
            break;
        }
    }

    randevu_level
}

fn main() {
    println!("RANDEVU v0.1.0\n");

    let filename = "RANDEVU.randevu";

    if !Path::new(filename).exists() {
        let mut file = File::create(filename).expect("Failed to create file");
        file.write_all(b"RANDEVU 5\n")
            .expect("Failed to write to file");
    }

    let items = load_items();

    let mut printed_dates: HashSet<String> = HashSet::new();

    for i in 0..3 {
        let date = utc_date_string_with_offset(i - 1);

        if printed_dates.insert(date.clone()) {
            println!("UTC {}", date);
        }

        for item in &items {
            let result = randevu(&item.name, &date);
            if result >= (item.value as i32).try_into().unwrap() {
                println!("    {} {}/{}", item.name, result, item.value);
            }
        }
    }
}
