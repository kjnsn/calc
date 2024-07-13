use regex::Regex;
use std::vec;

#[derive(Debug, PartialEq)]
pub enum OperatorKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    /// Some quantity of bytes.
    Quantity(u64),
    /// A number that is not representative of a quantity.
    Number(u64),
    /// An operator, See `OperatorKind`.
    Operator(OperatorKind),
}

pub struct Parser {
    number_regex: Regex,
    quantity_regex: Regex,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            number_regex: Regex::new(r"^(\d+)").unwrap(),
            quantity_regex: Regex::new(r"^(?<quantity>\d+)\s*(?<unit>(?:K|M|G|T|P)?i?(?:b|B))")
                .unwrap(),
        }
    }

    pub fn parse(&self, input: &str) -> Vec<Symbol> {
        if input.is_empty() {
            return vec![];
        }

        if let (Some(symbol), remaining) = self.parse_arg(input) {
            return vec![symbol]
                .into_iter()
                .chain(self.parse(remaining))
                .collect();
        }

        vec![]
    }

    /// Consumes a symbol, and returns that symbol plus any remaining input.
    fn parse_arg<'a>(&'a self, arg: &'a str) -> (Option<Symbol>, &str) {
        let trimmed = arg.trim();
        // Check the quantity regex first.
        if let Some((_, [quantity, unit])) = self
            .quantity_regex
            .captures(trimmed)
            .map(|caps| caps.extract())
        {
            let remaining = &trimmed[self.quantity_regex.find(trimmed).unwrap().end()..];
            return (
                quantity
                    .parse::<u64>()
                    .ok()
                    .map(|q| Symbol::Quantity(Parser::quantity_with_unit(q, unit))),
                remaining,
            );
        }

        // Check the number second.
        if let Some((_, [number])) = self
            .number_regex
            .captures(trimmed)
            .map(|caps| caps.extract())
        {
            let remaining = &trimmed[self.number_regex.find(trimmed).unwrap().end()..];
            return (number.parse::<u64>().ok().map(Symbol::Number), remaining);
        }

        if let Some(op) = Parser::parse_operator(trimmed).map(Symbol::Operator) {
            return (Some(op), &trimmed[1..]);
        }

        (None, trimmed)
    }

    fn parse_operator(arg: &str) -> Option<OperatorKind> {
        match &arg[..1] {
            "+" => Some(OperatorKind::Add),
            "-" => Some(OperatorKind::Subtract),
            "/" => Some(OperatorKind::Divide),
            "*" => Some(OperatorKind::Multiply),
            _ => None,
        }
    }

    /// Converts a number & unit into a number of bytes.
    fn quantity_with_unit(quantity: u64, unit: &str) -> u64 {
        match unit {
            "PiB" => quantity * u64::pow(2, 50),
            "TiB" => quantity * u64::pow(2, 40),
            "GiB" => quantity * u64::pow(2, 30),
            "MiB" => quantity * u64::pow(2, 20),
            "KiB" => quantity * u64::pow(2, 10),
            // No conversion for bytes or unknown values.
            _ => quantity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_arg_empty_string() {
        let parser = Parser::new();

        assert!(parser.parse("").is_empty());
    }

    #[test]
    fn parse_arg_number() {
        let parser = Parser::new();

        let expected: Vec<Symbol> = vec![Symbol::Number(1234)];
        assert_eq!(expected, parser.parse("1234"));
    }

    #[test]
    fn parse_arg_addition() {
        let parser = Parser::new();

        let expected: Vec<Symbol> = vec![
            Symbol::Quantity(12_884_901_888),
            Symbol::Operator(OperatorKind::Add),
            Symbol::Quantity(14_680_064),
            Symbol::Operator(OperatorKind::Multiply),
            Symbol::Number(2),
        ];
        assert_eq!(expected, parser.parse("12 GiB + 14MiB * 2"));
    }
}
