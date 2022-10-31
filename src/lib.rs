//! # chrono-intervals
//!
//! Library to compute time intervals grouped per-day, per week etc.
pub mod grouping;
mod intervals;

use chrono::DateTime;
pub use intervals::{get_extended_utc_intervals_with_defaults, get_utc_intervals_opts};

/// Error type of the crate.
pub type Error = Box<dyn std::error::Error>;

/// A tuple of `chrono::DateTime` objects forming a time interval.
pub type TimeIntervalTuple<T> = (DateTime<T>, DateTime<T>);
