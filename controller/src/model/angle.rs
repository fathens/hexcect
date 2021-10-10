use measure_units::*;
use num_traits::{Float, FloatConst, NumCast};

pub trait Angle<F: Float>: From<F> + Into<F> {
    fn modulo() -> F;

    fn normalize(self) -> Self {
        let modulo: F = Self::modulo();
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

impl<V> Angle<V> for Radians<V>
where
    V: Float,
    V: FloatConst,
    V: From<Radians<V>>,
{
    fn modulo() -> V {
        V::PI()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, CalcMix, Convertible, FloatStatus)]
#[calcmix(into = [f32, f64], unit_name = "°".to_string())]
#[convertible(Radians = v.to_radians())]
pub struct Degrees<V: Float>(V);

impl<V> Angle<V> for Degrees<V>
where
    V: Float,
    V: From<Degrees<V>>,
{
    fn modulo() -> V {
        <V as NumCast>::from(180).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_ulps_eq;
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
        let check = |a: f64, b: f64| {
            assert_ulps_eq!(Degrees::from(a).normalize().0, b, max_ulps = 8);
            assert_ulps_eq!(
                Radians::from(a.to_radians()).normalize().0,
                b.to_radians(),
                max_ulps = 8
            );
        };

        check(180.0, -180.0);
        check(-180.0, -180.0);
        check(540.0, -180.0);

        check(360.0, 0.0);
        check(720.0, 0.0);

        check(179.0, 179.0);
        check(-179.0, -179.0);

        check(340.0, -20.0);
        check(10.0, 10.0);
        check(400.0, 40.0);
        check(-400.0, -40.0);
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
