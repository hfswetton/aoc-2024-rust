use std::cmp::max;
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

fn get_file_start_end(disk: &Disk, file_id: u64) -> Result<(usize, usize), &str> {
    let start_opt = disk.iter().position(|id| {
        if let Some(id) = id { if *id == file_id { return true; } };
        false
    });
    let end_opt = disk.iter().rposition(|id| {
        if let Some(id) = id { if *id == file_id { return true; } };
        false
    });
    let start = match start_opt {
        Some(v) => v,
        None => { return Err("file not found on disk"); },
    };
    let end = match end_opt {
        Some(v) => v,
        None => { return Err("file not found on disk"); },
    };
    if disk[start..(end + 1)].iter().any(|id| (*id == None) || (id.unwrap() != file_id)) {
        return Err("file fragmented");
    }
    Ok((start, end))
}

fn swap_file(disk: &mut Disk, old_start: usize, old_end: usize, new_start: usize) -> Result<(), &str> {
    let file_len = old_end - old_start + 1;
    if disk[new_start..(new_start + file_len)].iter().any(|id| matches!(*id, Some(_))) {
        return Err("attempting to insert into existing file");
    }

    let split_point = max(old_start, new_start);
    let (left, right) = disk.split_at_mut(split_point);
    let left_slice_start = if split_point == old_start { new_start } else { old_start };
    left[left_slice_start..(left_slice_start + file_len)].swap_with_slice(&mut right[..file_len]);

    Ok(())
}

fn find_free_space(disk: &Disk, size: usize) -> Result<usize, ()> {
    let mut free_space_counter = 0;
    let mut start = 0;
    for (i, id) in disk.iter().enumerate() {
        match id {
            Some(_) => { free_space_counter = 0; },
            None => {
                if free_space_counter == 0 { start = i; }
                free_space_counter += 1;
            }
        }
        if free_space_counter == size {
            return Ok(start)
        }
    }
    Err(())
}

fn compress_disk(disk: &Disk) -> Disk {
    let max_file_id = disk.iter().filter_map(|e| *e).max().unwrap();
    let mut disk = disk.clone();
    for _id in 0..=max_file_id {
        let file_id = max_file_id - _id;
        let (file_start, file_end) = get_file_start_end(&disk, file_id).expect("unable to calculate file size");
        let file_size = file_end - file_start + 1;
        let free_space_start = find_free_space(&disk, file_size).expect("no free space found");
        if free_space_start < file_start {
            swap_file(&mut disk, file_start, file_end, free_space_start).expect("unable to move file");
        }
    }
    disk
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
