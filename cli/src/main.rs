use std::fs;
use data_transcriber::{convert, to_javascript};

fn main() -> std::io::Result<()> {
    let catalogus = convert("../api-bohemond/resources/catalogus.csv".to_string());
    fs::write("data.js", to_javascript(catalogus.unwrap()).unwrap());
    Ok(())
}
