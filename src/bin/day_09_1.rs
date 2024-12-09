use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Disk checksum";
const INPUT_FILE: &str = "input/day_09.txt";

type Disk = Vec<Option<u64>>;

fn parse_disk_map(disk_map: Vec<u32>) -> Disk {
    let mut disk: Disk = Vec::new();
    let mut file = true;
    let mut file_nr = 0;
    for n in disk_map {
        if file {
            for _ in 0..n { disk.push(Some(file_nr)); }
            file_nr += 1;
        } else {
            for _ in 0..n { disk.push(None); }
        }
        file = !file;
    }
    disk
}

fn compress_disk(disk: &Disk) -> Disk {
    let mut new_disk = disk.clone();
    let mut i = 0;
    let mut j = new_disk.len() - 1;
    while let Some(_) = new_disk[i] { i += 1; }
    while let None = new_disk[j] { j -= 1; }
    while i <= j {
        new_disk.swap(i, j);
        while let Some(_) = new_disk[i] { i += 1; }
        while let None = new_disk[j] { j -= 1; }
    }
    new_disk
}

fn calculate_checksum(disk: &Disk) -> u64 {
    disk
        .iter()
        .enumerate()
        .map(|(i, &id)| {
            if let Some(id_val) = id { (i as u64) * id_val } else { 0 }
        })
        .sum()
}

fn calculate_result(mut lines: Lines<BufReader<File>>) -> Result<u64, ()> {
    let starting_disk_map: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| char::to_digit(c, 10).expect("invalid digit"))
        .collect();
    // Example from AoC: let starting_disk_map = vec!(2,3,3,3,1,3,3,1,2,1,4,1,4,1,3,1,4,0,2);
    let starting_disk: Disk = parse_disk_map(starting_disk_map);
    let compressed_disk: Disk = compress_disk(&starting_disk);
    let checksum = calculate_checksum(&compressed_disk);
    Ok(checksum)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
