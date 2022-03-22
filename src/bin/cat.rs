/// the usage of BSD cat
/// cat [-belnstuv] [file ...]
/// [ ] 1. impl -belnstuv
/// [x] 2. accpet multiple file
///
/// -b Number the non-blank output lines, starting at 1.
/// -s combine multiple empty line into one empty line.
extern crate clap;
extern crate rust_coreutil;
use std::io::{BufRead};

use clap::{Arg, Command};
use rust_coreutil::open;

fn main() {
    // TODO if cat take no arguments, it became an echor
    let app = Command::new("cat")
        .about("Rust cat commandline app")
        .author("qinyuhangxiaoxiang@gmail.com")
        .arg(Arg::new("line number")
            .short('n')
            .takes_value(false)
            .help("Number the output lines, starting at 1.")
        )
        .arg(
            Arg::new("not blank line number")
                .short('b')
                .takes_value(false)
                .help("Number the non-blank output lines, starting at 1.")
        )
        .arg(
            Arg::new("Display non-printing characters")
                .short('e')
                .help("Display non-printing characters (see the -v option), and display a dollar sign (‘$’) at the end of each line."))
        .arg(
            Arg::new("file name")
                .takes_value(true)
                .min_values(0)
                .allow_invalid_utf8(true),
        );

    let m = app.get_matches();

    let file_names = m.values_of_lossy("file name").unwrap_or(vec!["-".to_string()]);
    file_names.iter().for_each(|f| {
        match open(f) {
            Err(err) => {eprint!("cat: {}: {}", f, err)},
            Ok(fi) => {
                // this code works fine with stdin
                let mut last_line_num = 0;
                fi.lines().into_iter().enumerate()
                .for_each(|(idx, fic)| {
                    let line = fic.unwrap();
                    if m.is_present("line number") {
                        println!("{:>6}\t{}", idx + 1, line);
                    } else if m.is_present("not blank line number") {
                        if !line.is_empty() {
                            last_line_num += 1;
                            println!("{:>6}\t{}", last_line_num, line)
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                });
                
                // let x = match std::fs::read_to_string(*fi) {
                //     Ok(r) => r,
                //     Err(e) => format!("cat: {}: {}", f, e.to_string()),
                // };
            },
        };
        // todo ? __dirname ?
        // let x = match std::fs::read_to_string(f) {
        //     Ok(r) => r,
        //     Err(e) => format!("cat: {}: {}", f, e.to_string()),
        // };
        // if m.is_present("line number") {
        //     // if -n add line number to x
        //     print!(
        //         "{}",
        //         x.split("\n")
        //             .enumerate()
        //             // todo handle -n -s -b
        //             .map(|(line, content)| { format!("{} {}", line + 1, content) })
        //             .collect::<Vec<String>>()
        //             .join("\n")
        //     );
        // } else {
        //     print!("{}", x);
        // };
    });
}
