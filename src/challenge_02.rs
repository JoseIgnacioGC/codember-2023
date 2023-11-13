use crate::messages::CHALLENGE_02_MESSAGE;

fn operate_with_symbols(symbols: &str) -> String {
    let nums: String = symbols
        .chars()
        .fold((String::new(), 0), |(msg, num), char| {
            let current_num: i32 = match char {
                '#' => num + 1,
                '@' => num - 1,
                '*' => num * num,
                '&' => num,
                _ => panic!("symbol not supported"),
            };
            if current_num == num {
                let msg = format!("{}{}", msg, current_num);
                (msg, num)
            } else {
                (msg, current_num)
            }
        })
        .0;
    nums
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
