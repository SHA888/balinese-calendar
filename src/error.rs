// src/error.rs
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error, PartialEq)]
pub enum BalineseDateError {
    #[error("Date out of supported range: year must be between 1800 and 2200 CE")]
    OutOfRange,

    #[error("Invalid date: {year}-{month:02}-{day:02}")]
    InvalidDate { year: i32, month: u32, day: u32 },

    #[error("Julian Day Number overflow")]
    JdnOverflow,

    #[error("invalid boundary hour {0}: must be 0–23")]
    InvalidBoundaryHour(u8),

    #[error("Not implemented: {0}")]
    NotImplemented(String),
}
