extern crate chrono;
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    const gigasecond: i64 = 1_000_000_000;
    start + Duration::seconds(gigasecond)
}
