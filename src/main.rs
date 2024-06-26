mod parser;

use itertools::Itertools;
use parser::Parser;

fn main() {
    let input: String = std::env::args().skip(1).join(" ");

    let parser = Parser::new();

    let symbols = parser.parse(&input);

    println!("Got symbols: {symbols:?}");
}
