use chrono::{DateTime, TimeZone, Utc};
use rand::Rng;

pub fn random_time(start_year: i32) -> DateTime<Utc> {
    let mut rng = rand::thread_rng();
    Utc.ymd(
        start_year + rng.gen_range(0..100),
        rng.gen_range(1..=12),
        rng.gen_range(1..=28),
    )
    .and_hms(
        rng.gen_range(0..24),
        rng.gen_range(0..60),
        rng.gen_range(0..60),
    )
}
