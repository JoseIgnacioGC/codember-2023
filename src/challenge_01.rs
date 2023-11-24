use crate::messages::CHALLENGE_01_MESSAGE;
use indexmap::map::IndexMap;

fn decode_message(msg: &str) -> String {
    msg.split_whitespace()
        .map(|word| word.to_lowercase())
        .fold(IndexMap::<String, u16>::new(), |mut words, word| {
            words
                .entry(word)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            words
        })
        .iter()
        .fold(String::new(), |msg, (key, value)| {
            format!("{}{}{}", msg, key, value)
        })
}

pub fn decode_challenge_message() -> String {
    decode_message(CHALLENGE_01_MESSAGE)
}

#[test]
#[ignore]
fn decode_dog_cat() {
    let decoded = decode_message("cat dog dog car Cat doG sun");
    assert_eq!(decoded, "cat2dog3car1sun1");
}

#[test]
#[ignore]
fn decode_key_house() {
    let decoded = decode_message("keys house HOUSE house keys");
    assert_eq!(decoded, "keys2house3");
}

#[test]
#[ignore]
fn decode_cup_te() {
    let decoded = decode_message("cup te a cup");
    assert_eq!(decoded, "cup2te1a1");
}

#[test]
#[ignore]
fn decode_house() {
    let decoded = decode_message("houses house housess");
    assert_eq!(decoded, "houses1house1housess1");
}
