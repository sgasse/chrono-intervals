//! # chrono-intervals: Grouped time intervals for Rust
//!
//! Create chrono time intervals as "per-day", "per-week" etc.
//!
//! ## Usage
//!
//! Get daily intervals between two times with default options:
//! ```rust
//! use chrono::{DateTime, Utc};
//! use chrono_intervals::{Grouping, get_extended_utc_intervals};
//!
//! let begin = DateTime::parse_from_rfc3339("2022-06-25T08:23:45.000000Z").unwrap();
//! let end = DateTime::parse_from_rfc3339("2022-06-27T08:23:45.000000Z").unwrap();
//!
//! let daily_intervals =
//!     get_extended_utc_intervals(begin, end, &Grouping::PerDay, 0);
//!
//! // daily_intervals:
//! // [
//! //     (
//! //         2022-06-25T00:00:00Z,
//! //         2022-06-25T23:59:59.999Z,
//! //     ),
//! //     (
//! //         2022-06-26T00:00:00Z,
//! //         2022-06-26T23:59:59.999Z,
//! //     ),
//! //     (
//! //         2022-06-27T00:00:00Z,
//! //         2022-06-27T23:59:59.999Z,
//! //     ),
//! // ]
//! ```
mod builder;
mod grouping;
mod intervals;
mod intervals_impl;

use chrono::DateTime;
pub use grouping::Grouping;
pub use intervals::{get_extended_utc_intervals, get_utc_intervals_opts};

/// Error type of the crate.
pub type Error = Box<dyn std::error::Error>;

/// A tuple of `chrono::DateTime` objects forming a time interval.
pub type TimeInterval<T> = (DateTime<T>, DateTime<T>);
