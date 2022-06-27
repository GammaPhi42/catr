use std::error::Error;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
    .version("0.1.0")
    .author("Ken Youens-Clark <kyclark@gmail.com")
    .about("Rust cat")
    .arg(
        Arg::with_name("files")
        .value_name("FILES")
        .help("Input files")
        .required(true)
        .min_values(1),
    )
    .arg(
        Arg::with_name("number_lines")
        .value_name("NUMBER_LINES")
        .help("Print line numbers")
    )
    .arg()
    .get_matches();

    Ok(Config {
        files: ...,
        number_lines: ...,
        number_nonblank_lines: ...,
    })
}