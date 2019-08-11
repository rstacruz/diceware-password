//! Utilities for generating random passwords
//!
//! We generate random passwords with this.
//!
//! ```
//! let password = Generator::new().generate();
//! println!("{}", password)
//! // => "correct horse Battery staple $20"
//! ```
//!
//! You can pass some options:
//!
//! ```
//! let password = Generator::new()
//!   .use_spaces(false)
//!   .word_count(3)
//!   .generate();
//! println!("{}", password)
//! // => "Correct-horse-staple-$20"
//! ```

extern crate clap;
use clap::{App, Arg};

mod generator;
mod symbols;
mod words;

pub use generator::Generator;

/// The CLI runner
fn main() {
    let app = App::new("Passgen")
        .about("Generates passwords")
        .version("0.0.1")
        .arg(
            Arg::with_name("no-spaces")
                .short("s")
                .long("no-spaces")
                .help("Remove spaces"),
        )
        .arg(
            Arg::with_name("newline")
                .short("n")
                .long("newline")
                .help("Print ending newline"),
            // )
            // .arg(
            //     Arg::with_name("no-symbols")
            //         .short("y")
            //         .long("no-symbols")
            //         .help("Remove symbols"),
            // )
            // .arg(
            //     Arg::with_name("words")
            //         .short("w")
            //         .long("words")
            //         .help("Number of words")
            //         .value_name("N")
            //         .takes_value(true),
            // )
        );

    let matches = app.get_matches();
    // println!("{:?}", matches);

    let gen = Generator::new().word_count(4);

    let gen = gen.use_spaces(!matches.is_present("no-spaces"));

    if matches.is_present("newline") {
        println!("{}", gen.generate())
    } else {
        print!("{}", gen.generate())
    };
}
