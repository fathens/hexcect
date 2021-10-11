use derive_more::{From, Into};
use num_traits::{Float, FromPrimitive};
use std::time::Instant;

use super::{Nanoseconds, Seconds};

#[derive(Debug, From, Into, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(Instant);

impl Timestamp {
    pub fn now() -> Self {
        Self(Instant::now())
    }

    /**
    与えられた Timestamp との差を返す。
    `other` は自分より前でも後でも構わない。

    # Examples
    ```
    use controller::model::Timestamp;

    let dur = std::time::Duration::from_secs(3);
    let a = std::time::Instant::now();
    let b = a + dur;

    let t_a: Timestamp = a.into();
    let t_b: Timestamp = b.into();

    let diff_a: f64 = t_b.diff(&t_a).into();
    let diff_b: f64 = t_a.diff(&t_b).into();

    assert_eq!(diff_a, dur.as_secs_f64());
    assert_eq!(diff_a, diff_b);
    ```
    */
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
        a.0.duration_since(b.0).into()
    }

    /**
    現時刻との差と現時刻の Timestamp とを返す。

    # Examples
    ```
    use controller::model::Timestamp;

    let dur = std::time::Duration::from_secs(1);
    let a = Timestamp::now();
    std::thread::sleep(dur);

    let (r, now) = a.past_dur();
    let diff_a: f64 = r.into();
    let diff_b: f64 = a.diff(&now).into();

    assert_eq!(diff_a, diff_b);
    ```
    */
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

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use std::time::Duration;

    use super::*;

    #[test]
    fn now_is_now() {
        let a = Timestamp::now();
        let b = Instant::now();
        let d = a.diff(&b.into());
        assert_relative_eq!(0.1_f64.powi(6), d.into(), max_relative = 1.0);
    }

    #[test]
    fn diff_later() {
        let dur = Duration::from_secs(3);
        let a = Instant::now();
        let b = a + dur;
        let r = Timestamp(a).diff(&Timestamp(b));
        assert_eq!(dur.as_secs_f64(), r.into());
    }

    #[test]
    fn diff_earlier() {
        let dur = Duration::from_secs(3);
        let a = Instant::now();
        let b = a + dur;
        let r = Timestamp(b).diff(&Timestamp(a));
        assert_eq!(dur.as_secs_f64(), r.into());
    }

    #[test]
    fn past_1second() {
        let dur = Duration::from_secs(1);
        let a = Timestamp(Instant::now());
        std::thread::sleep(dur);
        let (r, now) = a.past_dur();
        assert_eq!(r, a.diff(&now));
        assert_relative_eq!(dur.as_secs_f64(), r.into(), max_relative = 0.1);
    }

    #[test]
    fn past_1second_future() {
        let dur = Duration::from_secs(1);
        let a = Timestamp(Instant::now() + dur);
        let (r, now) = a.past_dur();
        assert_eq!(r, a.diff(&now));
        assert_relative_eq!(dur.as_secs_f64(), r.into(), max_relative = 0.1);
    }
}
