//! Time intervals grouping.
//!
//! Specify in which chunks time intervals should be grouped. Time intervals
//! with e.g. `Grouping::PerDay` have a length of 24h minus the duration of
//! `end_precision` (default 1ms).
//! Intervals per week start on Monday and end on Sunday night.
pub enum Grouping {
    PerDay,
    PerWeek,
    PerMonth,
}
