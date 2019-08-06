mod generator;
mod words;

use generator::Generator;

fn main() {
    let gen = Generator::new().word_count(4).spaces(true);

    println!("{}", gen.generate());
}
