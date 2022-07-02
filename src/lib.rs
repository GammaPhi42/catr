// Attribution:
// Command-Line Rust by
// Ken Youens-Clark (O’Reilly). Copyright 2022 Charles Kenneth Youens-Clark,
// 978-1-098-10943-1

use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// fn read_lines(file_handle: Box<dyn BufRead>, number_lines: bool, number_nonblank_lines: bool) -> MyResult<()> {
//     let mut line_number = 0;
//     for line in file_handle.lines() {
//         let line = line?;

//         if (number_nonblank_lines && !line.is_empty()) || number_lines {
//             line_number += 1;
//             println!("{:>6}\t{}", line_number, line);
//         } else {
//             println!("{}", line);
//         }

        
//     }
//     Ok(())
// }


pub fn run(config: Config) -> MyResult<()> {
    let mut line_number = 0;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),

            Ok (file) => {
                for line in file.lines() {
                    let line = line?;
                    if (config.number_nonblank_lines && !line.is_empty()) || config.number_lines {
                        line_number += 1;
                        println!("{:>6}\t{}", line_number, line);
                    }
                    else {
                        println!("{}", line);
                    }
                }
            
            
            },
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Michael Miller <GammaPhi42@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .multiple(true)
                .min_values(1) // Is this needed with .default_value?
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Print line numbers")
                .conflicts_with("number-nonblank"),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Print non-blank line numbers")
                .conflicts_with("number"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}
