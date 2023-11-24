mod challenge_01;
mod challenge_02;
mod challenge_03;

fn main() {
    let _challenge_01 = challenge_01::decode_challenge_message();
    let _challenge_02 = challenge_02::operate_challenge_message();
    let challenge_03 = challenge_03::get_encryption_policies_keys();

    println!("challenge_03:");
    println!(
        "13th invalid key: {}",
        challenge_03.invalid.get(12).expect("challenge 03 failed")
    );
    println!(
        "42nd invalid key: {}",
        challenge_03.invalid.get(41).expect("challenge 03 failed")
    );
}
