use std::error::Error;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
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
        .min_values(1)
        .default_value("-")
    )
    .arg(
        Arg::with_name("number")
        .short("n")
        .help("Print line numbers")
        .conflicts_with("number-nonblank")
    )
    .arg(
        Arg::with_name("number-nonblank")
        .short("b")
        .help("Print non-blank line numbers")
        .conflicts_with("number")
    )
    .get_matches();


    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}