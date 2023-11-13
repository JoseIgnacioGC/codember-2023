mod messages;
mod challenge_01;
mod challenge_02;

fn main() {
    let _challenge_01 = challenge_01::decode_challenge_message();
    let challenge_02 = challenge_02::operate_challenge_message();
    println!("challenge_02: {}", challenge_02);
}
