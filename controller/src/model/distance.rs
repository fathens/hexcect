use measure_units::*;

#[derive(
    Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, CalcMix, Convertible, FloatStatus, Approx,
)]
#[calcmix(unit_name = "m".to_string())]
#[convertible(Millimeters ^ 3)]
pub struct Meters<V>(V);

#[derive(
    Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, CalcMix, Convertible, FloatStatus, Approx,
)]
#[calcmix(unit_name = "mm".to_string())]
#[convertible(Meters ^ -3)]
pub struct Millimeters<V>(V);

pub trait MkDistance<V> {
    fn meters(self) -> Meters<V>;
    fn millimeters(self) -> Millimeters<V>;
}

impl MkDistance<f32> for f32 {
    fn meters(self) -> Meters<f32> {
        self.into()
    }

    fn millimeters(self) -> Millimeters<f32> {
        self.into()
    }
}

impl MkDistance<f64> for f64 {
    fn meters(self) -> Meters<f64> {
        self.into()
    }

    fn millimeters(self) -> Millimeters<f64> {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use approx::{assert_relative_eq, assert_ulps_eq};

    use super::*;

    #[test]
    fn conversions() {
        let a: Meters<f64> = 1.0_f64.meters();
        let b: Millimeters<f64> = a.into();
        assert_eq!(b.0, 1000.0_f64);

        let a: Millimeters<f32> = 1.0_f32.millimeters();
        let b: Meters<f32> = a.into();
        assert_eq!(b.0, 0.001_f32);
    }

    #[test]
    fn additions() {
        let a = 1_f64.meters() + 1_f64.millimeters();
        assert_eq!(a.to_string(), "1.001m");

        let a = 1_f32.millimeters() + 1_f32.meters();
        assert_eq!(a.to_string(), "1001mm");
    }

    #[test]
    fn nearly_equals() {
        let a = 0.00_000_000_000_001_f64;
        let b = a * 1.00_000_0001;

        assert_ne!(a, b);
        assert_ne!(a.meters(), b.meters());
        assert_ne!(a.millimeters(), b.millimeters());

        assert_ulps_eq!(a, b);
        assert_ulps_eq!(a.meters(), b.meters());
        assert_ulps_eq!(a.millimeters(), b.millimeters());

        assert_relative_eq!(a, b);
        assert_relative_eq!(a.meters(), b.meters());
        assert_relative_eq!(a.millimeters(), b.millimeters());
    }
}
