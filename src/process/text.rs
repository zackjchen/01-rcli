// #![allow(unused)]
use crate::{cli::text::TextSignFormat, utils::get_reader};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};

use super::process_genpass;

pub trait TextSign {
    // &[u8]实现了Read
    // 这里采用了动态分派, 编译出的体积较小，但是效率比静态分派低
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}
pub trait Verify {
    // fn verify(&self, reader: &dyn Read, sign: &[u8]) -> Result<bool>;
    // 这两种是静态分派,和动态分派能实现同样效果, 编译出的代码体积更大，但效率更高
    fn verify(&self, reader: impl Read, sign: &[u8]) -> Result<bool>;
    // fn verify<R: Read>(&self, reader: R, sign: &[u8]) -> Result<bool>;
}
pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

struct Blake3 {
    key: [u8; 32],
}

impl TextSign for Blake3 {
    /// sign the data from the reader and return the signiture
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf[性能] by reading in chunks
        let mut buf = vec![];
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes().to_vec())
    }
}
impl Verify for Blake3 {
    /// verify the data with the signiture
    fn verify(&self, mut reader: impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = vec![];
        reader.read_to_end(&mut buf)?;

        // blake3::hash()返回的是一个owner的Hash类型, as_bytes()返回的是一个引用
        // 这里不用指针指向owner的话，他就会free掉,导致as_bytes()返回的引用失效
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        println!("hash: {:?}", URL_SAFE_NO_PAD.decode(hash));
        Ok(hash == sign)
    }
}
impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Blake3::new(key))
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}
struct Ed25519Signer {
    key: SigningKey,
}
impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = vec![];
        reader.read_to_end(&mut buf)?;
        let sign = self.key.sign(&buf);
        Ok(sign.to_bytes().to_vec())
    }
}

struct Ed25519Verifier {
    key: VerifyingKey,
}
impl Verify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = vec![];
        reader.read_to_end(&mut buf)?;
        // from_bytes()接收[u8;64]的数组
        let sig = Signature::from_bytes(sign.try_into()?);
        // 这个lib同时支持用公钥和签名去验证
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}
impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }
}

impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }
    fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key: &[u8; 32] = key.try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self::new(key))
    }
}
impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let reader = fs::read(path)?;
        Self::try_new(&reader)
    }
}
impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let reader = fs::read(path)?;
        Self::try_new(&reader)
    }
}
impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        // 公钥
        let public_key = sk.verifying_key().to_bytes().to_vec();
        // 私钥
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, public_key])
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let mut input = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            // // 这里的key是文件路径，需要读取文件内容
            // let key = fs::read(key)?;
            // // 文件多了换行符，需要去掉最后一位
            // let key = &key[..32];
            // let key = key.try_into().unwrap();
            // let signer = Blake3{key};
            let signer: Blake3 = Blake3::load(key)?;
            signer.sign(&mut input)?
        }
        TextSignFormat::Ed25519 => {
            let sign = Ed25519Signer::load(key)?;
            sign.sign(&mut input)?
        }
    };

    // TODO: sign need encode to Base64
    let signed = URL_SAFE_NO_PAD.encode(signed);

    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    sign: &str,
    format: TextSignFormat,
) -> anyhow::Result<bool> {
    let mut input = get_reader(input)?;
    let sign = URL_SAFE_NO_PAD.decode(sign)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut input, &sign)?
        }
        TextSignFormat::Ed25519 => {
            let verifier: Ed25519Verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut input, &sign)?
        }
    };

    Ok(verified)
}
pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

#[cfg(test)]
mod test {

    use super::{Blake3, KeyLoader, TextSign};
    use crate::process::{
        text::{Ed25519Signer, Ed25519Verifier},
        Verify,
    };
    use anyhow::Result;
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let signer = Blake3::load("./fixtures/blake3.txt")?;
        let verifier = Blake3::load("./fixtures/blake3.txt")?;
        let data = b"hello world";
        let sign = signer.sign(&mut &data[..])?;
        let signed = URL_SAFE_NO_PAD.encode(sign);
        let sign = URL_SAFE_NO_PAD.decode(signed)?;
        let verify_res = verifier.verify(&data[..], &sign)?;
        assert!(verify_res);
        Ok(())
    }
    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let signer = Ed25519Signer::load("./fixtures/ed25519.sk")?;
        let data = b"hello world";
        let sign = signer.sign(&mut &data[..])?;
        let verifier: Ed25519Verifier = Ed25519Verifier::load("./fixtures/ed25519.pk")?;
        let verify_res = verifier.verify(&mut &data[..], &sign)?;
        assert!(verify_res);
        Ok(())
    }
}
