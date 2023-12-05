use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Clone)]
struct FileNameItems {
    alphanumeric: String,
    checksum: String,
}

fn get_file_name_items(file_name: &str) -> FileNameItems {
    let items = file_name.split('-').collect::<Vec<&str>>();

    FileNameItems {
        alphanumeric: items.first().unwrap().to_string(),
        checksum: items.get(1).unwrap().to_string(),
    }
}

fn is_the_file_real(items: FileNameItems) -> bool {
    let FileNameItems {
        alphanumeric,
        checksum,
    } = items;
    alphanumeric.starts_with(&checksum)
}

#[derive(Debug, Default)]
pub struct Checksums {
    pub real: HashMap<usize, String>,
    pub fake: HashMap<usize, String>,
}

fn get_files_quarantine_checksums(items: Vec<FileNameItems>) -> Checksums {
    items
        .iter()
        .enumerate()
        .fold(Checksums::default(), |mut checksums, (index, items)| {
            if is_the_file_real(items.clone()) {
                checksums.real.insert(index + 1, items.checksum.clone());
            } else {
                checksums.fake.insert(index + 1, items.checksum.clone());
            };
            checksums
        })
}

//  NOTE: Disclaimer, I did not fully understand the problem.
pub fn print_challenge_real_file_checksum() {
    let filepath = Path::new("data/files_quarantine.txt");
    let file = File::open(filepath).unwrap();
    let files_quarantine_raw = BufReader::new(file);

    let items = files_quarantine_raw
        .lines()
        .map(|line| get_file_name_items(line.unwrap().trim()))
        .collect::<Vec<FileNameItems>>();

    let checksums = get_files_quarantine_checksums(items);
    let real_checksum = checksums.real.get(&33usize).expect("challenge 04 failed");

    assert_eq!("O2hrQ", real_checksum);
    println!("challenge_04:");
    println!("  33th real file: {}", real_checksum);
}

#[test]
fn real_file_1() {
    const FILE_NAME: &str = "xyzz33-xy";
    let file_name_items = get_file_name_items(FILE_NAME);
    let it_is_real = is_the_file_real(file_name_items);
    assert!(it_is_real)
}

#[test]
fn fake_file_1() {
    const FILE_NAME: &str = "abcca1-ab1";
    let file_name_items = get_file_name_items(FILE_NAME);
    let it_is_fake = !is_the_file_real(file_name_items);
    assert!(it_is_fake)
}

#[test]
fn fake_file_2() {
    const FILE_NAME: &str = "abbc11-ca";
    let file_name_items = get_file_name_items(FILE_NAME);
    let it_is_fake = !is_the_file_real(file_name_items);
    assert!(it_is_fake)
}

#[test]
fn getting_files_quarantine_checksums() {
    const REAL_FILE_NAME: &str = "xyzz33-xy";
    const FAKE_FILE_NAME: &str = "abcca1-ab1";
    let items = vec![
        get_file_name_items(REAL_FILE_NAME),
        get_file_name_items(FAKE_FILE_NAME),
    ];
    let checksums = get_files_quarantine_checksums(items);
    let valid_checksum = checksums.real.get(&1usize).unwrap();
    let invalid_checksum = checksums.fake.get(&2usize).unwrap();
    assert_eq!("xy", valid_checksum);
    assert_eq!("ab1", invalid_checksum);
}
