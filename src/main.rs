mod calculate;
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
            let formatted = format_quantity(quantity);
            println!("{formatted}");
        }
        Err(msg) => {
            eprintln!("{msg}");
        }
    }
}

fn format_quantity(quantity: u64) -> String {
    let round = |q: u64, power: u32| (q as f64) / (u64::pow(2, power)) as f64;

    let (rounded_quantity, suffix) = match (quantity as f64).log2() {
        10.0..=20.0 => (round(quantity, 10), "KiB"),
        20.0..=30.0 => (round(quantity, 20), "MiB"),
        30.0..=40.0 => (round(quantity, 30), "GiB"),
        40.0..=50.0 => (round(quantity, 40), "TiB"),
        50.0..=60.0 => (round(quantity, 50), "PiB"),
        _ => (quantity as f64, ""),
    };

    format!("{rounded_quantity:.2} {suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_quantity_kib() {
        assert_eq!("10.00 KiB", format_quantity(1_024 * 10));

        assert_eq!("220.29 KiB", format_quantity(1_834 * 123));

        assert_eq!("18.35 MiB", format_quantity(1_924 * 100 * 100));
    }
}
