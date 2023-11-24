use std::fs;

fn operate_with_symbols(symbols: &str) -> String {
    let (msg, _) = symbols
        .chars()
        .fold((String::new(), 0), |(msg, count), char| {
            let operated = match char {
                '#' => Some(count + 1),
                '@' => Some(count - 1),
                '*' => Some(count * count),
                '&' => None,
                _ => panic!("symbol not supported: {}", char),
            };
            let msg = match operated {
                Some(_) => msg,
                None => format!("{}{}", msg, count),
            };
            (msg, operated.unwrap_or(count))
        });
    msg
}

pub fn print_operated_challenge_message() {
    let challenge_message = fs::read_to_string("data/message_02.txt").unwrap();
    let operated_symbols = operate_with_symbols(challenge_message.trim());

    println!("challenge_02: {}", operated_symbols);
}

#[test]
#[ignore]
fn operation_1() {
    let operation = operate_with_symbols("##*&");
    assert_eq!(operation, "4");
}

#[test]
#[ignore]
fn operation_2() {
    let operation = operate_with_symbols("&##&*&@&");
    assert_eq!(operation, "0243");
}
