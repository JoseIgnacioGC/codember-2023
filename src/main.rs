mod challenge_01;
mod challenge_02;
mod challenge_03;

fn main() {
    let _challenge_01 = challenge_01::print_decoded_challenge_message;
    let _challenge_02 = challenge_02::print_operated_challenge_message;
    let challenge_03 = challenge_03::print_challenge_invalid_keys;

    challenge_03();
}
