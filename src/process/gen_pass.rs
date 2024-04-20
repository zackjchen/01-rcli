use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghjklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_+-=";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();
    if upper {
        chars.extend_from_slice(UPPER);
        // make sure password has at least one of each type of character
        password.push(
            *UPPER
                .choose(&mut rng)
                .expect("UPPER won't be empty in this context"),
        );
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(
            *LOWER
                .choose(&mut rng)
                .expect("UPPER won't be empty in this context"),
        );
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(
            *NUMBER
                .choose(&mut rng)
                .expect("UPPER won't be empty in this context"),
        );
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("UPPER won't be empty in this context"),
        );
    }
    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    password.shuffle(&mut rng);
    let password = String::from_utf8(password).unwrap();

    // output the password strength
    let estimate = zxcvbn(&password, &[]).unwrap();
    println!("{}", password);
    // 这种print的方式是为了在 rcli genpass -> output.txt 时候，只保存密码, 不保存这个信息
    eprintln!("password strength: {}", estimate.score());

    Ok(())
}
