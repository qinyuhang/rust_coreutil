/// -c      The number of bytes in each input file is written to the standard output.  This will cancel out any prior usage of the -m option.

/// -l      The number of lines in each input file is written to the standard output.

/// -m      The number of characters in each input file is written to the standard output.  If the current locale does not support multibyte characters, this is equivalent
///         to the -c option.  This will cancel out any prior usage of the -c option.

/// -w      The number of words in each input file is written to the standard output.

/// When an option is specified, wc only reports the information requested by that option.  The order of output always takes the form of line, word, byte, and file name.
/// The default action is equivalent to specifying the -c, -l and -w options.
extern crate clap;
use clap::{Arg, Command};
use rust_coreutil::open;
use std::io::prelude::*;

#[allow(dead_code)]
#[derive(Debug)]
struct AppConfig {
    /// -c
    bytes: bool,
    /// -l
    lines: bool,
    /// -m
    chars: bool,
    /// -w
    words: bool,
    /// files
    files: Vec<String>,
}

fn make_app() -> AppConfig {
    let app = Command::new("wc")
        .arg(Arg::new("bytes").short('c').takes_value(false))
        .arg(Arg::new("lines").short('l').takes_value(false))
        .arg(Arg::new("chars").short('m').takes_value(false))
        .arg(Arg::new("words").short('w').takes_value(false))
        .arg(
            Arg::new("files")
                .takes_value(true)
                .min_values(0)
                .allow_invalid_utf8(true)
                .default_value("-"),
        );
    let m = app.get_matches();
    AppConfig {
        bytes: m.is_present("bytes"),
        lines: m.is_present("lines"),
        chars: m.is_present("chars"),
        words: m.is_present("words"),
        files: m.values_of_lossy("files").unwrap(),
    }
}

#[derive(Debug)]
struct FileInfo {
    num_lines: i32,
    num_chars: i32,
    num_bytes: i32,
    num_words: i32,
}
fn count(mut file: impl BufRead) -> std::io::Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_chars: i32 = 0;
    let mut num_bytes: i32 = 0;
    let mut num_words: i32 = 0;
    let mut line = String::new();
    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes as i32;
        num_lines += 1;
        num_words += line.split_whitespace().count() as i32;
        num_chars += line.chars().count() as i32;
        line.clear();
    }
    Ok(FileInfo {
        num_lines,
        num_chars,
        num_bytes,
        num_words,
    })
}

fn main() {
    let config = make_app();
    // println!("{:?}", config);

    // let f_len = config.files.len();

    // move all the count to a single fn
    config.files.iter().for_each(|f| {
        // if f_len > 1 {
        //     println!("==> {} <==", f);
        // }
        // TODO handle -c
        let mut fi = match open(f) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("head: {}: {}", f, e.to_string());
                std::process::exit(1);
            }
        };

        if let Ok(f_info) = count(fi) {
            if config.bytes {
                println!("{:>8} {}", f_info.num_bytes, f);
            }
            else if config.lines {
                println!("{:>8} {}", f_info.num_lines, f);
            }
            else if config.chars {
                println!("{:>8} {}", f_info.num_chars, f);
            }
            else if config.words {
                println!("{:>8} {}", f_info.num_words, f);
            }
            else {
                // has none options
                println!(
                    "{:>8}{:>8}{:>8} {}",
                    f_info.num_lines, f_info.num_words, f_info.num_bytes, f
                );
            }
            // todo handle other options
        }
        // println!("{:?}", f_info.unwrap());
        // if let Some(bytes) = config.bytes {
        //     let mut handle = fi.take(bytes as u64);
        //     let mut buf = vec![0; bytes];
        //     let n = handle.read(&mut buf).unwrap();
        //     print!("{}", String::from_utf8_lossy(&buf[..n]));
        // } else {
        //     println!(
        //         "{}",
        //         fi
        //             .lines()
        //             .take(config.count)
        //             .filter(|line| line.is_ok())
        //             .map(|line| line.unwrap())
        //             .collect::<Vec<String>>()
        //             .join("\n")
        //     );
        //     if f_len > 1 && idx != f_len - 1 {
        //         println!();
        //     }
        // }
    })
}
