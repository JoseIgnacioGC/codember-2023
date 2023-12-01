use std::fs;

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

pub fn print_decoded_challenge_message() {
    let challenge_message = fs::read_to_string("data/message_01.txt").unwrap();
    let decoded_message = decode_message(challenge_message.trim());

    assert_eq!("murcielago15leon15jirafa15cebra6elefante15rinoceronte15hipopotamo15ardilla15mapache15zorro15lobo15oso15puma2jaguar14tigre10leopardo10gato12perro12caballo14vaca14toro14cerdo14oveja14cabra14gallina10pato10ganso10pavo10paloma10halcon11aguila11buho11colibri9canario8loro8tucan8pinguino7flamenco7", decoded_message);
    println!("challenge_01: {}", decoded_message);
}

#[test]
fn decode_dog_cat() {
    let decoded = decode_message("cat dog dog car Cat doG sun");
    assert_eq!(decoded, "cat2dog3car1sun1");
}

#[test]
fn decode_key_house() {
    let decoded = decode_message("keys house HOUSE house keys");
    assert_eq!(decoded, "keys2house3");
}

#[test]
fn decode_cup_te() {
    let decoded = decode_message("cup te a cup");
    assert_eq!(decoded, "cup2te1a1");
}

#[test]
fn decode_house() {
    let decoded = decode_message("houses house housess");
    assert_eq!(decoded, "houses1house1housess1");
}
