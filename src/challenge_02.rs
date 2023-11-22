use crate::messages::CHALLENGE_02_MESSAGE;

// TODO: should refactor `operated` and return
fn operate_with_symbols(symbols: &str) -> String {
    let (msg, _) = symbols
        .chars()
        .fold((String::new(), 0), |(msg, count), char| {
            let operated = match char {
                '#' => Some(count + 1),
                '@' => Some(count - 1),
                '*' => Some(count * count),
                '&' => None,
                _ => panic!("symbol not supported"),
            };
            let msg = match operated {
                Some(_) => msg,
                None => format!("{}{}", msg, count),
            };
            (msg, operated.unwrap_or(count))
        });
    return msg;
}

pub fn operate_challenge_message() -> String {
    operate_with_symbols(CHALLENGE_02_MESSAGE)
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
