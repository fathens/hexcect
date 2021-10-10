use std::time::Instant;

use num_traits::{Float, FromPrimitive};

use super::{Nanoseconds, Seconds};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(Instant);

impl Timestamp {
    pub fn now() -> Self {
        Self(Instant::now())
    }

    pub fn diff<V>(&self, other: &Self) -> Seconds<V>
    where
        V: Float,
        V: FromPrimitive,
        V: From<Nanoseconds<V>>,
    {
        let (a, b) = if self > other {
            (self, other)
        } else {
            (other, self)
        };
        (a.0 - b.0).into()
    }

    pub fn past_dur<V>(&self) -> (Seconds<V>, Timestamp)
    where
        V: Float,
        V: FromPrimitive,
        V: From<Nanoseconds<V>>,
    {
        let now = Timestamp::now();
        (self.diff(&now), now)
    }
}
