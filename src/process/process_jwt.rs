use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

const SECRET: &[u8] = b"zack is very handsome";
const AUDIENCE: &[&str] = &["hkjc", "ACME", "acme", "HKJC", "device1"];

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
    aud: String,
}

pub fn process_jwt_sign(sub: &str, exp: u64, aud: &str) -> anyhow::Result<String> {
    let claims = Claims {
        sub: sub.to_string(),
        exp: get_current_timestamp() + exp * 3600,
        aud: aud.to_string(),
    };
    println!("{:?}", claims);

    // default is sha256
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .unwrap();

    Ok(token)
}

pub fn process_jwt_verify(token: &str) -> anyhow::Result<String> {
    let mut valdation = Validation::new(Algorithm::default());
    valdation.set_audience(AUDIENCE);
    let token_message = decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &valdation)?;
    Ok(format!("{:?}", token_message.claims))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_sign() -> anyhow::Result<()> {
        let sub = "zack";
        let exp = 1;
        let aud = "device1";
        let token = process_jwt_sign(sub, exp, aud)?;
        println!("{}", token);

        let message = process_jwt_verify(&token)?;
        println!("message: {}", message);
        Ok(())
    }
}
