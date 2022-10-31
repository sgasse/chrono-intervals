pub mod grouping;
pub mod intervals;

use chrono::DateTime;

pub type Error = Box<dyn std::error::Error>;
pub type TimeIntervalTuple<T> = (DateTime<T>, DateTime<T>);
