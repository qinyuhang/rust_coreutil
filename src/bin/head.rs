/// head [-n count | -c bytes] [file ...]
extern crate clap;
extern crate rust_coreutil;

use clap::{Arg, Command};
use rust_coreutil::open;

#[derive(Debug)]
struct AppConfig {
    files: Vec<String>,
    count: usize,
    bytes: Option<usize>,
}

fn get_config() -> AppConfig {
    let app = Command::new("head")
        .arg(
            Arg::new("files")
                .takes_value(true)
                .min_values(0)
                // .allow_invalid_utf8(true)
                .default_value("-"),
        )
        .arg(
            Arg::new("count")
                .short('n')
                .takes_value(true)
                .min_values(0)
                .max_values(1)
                .conflicts_with("bytes")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .takes_value(true)
                .min_values(1)
                .max_values(1)
                .conflicts_with("count"),
        );

    let m = app.get_matches();
    let mut c = AppConfig {
        files: m
            .values_of("files")
            .unwrap()
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        count: usize::from_str_radix(m.value_of("count").unwrap(), 10).unwrap(),
        bytes: None,
    };
    if m.is_present("bytes") {
        c.bytes = Some(usize::from_str_radix(m.value_of("bytes").unwrap(), 10).unwrap());
    }
    c
}

fn main() {
    let config = get_config();
    // println!("{:?}", config);
    let f_len = config.files.len();
    config.files.iter().enumerate().for_each(|(idx, f)| {
        if f_len > 1 {
            println!("==> {} <==", f);
        }
        println!(
            "{}",
            std::fs::read_to_string(f)
                .unwrap()
                .lines()
                .take(config.count)
                .collect::<Vec<&str>>()
                .join("\n")
        );
        if f_len > 1 && idx != f_len - 1 {
            println!();
        }
    })
}
