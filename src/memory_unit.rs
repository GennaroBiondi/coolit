use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseMemoryUnitError {
    #[error("invalid memory unit")]
    InvalidUnit,

    #[error("invalid memory unit amount")]
    InvalidAmount,
}

/// A memory unit, representing a multiple of a byte.
#[derive(Debug, Clone, Copy)]
pub enum MemoryUnit {
    /// 8 Bits.
    Byte(u64),
    /// 1000 Bytes.
    Kilo(u64),
    /// 1024 Bytes.
    KiloBinary(u64),
    /// 1000 Kb.
    Mega(u64),
    /// 1024 Kb.
    MegaBinary(u64),
    /// 1000 Mb.
    Giga(u64),
    /// 1024 Mb.
    GigaBinary(u64),
}

impl MemoryUnit {
    /// Returns the [`MemoryUnit`] converted into bytes.
    pub const fn as_byte_amount(&self) -> u64 {
        match self {
            Self::Byte(x) => *x,
            Self::Kilo(x) => *x * 1000,
            Self::KiloBinary(x) => *x * 1024,
            Self::Mega(x) => *x * 1000 * 1000,
            Self::MegaBinary(x) => *x * 1024 * 1024,
            Self::Giga(x) => *x * 1000 * 1000 * 1000,
            Self::GigaBinary(x) => *x * 1024 * 1024 * 1024,
        }
    }
}

impl Display for MemoryUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte(x) => write!(f, "{x}B"),
            Self::Kilo(x) => write!(f, "{x}Kb"),
            Self::KiloBinary(x) => write!(f, "{x}KiB"),
            Self::Mega(x) => write!(f, "{x}Mb"),
            Self::MegaBinary(x) => write!(f, "{x}MiB"),
            Self::Giga(x) => write!(f, "{x}Gb"),
            Self::GigaBinary(x) => write!(f, "{x}GiB"),
        }
    }
}

macro_rules! get_suffix_and_amount {
    ($s:expr, $suffix:literal $(, $rest:literal)*) => {
        if let Some(x) = $s.strip_suffix($suffix) {
            Some((x, $suffix))
        } else {
            get_suffix_and_amount!($s $(, $rest)*)
        }
    };

    ($s:expr) => {
        None
    };
}

impl FromStr for MemoryUnit {
    type Err = ParseMemoryUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        // Thank god macros exist, before it was a pretty long if chain.
        let (num, unit) = get_suffix_and_amount!(s, "gib", "mib", "kib", "gb", "mb", "kb", "b")
            .ok_or(ParseMemoryUnitError::InvalidUnit)?;

        let value = num
            .parse::<u64>()
            .map_err(|_| ParseMemoryUnitError::InvalidAmount)?;

        Ok(match unit {
            "b" => Self::Byte(value),
            "kb" => Self::Kilo(value),
            "mb" => Self::Mega(value),
            "gb" => Self::Giga(value),
            "kib" => Self::KiloBinary(value),
            "mib" => Self::MegaBinary(value),
            "gib" => Self::GigaBinary(value),
            _ => unreachable!(),
        })
    }
}
