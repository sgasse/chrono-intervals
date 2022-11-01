//! # chrono-intervals: Grouped time intervals for Rust
//!
//! Create chrono time intervals as "per-day", "per-week" etc.
//!
//! ## Usage
//!
//! There are two main functions to get intervals based on a `begin` and `end`:
//! - [get_extended_utc_intervals] returns grouped intervals which enclose the
//!   `begin` and `end` and have a precision of 1ms.
//! - [get_utc_intervals_opts] returns grouped intervals and allows the user
//!   to specify whether the first/last interval should be extended to next
//!   boundary and which precision to use.
//!
//! ### Examples
//!
//! Get daily intervals between two times with default options:
//! ```rust
//! use chrono::{DateTime, TimeZone, Utc};
//! use chrono_intervals::{Grouping, get_extended_utc_intervals};
//!
//! let begin = DateTime::parse_from_rfc3339("2022-06-25T08:23:45.000000Z").unwrap();
//! let end = DateTime::parse_from_rfc3339("2022-06-27T09:31:12.000000Z").unwrap();
//!
//! let daily_intervals =
//!     get_extended_utc_intervals(begin, end, &Grouping::PerDay, 0);
//! assert_eq!(
//!     daily_intervals,
//!     vec![
//!         (
//!             Utc.ymd(2022, 6, 25).and_hms(0, 0, 0),
//!             Utc.ymd(2022, 6, 25).and_hms_milli(23, 59, 59, 999),
//!         ),
//!         (
//!             Utc.ymd(2022, 6, 26).and_hms(0, 0, 0),
//!             Utc.ymd(2022, 6, 26).and_hms_milli(23, 59, 59, 999),
//!         ),
//!         (
//!             Utc.ymd(2022, 6, 27).and_hms(0, 0, 0),
//!             Utc.ymd(2022, 6, 27).and_hms_milli(23, 59, 59, 999),
//!         ),
//!     ]
//! );
//! ```
//!
//! Get monthly intervals with default options in the Pacific Daylight Time
//! (PDT) timezone:
//! ```rust
//! use chrono::{DateTime, TimeZone, Utc};
//! use chrono_intervals::{Grouping, get_extended_utc_intervals};
//!
//! // We want to obtain daily intervals for days in PDT instead of in UTC.
//! let begin = DateTime::parse_from_rfc3339("2022-06-10T12:23:45.000000-07:00").unwrap();
//! let end = DateTime::parse_from_rfc3339("2022-08-26T12:23:45.000000-07:00").unwrap();
//!
//! // PDT is 7h behind of UTC (towards the **west**), thus the
//! // `offset_west_seconds` are 7*3600
//! let pdf_offset_west_seconds = 7 * 3600;
//!
//! let monthly_intervals =
//!     get_extended_utc_intervals(begin, end, &Grouping::PerMonth, pdf_offset_west_seconds);
//! // In UTC, we expect the intervals to start 7h after the day boundary.
//! assert_eq!(
//!     monthly_intervals,
//!     vec![
//!         (
//!             Utc.ymd(2022, 6, 1).and_hms(7, 0, 0),
//!             Utc.ymd(2022, 7, 1).and_hms_milli(6, 59, 59, 999),
//!         ),
//!         (
//!             Utc.ymd(2022, 7, 1).and_hms(7, 0, 0),
//!             Utc.ymd(2022, 8, 1).and_hms_milli(6, 59, 59, 999),
//!         ),
//!         (
//!             Utc.ymd(2022, 8, 1).and_hms(7, 0, 0),
//!             Utc.ymd(2022, 9, 1).and_hms_milli(6, 59, 59, 999),
//!         ),
//!     ]
//! );
//! ```
//!
//! ### Configuration options
//!
//! The following things can be configured:
//! - With which precision is the end of an interval before the next interval?
//!   The default is 1ms, but other `Duration`s can be passed as
//!   `end_precision`.
//! - Should the first interval start on the boundary before or after `begin`?
//!   This is controlled with `extend_begin`.
//! - Should the last interval stop at the boundary before or after `end`? This
//!   is controlled with `extend_end`.
//!
//! An example call might look like this:
//!
//! ```rust
//! use chrono::{DateTime, Duration, TimeZone, Utc};
//! use chrono_intervals::{Grouping, get_utc_intervals_opts};
//!
//! let begin = DateTime::parse_from_rfc3339("2022-06-15T08:23:45.000000Z").unwrap();
//! let end = DateTime::parse_from_rfc3339("2022-06-30T09:31:12.000000Z").unwrap();
//!
//! let weekly_intervals =
//!     get_utc_intervals_opts(
//!         begin,
//!         end,
//!         &Grouping::PerWeek,
//!         0,
//!         Duration::microseconds(1),  // interval end is 1µs before the next
//!         false,                      // start on the boundary after `start`
//!         true,                       // end at the boundary after `end`
//!     );
//! assert_eq!(
//!     weekly_intervals,
//!     vec![
//!         (
//!             // First interval begins **after** `begin`
//!             Utc.ymd(2022, 6, 20).and_hms(0, 0, 0),
//!             Utc.ymd(2022, 6, 26).and_hms_micro(23, 59, 59, 999999),
//!         ),
//!         (
//!             Utc.ymd(2022, 6, 27).and_hms(0, 0, 0),
//!             // Last interval ends **after** `end`
//!             Utc.ymd(2022, 7, 3).and_hms_micro(23, 59, 59, 999999),
//!         ),
//!     ]
//! );
//! ```
//!
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
