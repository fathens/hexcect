use measure_units::*;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, CalcMix, Convertible)]
#[calcmix(unit_name = "s".to_string())]
#[convertible(Milliseconds ^ 3)]
#[convertible(Nanoseconds ^ 9)]
pub struct Seconds<V>(V);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, CalcMix, Convertible)]
#[calcmix(unit_name = "ms".to_string())]
#[convertible(Seconds ^ -3)]
#[convertible(Nanoseconds ^ 6)]
pub struct Milliseconds<V>(V);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, CalcMix, Convertible)]
#[calcmix(unit_name = "ms".to_string())]
#[convertible(Seconds ^ -9)]
#[convertible(Milliseconds ^ -6)]
pub struct Nanoseconds<V>(V);

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
    fn additions() {
        let a = 1_f64.seconds() + 1_f64.milliseconds();
        assert_eq!(a.to_string(), "1.001s");

        let a = 1_f32.milliseconds() + 1_f32.seconds();
        assert_eq!(a.to_string(), "1001ms");
    }
}
