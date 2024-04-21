use std::{fs::File, io::Read};

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        eprintln!("path: {}", input);
        let file = File::open(input)?;
        Box::new(file)
    };
    Ok(reader)
}
