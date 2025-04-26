use crate::opts::GenPassOpts;
use rand::prelude::SliceRandom;
use rand::seq::IndexedRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"23456789";
const SYMBOL: &[u8] = b"!@#$%^&*()-_?";

pub fn process_gen_pass(opts: &GenPassOpts) -> anyhow::Result<()> {
    println!("Generating passwords...{:?}", opts);
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars: Vec<u8> = Vec::new(); // 明确指定类型为 Vec<char>
    if opts.uppercase {
        chars.extend_from_slice(UPPER); // 将字符串转换为 char 切片
        password.push(
            *UPPER
                .choose(&mut rng)
                .expect("Failed to choose a character from the list"),
        );
    }
    if opts.lowercase {
        chars.extend_from_slice(LOWER);
        password.push(
            *LOWER
                .choose(&mut rng)
                .expect("Failed to choose a character from the list"),
        );
    }
    if opts.number {
        chars.extend_from_slice(NUMBER);
        password.push(
            *NUMBER
                .choose(&mut rng)
                .expect("Failed to choose a character from the list"),
        );
    }
    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("Failed to choose a character from the list"),
        );
    }
    for _ in 0..(opts.length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("Failed to choose a character from the list");
        password.push(*c);
    }
    //TODO make sure the password has each type
    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;
    println!("{}", password);

    let result = zxcvbn(&password, &[]);
    println!("{}", result.score());

    Ok(())
}
