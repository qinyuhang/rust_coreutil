use clap::{Arg, Command, ArgMatches};
fn main() {
    let c = Command::new("echo")
        .version("0.1.0")
        .author("qinyuhangxiaoxiang@gmail.com")
        .about("Rust echo")
        .arg(Arg::new("no new line").short('n'))
        .arg(
            Arg::new("other")
                .takes_value(true)
                .min_values(0)
                .allow_invalid_utf8(true),
        );

    let m = c.get_matches();

    if m.is_present("no new line") {
        print!("{}", pure_std_impl());
    } else {
        println!("{}", pure_std_impl());
    }
}

#[allow(dead_code)]
fn pure_std_impl() -> String {
    let mut r = std::env::args().skip(1).collect::<Vec<String>>();
    r.retain(|c| c != "-n");
    r.join(" ")
}

#[allow(dead_code)]
fn clap_impl(m: &ArgMatches) -> String {
    m.values_of_lossy("other").unwrap().join(" ").to_string()
}
