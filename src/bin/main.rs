extern crate clap;
use clap::{App, Arg};

use r_jvm;

fn main() {
    let matches = App::new("rj")
        .version("0.1")
        .author("rchaser53 <tayoshizawa29@gmail.com>")
        .about("toy jvm implemented by Rust")
        .arg(
            Arg::with_name("debug")
                .help("emits the debug information")
                .long("debug")
                .takes_value(true),
        )
        .args_from_usage(
            "
            <INPUT>              'Sets the input file to use'",
        )
        .get_matches();

    if let Some(file_name) = matches.value_of("INPUT") {
        r_jvm::execute(
            file_name.to_string(),
            matches
                .value_of("debug")
                .unwrap_or("0")
                .parse::<usize>()
                .unwrap_or(0),
        );
    } else {
        println!("should input the file");
    }
}
