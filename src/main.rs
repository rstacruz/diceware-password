extern crate clap;
use clap::{App, Arg};
mod generator;
mod words;

use generator::Generator;

fn main() {
    let app = App::new("Passgen")
        .about("Generates passwords")
        .version("0.0.1")
        .arg(
            Arg::with_name("no-spaces")
                .short("s")
                .long("no-spaces")
                .help("Remove spaces"),
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

    let gen = gen.spaces(!matches.is_present("no-spaces"));
    print!("{}", gen.generate())
}
