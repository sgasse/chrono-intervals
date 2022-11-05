//! Time interval generator.
use chrono::{DateTime, Duration, FixedOffset, TimeZone, Utc};

use crate::{intervals_impl::get_intervals_impl, Grouping, TimeInterval};

/// Generator for time intervals.
pub struct IntervalGenerator {
    grouping: Grouping,
    end_precision: Duration,
    local_timezone: FixedOffset,
    extend_begin: bool,
    extend_end: bool,
}

impl IntervalGenerator {
    pub fn new() -> Self {
        IntervalGenerator {
            grouping: Grouping::PerDay,
            end_precision: Duration::milliseconds(1),
            local_timezone: FixedOffset::west(0),
            extend_begin: true,
            extend_end: true,
        }
    }

    pub fn with_grouping(mut self, grouping: Grouping) -> Self {
        self.grouping = grouping;
        self
    }

    pub fn with_precision(mut self, precision: Duration) -> Self {
        self.end_precision = precision;
        self
    }

    pub fn with_offset_west_secs(mut self, offset_west_secs: i32) -> Self {
        self.local_timezone = FixedOffset::west(offset_west_secs);
        self
    }

    pub fn without_extended_begin(mut self) -> Self {
        self.extend_begin = false;
        self
    }

    pub fn without_extended_end(mut self) -> Self {
        self.extend_end = false;
        self
    }

    pub fn get_intervals<T>(&self, begin: DateTime<T>, end: DateTime<T>) -> Vec<TimeInterval<Utc>>
    where
        T: TimeZone,
    {
        get_intervals_impl(
            begin,
            end,
            &self.grouping,
            self.end_precision,
            &self.local_timezone,
            &Utc,
            self.extend_begin,
            self.extend_end,
        )
    }
}

impl Default for IntervalGenerator {
    fn default() -> Self {
        IntervalGenerator::new()
    }
}
