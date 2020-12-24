use eyre::{Result, WrapErr};
use std::fs;

fn read_input(filenumber: &str) -> Result<String> {
    fs::read_to_string(format!("inputs/{}.txt", filenumber))
        .wrap_err(format!("could not find input file for {}", filenumber))
}

pub fn input(filenumber: &str) -> Result<String> {
    read_input(filenumber)
}

pub fn input_lines(filenumber: &str) -> Result<Vec<String>> {
    let contents = read_input(filenumber)?;
    Ok(contents.split("\n").map(ToString::to_string).collect())
}

pub fn input_paragraphs(filenumber: &str) -> Result<Vec<String>> {
    let contents = read_input(filenumber)?;
    Ok(contents.split("\n\n").map(ToString::to_string).collect())
}
