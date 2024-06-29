mod calculate;
mod format;
mod parser;

use calculate::calculate;
use itertools::Itertools;
use parser::Parser;

fn main() {
    let input: String = std::env::args().skip(1).join(" ");

    let parser = Parser::new();

    let symbols = parser.parse(&input);

    match calculate(symbols) {
        Ok(quantity) => {
            let formatted = format::pretty_quantity(quantity);
            println!("{formatted}");
        }
        Err(msg) => {
            eprintln!("{msg}");
        }
    }
}
