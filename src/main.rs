mod calculate;
mod format;
mod parser;

use calculate::calculate;
use parser::Parser;

fn main() {
    let input = std::env::args().skip(1).fold(String::new(), |mut acc, e| {
        acc.push(' ');
        acc.push_str(&e);

        acc
    });

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
