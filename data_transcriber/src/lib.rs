mod data_catalogus;
mod errors;

use std::{result};
pub use crate::data_catalogus::{Catalogus, CatalogusItem};
use std::env::current_dir;
use crate::errors::{io_err, TranscriberError};
use path_clean::PathClean;
use tracing::debug;

pub type Result<T> = result::Result<T, TranscriberError>;


pub fn to_javascript(catalogus: Catalogus) -> Result<String> {
    let result: Vec<String> = catalogus.iter().map(|i| i.to_js()).collect();
    let js = format!("var data = [\n\t{}\n]\n", result.join(",\n\t"));
    Ok(js)
}

pub fn convert(filename: String) -> Result<Catalogus> {
    let input_path = current_dir()
        .map_err(|e| io_err(&e, "Unable to determine current directory."))?
        .join(filename)
        .clean();
    debug!("Importing data file from {:#?}", input_path);
    let mut reader = csv::ReaderBuilder::new().from_path(input_path)?;
    let mut catalogus: Catalogus = vec!();
    for record in reader.deserialize() {
        let record: CatalogusItem = record?;
        catalogus.push(record);
    }
    Ok(catalogus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = convert("../api-bohemond/resources/catalogus.csv".to_string());

        // let result = to_javascript(result.unwrap());
        println!("Result is {:#?}", result.unwrap());
    }
}
