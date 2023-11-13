use crate::messages::CHALLENGE_02_MESSAGE;

fn operate_with_symbols(symbols: &str) -> String {
    symbols
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
        })
        .0
}

pub fn operate_challenge_message() -> String {
    operate_with_symbols(CHALLENGE_02_MESSAGE)
}

#[test]
fn operation_1() {
    let operation = operate_with_symbols("##*&");
    assert_eq!(operation, "4");
}

#[test]
fn operation_2() {
    let operation = operate_with_symbols("&##&*&@&");
    assert_eq!(operation, "0243");
}
