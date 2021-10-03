use measure_units::*;
use num_traits::{Float, FromPrimitive, NumCast};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, CalcMix, Convertible, FloatStatus)]
#[calcmix(unit_name = "s".to_string())]
#[convertible(Milliseconds ^ 3)]
#[convertible(Nanoseconds ^ 9)]
pub struct Seconds<V>(V);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, CalcMix, Convertible, FloatStatus)]
#[calcmix(unit_name = "ms".to_string())]
#[convertible(Seconds ^ -3)]
#[convertible(Nanoseconds ^ 6)]
pub struct Milliseconds<V>(V);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, CalcMix, Convertible, FloatStatus)]
#[calcmix(unit_name = "ms".to_string())]
#[convertible(Seconds ^ -9)]
#[convertible(Milliseconds ^ -6)]
pub struct Nanoseconds<V>(V);

// ================================================================

pub trait Duration<V> {
    fn to_seconds(self) -> Seconds<V>;
    fn to_milliseconds(self) -> Milliseconds<V>;
    fn to_nanoseconds(self) -> Nanoseconds<V>;
}

impl<V> Duration<V> for Seconds<V>
where
    V: Float + FromPrimitive,
    V: From<Self>,
{
    fn to_seconds(self) -> Seconds<V> {
        self
    }

    fn to_milliseconds(self) -> Milliseconds<V> {
        self.into()
    }

    fn to_nanoseconds(self) -> Nanoseconds<V> {
        self.into()
    }
}

impl<V> Duration<V> for Milliseconds<V>
where
    V: Float + FromPrimitive,
    V: From<Self>,
{
    fn to_seconds(self) -> Seconds<V> {
        self.into()
    }

    fn to_milliseconds(self) -> Milliseconds<V> {
        self
    }

    fn to_nanoseconds(self) -> Nanoseconds<V> {
        self.into()
    }
}

impl<V> Duration<V> for Nanoseconds<V>
where
    V: Float + FromPrimitive,
    V: From<Self>,
{
    fn to_seconds(self) -> Seconds<V> {
        self.into()
    }

    fn to_milliseconds(self) -> Milliseconds<V> {
        self.into()
    }

    fn to_nanoseconds(self) -> Nanoseconds<V> {
        self
    }
}

// ================================================================

impl<V> From<std::time::Duration> for Seconds<V>
where
    V: Float + FromPrimitive,
    V: From<Nanoseconds<V>>,
{
    fn from(s: std::time::Duration) -> Self {
        let n = s.as_nanos();
        let v = <V as NumCast>::from(n).expect("u128 must be converted to Float.");
        Nanoseconds::from(v).into()
    }
}

impl<V> From<std::time::Duration> for Milliseconds<V>
where
    V: Float + FromPrimitive,
    V: From<Nanoseconds<V>>,
{
    fn from(s: std::time::Duration) -> Self {
        let n = s.as_nanos();
        let v = <V as NumCast>::from(n).expect("u128 must be converted to Float.");
        Nanoseconds::from(v).into()
    }
}

impl<V> From<std::time::Duration> for Nanoseconds<V>
where
    V: Float + FromPrimitive,
    V: From<Nanoseconds<V>>,
{
    fn from(s: std::time::Duration) -> Self {
        let n = s.as_nanos();
        let v = <V as NumCast>::from(n).expect("u128 must be converted to Float.");
        Nanoseconds::from(v)
    }
}

// ---------------------------------------------------------------

impl<V> From<Seconds<V>> for Option<std::time::Duration>
where
    V: Float + FromPrimitive,
    V: From<Seconds<V>>,
{
    fn from(s: Seconds<V>) -> Self {
        s.to_nanoseconds()
            .0
            .to_u64()
            .map(std::time::Duration::from_nanos)
    }
}

impl<V> From<Milliseconds<V>> for Option<std::time::Duration>
where
    V: Float + FromPrimitive,
    V: From<Milliseconds<V>>,
{
    fn from(s: Milliseconds<V>) -> Self {
        s.to_nanoseconds()
            .0
            .to_u64()
            .map(std::time::Duration::from_nanos)
    }
}

impl<V> From<Nanoseconds<V>> for Option<std::time::Duration>
where
    V: Float + FromPrimitive,
    V: From<Nanoseconds<V>>,
{
    fn from(s: Nanoseconds<V>) -> Self {
        s.to_nanoseconds()
            .0
            .to_u64()
            .map(std::time::Duration::from_nanos)
    }
}

// ================================================================

pub trait MkDuration<V> {
    fn seconds(self) -> Seconds<V>;
    fn milliseconds(self) -> Milliseconds<V>;
    fn nanoseconds(self) -> Nanoseconds<V>;
}

impl MkDuration<f32> for f32 {
    fn seconds(self) -> Seconds<f32> {
        self.into()
    }

    fn milliseconds(self) -> Milliseconds<f32> {
        self.into()
    }

    fn nanoseconds(self) -> Nanoseconds<f32> {
        self.into()
    }
}

impl MkDuration<f64> for f64 {
    fn seconds(self) -> Seconds<f64> {
        self.into()
    }

    fn milliseconds(self) -> Milliseconds<f64> {
        self.into()
    }

    fn nanoseconds(self) -> Nanoseconds<f64> {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_ulps_eq;

    #[test]
    fn conversions_s_ms() {
        let a: Seconds<f64> = 1.0_f64.seconds();
        let b: Milliseconds<f64> = a.into();
        assert_eq!(b.0, 1000_f64);

        let a: Milliseconds<f32> = 1.0_f32.milliseconds();
        let b: Seconds<f32> = a.into();
        assert_eq!(b.0, 0.001_f32);
    }

    #[test]
    fn conversions_s_nano() {
        let a: Seconds<f64> = 1.0_f64.seconds();
        let b: Nanoseconds<f64> = a.into();
        assert_eq!(b.0, 1000_000_000_f64);

        let a: Nanoseconds<f32> = 1.0_f32.nanoseconds();
        let b: Seconds<f32> = a.into();
        assert_eq!(b.0, 0.00_000_0001_f32);
    }

    #[test]
    fn conversions_ms_nano() {
        let a: Milliseconds<f64> = 1.0_f64.milliseconds();
        let b: Nanoseconds<f64> = a.into();
        assert_eq!(b.0, 1000_000_f64);

        let a: Nanoseconds<f32> = 1.0_f32.nanoseconds();
        let b: Milliseconds<f32> = a.into();
        assert_eq!(b.0, 0.00_0001_f32);
    }

    #[test]
    fn conversions_to() {
        let a: Nanoseconds<f32> = 1.0_f32.nanoseconds();
        let b: Milliseconds<f32> = a.to_milliseconds();
        let c: Seconds<f32> = a.to_seconds();

        assert_eq!(1_f32, a.to_nanoseconds().0);
        assert_eq!(1_f32, b.to_nanoseconds().0);
        assert_eq!(1_f32, c.to_nanoseconds().0);

        assert_eq!(0.00_0001_f32, a.to_milliseconds().0);
        assert_eq!(0.00_0001_f32, b.to_milliseconds().0);
        assert_eq!(0.00_0001_f32, c.to_milliseconds().0);

        assert_eq!(0.00_000_0001_f32, a.to_seconds().0);
        assert_eq!(0.00_000_0001_f32, b.to_seconds().0);
        assert_eq!(0.00_000_0001_f32, c.to_seconds().0);
    }

    #[test]
    fn additions() {
        let a: Seconds<f64> = 1_f64.seconds() + 1_f64.milliseconds();
        assert_ulps_eq!(a.0, 1.001_f64);
        let b: Milliseconds<f64> = 1_f64.milliseconds() + a;
        assert_ulps_eq!(b.0, 1002_f64);
        let c: Nanoseconds<f64> = 1_f64.nanoseconds() + b;
        assert_ulps_eq!(c.0, 1002_000_001_f64);
        let d: Milliseconds<f64> = 1_f64.milliseconds() + c;
        assert_ulps_eq!(d.0, 1003.000_001_f64);
        let e: Seconds<f64> = 1_f64.seconds() + d.to_nanoseconds();
        assert_ulps_eq!(e.0, 2.003_000_001_f64);
        let f: Nanoseconds<f64> = 1_f64.nanoseconds() + e;
        assert_ulps_eq!(f.0, 2003_000_002_f64);

        let a: Seconds<f32> = 1_f32.seconds() + 1_f32.milliseconds();
        assert_ulps_eq!(a.0, 1.001_f32);
        let b: Milliseconds<f32> = 1_f32.milliseconds() + a;
        assert_ulps_eq!(b.0, 1002_f32);
        let c: Nanoseconds<f32> = 1_f32.nanoseconds() + b;
        assert_ulps_eq!(c.0, 1002_000_001_f32);
        let d: Milliseconds<f32> = 1_f32.milliseconds() + c;
        assert_ulps_eq!(d.0, 1003.000_001_f32);
        let e: Seconds<f32> = 1_f32.seconds() + d.to_nanoseconds();
        assert_ulps_eq!(e.0, 2.003_000_001_f32);
        let f: Nanoseconds<f32> = 1_f32.nanoseconds() + e;
        assert_ulps_eq!(f.0, 2003_000_002_f32);
    }

    #[test]
    fn from_stddur() {
        let dur = std::time::Duration::from_micros(1);
        let a: Nanoseconds<f64> = dur.into();
        let b: Milliseconds<f64> = dur.into();
        let c: Seconds<f64> = dur.into();

        assert_eq!(1000_f64, a.0);
        assert_eq!(0.001_f64, b.0);
        assert_eq!(0.00_0001_f64, c.0);

        assert_eq!(Some(dur), a.into());
        assert_eq!(Some(dur), b.into());
        assert_eq!(Some(dur), c.into());
    }

    #[test]
    fn minus_dur() {
        let a: Seconds<f32> = (-1.2).into();
        let b: Option<std::time::Duration> = a.into();
        assert_eq!(b, None);
    }
}
