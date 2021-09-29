use measure_units::*;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, CalcMix, Convertible)]
#[calcmix(unit_name = "s".to_string())]
#[convertible(Milliseconds ^ 3)]
pub struct Seconds<V>(V);

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, CalcMix, Convertible)]
#[calcmix(unit_name = "ms".to_string())]
#[convertible(Seconds ^ -3)]
pub struct Milliseconds<V>(V);

pub trait Duration<V> {
    fn seconds(self) -> Seconds<V>;
    fn milliseconds(self) -> Milliseconds<V>;
}

impl Duration<f32> for f32 {
    fn seconds(self) -> Seconds<f32> {
        self.into()
    }

    fn milliseconds(self) -> Milliseconds<f32> {
        self.into()
    }
}

impl Duration<f64> for f64 {
    fn seconds(self) -> Seconds<f64> {
        self.into()
    }

    fn milliseconds(self) -> Milliseconds<f64> {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        let a: Seconds<f64> = 1.0_f64.seconds();
        let b: Milliseconds<f64> = a.into();
        assert_eq!(b.0, 1000.0_f64);

        let a: Milliseconds<f32> = 1.0_f32.milliseconds();
        let b: Seconds<f32> = a.into();
        assert_eq!(b.0, 0.001_f32);
    }
}
