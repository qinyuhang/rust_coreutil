/// the usage of BSD cat
/// cat [-belnstuv] [file ...]
/// [ ] 1. impl -belnstuv
/// [x] 2. accpet multiple file
///
/// -b Number the non-blank output lines, starting at 1.
extern crate clap;
use clap::{Arg, Command};

fn main() {
    // TODO if cat take no arguments, it became an echor
    let app = Command::new("cat")
        .about("Rust cat commandline app")
        .author("qinyuhangxiaoxiang@gmail.com")
        .arg(Arg::new("line number").short('b').takes_value(false))
        .arg(
            Arg::new("file name")
                .takes_value(true)
                .min_values(1)
                .allow_invalid_utf8(true),
        );

    let m = app.get_matches();

    let file_name = m.values_of_lossy("file name").unwrap_or(vec![]);
    file_name.iter().for_each(|f| {
        // todo ? __dirname ?
        let x = match std::fs::read_to_string(f) {
            Ok(r) => r,
            Err(e) => format!("cat: {}: {}", f, e.to_string()),
        };
        if m.is_present("line number") {
            // if -b add line number to x
            print!(
                "{}",
                x.split("\n")
                    .enumerate()
                    .map(|(line, content)| { format!("{} {}", line + 1, content) })
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        } else {
            print!("{}", x);
        };
    });
}
