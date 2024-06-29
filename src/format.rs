use std::fmt;

/// The unit that is shown.
#[derive(PartialEq, Debug)]
pub enum Unit {
    Byte,
    KiB,
    MiB,
    GiB,
    TiB,
    PiB,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Unit::Byte => "bytes",
                Unit::KiB => "KiB",
                Unit::MiB => "MiB",
                Unit::GiB => "GiB",
                Unit::TiB => "TiB",
                Unit::PiB => "PiB",
            }
        )
    }
}

impl Unit {
    /// Returns the number of bytes to the power of 2 in this [Unit].
    /// So a [`Byte`][Unit::Byte] will be `1`, and a Kibibyte will be `1024`, and so on.
    pub fn as_bytes(&self) -> u64 {
        1 << match self {
            Unit::Byte => 0,
            Unit::KiB => 10,
            Unit::MiB => 20,
            Unit::GiB => 30,
            Unit::TiB => 40,
            Unit::PiB => 50,
        }
    }
}

/// Determines the best unit to use for the given quantity.
pub fn best_unit(quantity: u64) -> Unit {
    if (quantity >> 10) == 0 {
        return Unit::Byte;
    }

    if (quantity >> 20) == 0 {
        return Unit::KiB;
    }

    if (quantity >> 30) == 0 {
        return Unit::MiB;
    }

    if (quantity >> 40) == 0 {
        return Unit::GiB;
    }

    if (quantity >> 50) == 0 {
        return Unit::TiB;
    }

    if (quantity >> 60) == 0 {
        return Unit::PiB;
    }

    Unit::Byte
}

pub fn pretty_quantity(quantity: u64) -> String {
    let unit = best_unit(quantity);

    let res = quantity / unit.as_bytes();
    let frac = quantity % unit.as_bytes();
    format!("{res}.{frac:#02} {unit}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_quantity() {
        assert_eq!("10.00 KiB", pretty_quantity(1_024 * 10));

        assert_eq!("220.302 KiB", pretty_quantity(1_834 * 123));

        assert_eq!("18.365632 MiB", pretty_quantity(1_924 * 100 * 100));

        assert_eq!(
            "109.192115900416 TiB",
            pretty_quantity(1_024 * 1_024 * 1_700 * 1_924 * 35)
        );
    }

    #[test]
    fn test_best_unit() {
        assert_eq!(Unit::Byte, best_unit(0));
        assert_eq!(Unit::KiB, best_unit(1_024));

        assert_eq!(Unit::MiB, best_unit(1_024 * 1_024));
        assert_eq!(Unit::MiB, best_unit(1_024 * 1_024 + 1));

        assert_eq!(Unit::GiB, best_unit(1_024 * 1_024 * 1_024));
        assert_eq!(Unit::GiB, best_unit(1_024 * 1_024 * 1_024 + 1));

        assert_eq!(Unit::TiB, best_unit(1_024 * 1_024 * 1_024 * 1_024));
    }

    #[test]
    fn unit_display() {
        assert_eq!("bytes", format!("{}", Unit::Byte));
        assert_eq!("KiB", format!("{}", Unit::KiB));
        assert_eq!("MiB", format!("{}", Unit::MiB));
        assert_eq!("GiB", format!("{}", Unit::GiB));
        assert_eq!("TiB", format!("{}", Unit::TiB));
        assert_eq!("PiB", format!("{}", Unit::PiB));
    }
}
