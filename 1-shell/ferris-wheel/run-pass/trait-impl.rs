// FIXME: Make me pass! Diff budget: 25 lines.

use std::cmp::PartialEq;

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16),
}

impl Duration {
    fn ms(&self) -> u64 {
        match *self {
            Duration::MilliSeconds(m) => m,
            Duration::Seconds(s) => (s as u64 * 1000) as u64,
            Duration::Minutes(m) => (m as u64 * 60 * 1000) as u64,
        }
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.ms() == other.ms()
    }
}

fn main() {
    assert_eq!(Duration::Seconds(120), Duration::Minutes(2));
    assert_eq!(Duration::Seconds(420), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(420000), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(43000), Duration::Seconds(43));
}
