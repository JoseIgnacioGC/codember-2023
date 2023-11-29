use std::io::prelude::*;
use std::ops::Range;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Clone)]
struct Policies {
    range: Range<u16>,
    key_policy: char,
    key: String,
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

    let key_policy = fields.get(1).unwrap().chars().next().unwrap();

    let key = fields.get(2).unwrap().to_string();

    Policies {
        range,
        key_policy,
        key,
    }
}

fn is_the_key_valid(policies: Policies) -> bool {
    let Policies {
        range,
        key_policy,
        key,
    } = policies;

    let key_policy_count = key
        .chars()
        .filter(|char| *char == key_policy)
        .collect::<Vec<char>>()
        .len()
        .try_into()
        .unwrap();

    range.contains(&key_policy_count)
}

#[derive(Default)]
pub struct Keys {
    pub valid: Vec<String>,
    pub invalid: Vec<String>,
}

fn get_encryption_policies_keys(policies: Vec<Policies>) -> Keys {
    policies
        .iter()
        .fold(Keys::default(), |mut key, policies_fields| {
            if is_the_key_valid(policies_fields.clone()) {
                key.valid.push(policies_fields.key.clone());
            } else {
                key.invalid.push(policies_fields.key.clone());
            };
            key
        })
}

pub fn print_challenge_invalid_keys() {
    let filepath = Path::new("data/encryption_policies.txt");
    let file = File::open(filepath).unwrap();
    let encryption_policies_raw = BufReader::new(file);

    let encryption_policies = encryption_policies_raw
        .lines()
        .map(|line| get_policies_fields(&line.unwrap()))
        .collect::<Vec<Policies>>();

    let keys = get_encryption_policies_keys(encryption_policies);

    println!("challenge_03:");
    println!(
        "  13th invalid key: {}",
        keys.invalid.get(12).expect("challenge 03 failed")
    );
    println!(
        "  42nd invalid key: {}",
        keys.invalid.get(41).expect("challenge 03 failed")
    );
}

#[test]
#[ignore]
fn valid_key_1() {
    const KEY: &str = "2-4 f: fgff";
    let policies_fields = get_policies_fields(KEY);
    let is_valid = is_the_key_valid(policies_fields);
    assert!(is_valid)
}

#[test]
#[ignore]
fn valid_key_2() {
    const KEY: &str = "1-6 h: hhhhhh";
    let policies_fields = get_policies_fields(KEY);
    let is_valid = is_the_key_valid(policies_fields);
    assert!(is_valid)
}

#[test]
#[ignore]
fn invalid_key() {
    const KEY: &str = "4-6 z: zzzsg";
    let policies_fields = get_policies_fields(KEY);
    let is_not_valid = !is_the_key_valid(policies_fields);
    assert!(is_not_valid)
}

#[test]
fn getting_encryption_policies_keys() {
    const VALID_KEY: &str = "1-6 h: hhhhhh";
    const INVALID_KEY: &str = "4-6 z: zzzsg";
    let encryption_policies = vec![
        get_policies_fields(VALID_KEY),
        get_policies_fields(INVALID_KEY),
    ];
    let keys = get_encryption_policies_keys(encryption_policies);
    let valid_key = keys.valid.first().unwrap();
    let invalid_key = keys.invalid.first().unwrap();
    assert_eq!("hhhhhh", valid_key);
    assert_eq!("zzzsg", invalid_key);
}
