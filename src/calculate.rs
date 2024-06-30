use crate::parser::{OperatorKind, Symbol};

pub fn calculate(input: Vec<Symbol>) -> Result<u64, &'static str> {
    let mut result: u64 = 0;
    let mut last_operator: Option<OperatorKind> = None;

    for symbol in input {
        match symbol {
            Symbol::Quantity(quantity) => match last_operator {
                Some(op) => {
                    result = apply_op(op, result, quantity);
                    last_operator = None;
                }
                None => {
                    result = quantity;
                }
            },
            Symbol::Number(number) => match last_operator {
                Some(op) => {
                    result = apply_op(op, result, number);
                    last_operator = None;
                }
                None => {
                    result = number;
                }
            },
            Symbol::Operator(op) => {
                last_operator = Some(op);
            }
        }
    }

    if last_operator.is_some() {
        return Err("Expecting another number, but input finished with an operator");
    }

    Ok(result)
}

fn apply_op(op: OperatorKind, a: u64, b: u64) -> u64 {
    match op {
        OperatorKind::Add => a + b,
        OperatorKind::Subtract => a - b,
        OperatorKind::Multiply => a * b,
        OperatorKind::Divide => a / b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_empty_input() {
        assert_eq!(Ok(0), calculate(vec![]));
    }

    #[test]
    fn calculate_single_quantity() {
        assert_eq!(Ok(1_024), calculate(vec![Symbol::Quantity(1_024)]));
    }

    #[test]
    fn calculate_addition() {
        assert_eq!(
            Ok(2_048),
            calculate(vec![
                Symbol::Quantity(1_024),
                Symbol::Operator(OperatorKind::Add),
                Symbol::Quantity(1_024)
            ])
        );
    }

    #[test]
    fn calculate_multiply() {
        assert_eq!(
            Ok(5120),
            calculate(vec![
                Symbol::Quantity(1_024),
                Symbol::Operator(OperatorKind::Multiply),
                Symbol::Number(5),
            ])
        );
    }
}
