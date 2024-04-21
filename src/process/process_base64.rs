use crate::cli::base64_opts::Base64Format;
use crate::utils::get_reader;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut input = get_reader(input)?;
    let mut buf = vec![];
    input.read_to_end(buf.as_mut())?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encode)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // 读取完了会多出来一个换行
    let buf = buf.trim();
    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    Ok(String::from_utf8(decode)?)
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_process_encode() {
        let input = "fixtures/test.csv";
        let format = Base64Format::Standard;
        process_encode(input, format).unwrap();
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/test.txt";
        let format = Base64Format::Standard;
        process_decode(input, format).unwrap();
    }
}
