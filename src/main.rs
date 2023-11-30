mod challenge_01;
mod challenge_02;
mod challenge_03;
mod challenge_04;
mod challenge_05;

fn main() {
    let _challenge_01 = challenge_01::print_decoded_challenge_message;
    let _challenge_02 = challenge_02::print_operated_challenge_message;
    let _challenge_03 = challenge_03::print_challenge_invalid_keys;
    let _challenge_04 = challenge_04::print_challenge_real_file_checksum;
    let challenge_05 = challenge_05::print_challenge_hidden_message;

    challenge_05();
}
