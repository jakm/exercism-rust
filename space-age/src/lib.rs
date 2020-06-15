// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    duration: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { duration: s as f64 }
    }
}

pub trait Planet {
    const YEARS_IN_SECONDS: f64;
    fn years_during(d: &Duration) -> f64 {
        d.duration / Self::YEARS_IN_SECONDS
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const YEARS_IN_SECONDS: f64 = 0.240_846_7_f64 * Earth::YEARS_IN_SECONDS;
}
impl Planet for Venus {
    const YEARS_IN_SECONDS: f64 = 0.615_197_26_f64 * Earth::YEARS_IN_SECONDS;
}
impl Planet for Earth {
    const YEARS_IN_SECONDS: f64 = 31_557_600_f64;
}
impl Planet for Mars {
    const YEARS_IN_SECONDS: f64 = 1.880_815_8_f64 * Earth::YEARS_IN_SECONDS;
}
impl Planet for Jupiter {
    const YEARS_IN_SECONDS: f64 = 11.862_615_f64 * Earth::YEARS_IN_SECONDS;
}
impl Planet for Saturn {
    const YEARS_IN_SECONDS: f64 = 29.447_498_f64 * Earth::YEARS_IN_SECONDS;
}
impl Planet for Uranus {
    const YEARS_IN_SECONDS: f64 = 84.016_846_f64 * Earth::YEARS_IN_SECONDS;
}
impl Planet for Neptune {
    const YEARS_IN_SECONDS: f64 = 164.791_32_f64 * Earth::YEARS_IN_SECONDS;
}
