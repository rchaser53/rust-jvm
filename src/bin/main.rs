extern crate clap;
use clap::App;

use r_jvm;

fn main() {
    let matches = App::new("rj")
        .version("0.1")
        .author("rchaser53 <tayoshizawa29@gmail.com>")
        .about("toy jvm implemented by Rust")
        .args_from_usage(
            "
            <INPUT>              'Sets the input file to use'
            --debug              'emits the debug information'",
        )
        .get_matches();

    if let Some(file_name) = matches.value_of("INPUT") {
        r_jvm::execute(file_name.to_string(), matches.occurrences_of("debug") == 1);
    } else {
        println!("should input the file");
    }
}
