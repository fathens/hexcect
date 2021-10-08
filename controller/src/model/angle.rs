use measure_units::*;
use num_traits::Float;

pub trait Angle<F: Float>: From<F> + Into<F> {
    const MODULO: F;

    fn normalize(self) -> Self {
        let modulo: F = Self::MODULO;
        let round = modulo + modulo;

        let value: F = self.into();
        let mut r = value % round;

        if r.abs() > modulo {
            r = r + if r.is_sign_positive() { -round } else { round };
        }
        if (r - modulo).abs() < F::epsilon() {
            r = -modulo;
        }

        r.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, CalcMix, Convertible, FloatStatus)]
#[calcmix(into = [f32, f64], unit_name = "rad".to_string())]
#[convertible(Degrees = v.to_degrees())]
pub struct Radians<V: Float>(V);

impl<V: Float> Radians<V> {
    pub fn sin(&self) -> V {
        self.0.sin()
    }

    pub fn cos(&self) -> V {
        self.0.cos()
    }
}

impl Angle<f32> for Radians<f32> {
    const MODULO: f32 = core::f32::consts::PI;
}

impl Angle<f64> for Radians<f64> {
    const MODULO: f64 = core::f64::consts::PI;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, CalcMix, Convertible, FloatStatus)]
#[calcmix(into = [f32, f64], unit_name = "°".to_string())]
#[convertible(Radians = v.to_radians())]
pub struct Degrees<V: Float>(V);

impl Angle<f32> for Degrees<f32> {
    const MODULO: f32 = 180.0;
}

impl Angle<f64> for Degrees<f64> {
    const MODULO: f64 = 180.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;

    #[test]
    fn convert() {
        let a = Degrees::from(90.0);
        let b: Radians<f64> = a.into();
        let c: Degrees<f64> = b.into();
        assert_eq!(b.0, a.0.to_radians());
        assert_eq!(c.0, b.0.to_degrees());
    }

    #[test]
    fn normalize() {
        assert_eq!(Degrees::from(180.0).normalize().0, -180.0);
        assert_eq!(Degrees::from(-180.0).normalize().0, -180.0);
        assert_eq!(Degrees::from(540.0).normalize().0, -180.0);

        assert_eq!(Degrees::from(360.0).normalize().0, 0.0);
        assert_eq!(Degrees::from(720.0).normalize().0, 0.0);

        assert_eq!(Degrees::from(179.0).normalize().0, 179.0);
        assert_eq!(Degrees::from(-179.0).normalize().0, -179.0);

        assert_eq!(Degrees::from(340.0).normalize().0, -20.0);
        assert_eq!(Degrees::from(10.0).normalize().0, 10.0);
        assert_eq!(Degrees::from(400.0).normalize().0, 40.0);
        assert_eq!(Degrees::from(-400.0).normalize().0, -40.0);
    }

    #[test]
    fn sin_cos() {
        let mut rnd = rand::thread_rng();
        for _ in 0..100 {
            let v: f64 = rnd.gen();
            let r: Radians<f64> = v.into();
            assert_eq!(v.sin(), r.sin());
            assert_eq!(v.cos(), r.cos());
        }
    }

    #[test]
    fn additions() {
        let a: Degrees<f64> = 1_f64.into();
        let b: Radians<f64> = a.into();
        assert_eq!((a + b).to_string(), "2°");

        let a: Radians<f64> = 1_f64.into();
        let b: Degrees<f64> = a.into();
        assert_eq!((a + b).to_string(), "2rad");
    }
}
