use std::io::prelude::*;
use std::ops::Range;
use std::{fs::File, io::BufReader, path::Path};

struct Policies<'a> {
    range: Range<u16>,
    key_policy: char,
    key: &'a str,
}

fn get_policies_fields(policies_raw: &str) -> Policies {
    let fields = policies_raw.split_whitespace().collect::<Vec<&str>>();

    let range_nums = fields
        .first()
        .unwrap()
        .split('-')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<u16>>();
    let range = *range_nums.first().unwrap()..(range_nums.get(1).unwrap() + 1u16);

    let key_policy = *fields
        .get(1)
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .first()
        .unwrap();

    let key = fields.get(2).unwrap();

    Policies {
        range,
        key_policy,
        key,
    }
}

fn is_the_key_valid(policies_raw: &str) -> bool {
    let Policies {
        range,
        key_policy,
        key,
    } = get_policies_fields(policies_raw);
    let key_policy_count = key.chars().fold(u16::default(), |count, char| {
        if char == key_policy {
            count + 1
        } else {
            count
        }
    });

    range.contains(&key_policy_count)
}

#[derive(Default)]
pub struct Keys {
    pub valid: Vec<String>,
    pub invalid: Vec<String>,
}

fn get_encryption_policies_reader(filepath: &str) -> BufReader<File> {
    let filepath = Path::new(filepath);
    let file = File::open(filepath).unwrap();
    BufReader::new(file)
}

const FILEPATH: &str = "data/encryption_policies.txt";

pub fn get_keys() -> Keys {
    get_encryption_policies_reader(FILEPATH)
        .lines()
        .map(|line| line.unwrap())
        .fold(Keys::default(), |mut key, policies_raw| {
            let fields = get_policies_fields(&policies_raw);
            if is_the_key_valid(&policies_raw) {
                key.valid.push(fields.key.to_string());
                key
            } else {
                key.invalid.push(fields.key.to_string());
                key
            }
        })
}

// TODO: create more unit testing
#[test]
fn valid_key_1() {
    const KEY: &str = "2-4 f: fgff";
    let is_valid = is_the_key_valid(KEY);
    assert!(is_valid)
}

#[test]
fn valid_key_2() {
    const KEY: &str = "1-6 h: hhhhhh";
    let is_valid = is_the_key_valid(KEY);
    assert!(is_valid)
}

#[test]
fn invalid_key() {
    const KEY: &str = "4-6 z: zzzsg";
    let is_not_valid = !is_the_key_valid(KEY);
    assert!(is_not_valid)
}
