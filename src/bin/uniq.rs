/// BSD/GNU uniq
/// uniq [-c | -d | -D | -u] [-i] [-f num] [-s chars] [input_file [output_file]]
use clap::Parser;
use rust_coreutil::open;
use std::io::prelude::*;

#[derive(Debug, clap::ArgEnum, Clone)]
enum AllRepeated {
    /// Do not separate groups of lines (this is the default).
    None,
    Prepend,
    Separate,
}

#[derive(Debug, Parser)]
#[clap(name = "uniq")]
struct AppConfig {
    /// Precede each output line with the count of the number of times the line occurred in the input, followed by a single space.
    #[clap(short = 'c', long = "count")]
    count: bool,

    /// Output a single copy of each line that is repeated in the input.
    #[clap(short = 'd', long = "repeated")]
    repeated: bool,

    /// Output all lines that are repeated (like -d, but each copy of the repeated line is written).
    /// The optional septype argument controls how to separate groups of repeated
    /// lines in the output; it must be one of the following values:
    ///
    /// none      Do not separate groups of lines (this is the default).
    ///
    /// prepend   Output an empty line before each group of lines.
    ///
    /// separate  Output an empty line after each group of lines.
    #[clap(short='D', long="all-repeated", arg_enum, default_value_t=AllRepeated::None)]
    all_repeated: AllRepeated,

    /// Ignore the first num fields in each input line when doing comparisons.
    /// A field is a string of non-blank characters separated from adjacent fields by blanks.
    /// Field numbers are one based, i.e., the first field is field one.
    #[clap(short = 'f', long = "skip-fields")]
    skip_fields: Option<i32>,

    /// Case insensitive comparison of lines.
    #[clap(short = 'i', long = "ignore-case")]
    ignore_case: bool,

    /// Ignore the first chars characters in each input line when doing comparisons.
    /// If specified in conjunction with the -f, --unique option, the first chars characters after
    /// the first num fields will be ignored.
    /// Character numbers are one based, i.e., the first character is character one.
    #[clap(short = 's', long = "skip-chars")]
    skip_chars: Option<i32>,

    /// Only output lines that are not repeated in the input.
    #[clap(short = 'u', long = "unique")]
    unique: bool,

    /// file name
    #[clap(default_value = "-")]
    files: Vec<String>,
}

fn format_print(config: &AppConfig, line_data: (&String, &u32)) {
    if config.count {
        print!("{:>4} {}", line_data.1, line_data.0);
    } else {
        print!("{}", line_data.0);
    }
}
/// return a hashMap<String, i8>
fn make_uniqu_collection() -> std::collections::HashMap<String, u32> {
    std::collections::HashMap::<String, u32>::new()
}
fn main() {
    let config = AppConfig::parse();

    config.files.iter().for_each(|f| {
        let mut collection = make_uniqu_collection();
        let mut orderd_colloection = vec![];
        // use hashMap to store the lines and insert into collection
        let mut f = open(f).unwrap();
        let mut line = String::new();
        loop {
            let size = f.read_line(&mut line).unwrap();
            if size == 0 {
                break;
            }
            if line == "\n".to_string() {
                collection.insert(line.clone(), 1);
                orderd_colloection.push(line.clone());
                line.clear();
                continue;
            }
            if let Some(l) = collection.get_mut(&line) {
                *l += 1;
                line.clear();
                continue;
            } else {
                collection.insert(line.clone(), 1);
                orderd_colloection.push(line.clone());
            }
            line.clear();
        }

        // TODO hasMap is unorderd! bug fixme
        orderd_colloection.iter().for_each(|line_data| {
            format_print(&config, (&line_data, collection.get(line_data).unwrap()));
        });
    });

    // println!("{:?}", config);
}
