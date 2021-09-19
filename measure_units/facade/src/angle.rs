use crate::{Convertible, FloatStatus};
use measure_units_derive::*;

use derive_more::{From, Into};
use num_derive::*;

pub trait Angle<F>: From<F> + Into<F>
where
    F: From<f64>,
    f64: From<F>,
{
    const MODULO: F;

    fn normalize(self) -> Self {
        let modulo: f64 = Self::MODULO.into();

        let self_value: F = self.into();
        let value: f64 = self_value.into();

        let round = modulo * 2.0;
        let v = value % round;
        let mut r = if v.abs() < modulo {
            v
        } else {
            let adding = if v.is_sign_positive() { -round } else { round };
            v + adding
        };
        if (r - modulo).abs() < f64::EPSILON {
            r = -modulo;
        }

        let result: F = r.into();
        result.into()
    }
}

#[derive(
    Debug,
    From,
    Into,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    ToPrimitive,
    FromPrimitive,
    FloatStatus,
    Convertible,
)]
#[convertible(Degree = Degree::MODULO / Radian::MODULO)]
pub struct Radian(f64);

impl Angle<f64> for Radian {
    const MODULO: f64 = core::f64::consts::PI;
}

#[derive(
    Debug,
    From,
    Into,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    ToPrimitive,
    FromPrimitive,
    FloatStatus,
    Convertible,
)]
#[convertible(Radian = Radian::MODULO / Degree::MODULO)]
pub struct Degree(f64);

impl Angle<f64> for Degree {
    const MODULO: f64 = 180.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cmp::Ordering;

    #[test]
    fn convert() {
        let a = Degree::from(90.0);
        let b: Radian = a.convert();
        let c: Degree = b.convert();
        assert_eq!(b.0, a.0.to_radians());
        assert_eq!(c.0, b.0.to_degrees());
    }

    #[test]
    fn normalize() {
        assert_eq!(Degree::from(180.0).normalize().0, -180.0);
        assert_eq!(Degree::from(-180.0).normalize().0, -180.0);
        assert_eq!(Degree::from(540.0).normalize().0, -180.0);

        assert_eq!(Degree::from(360.0).normalize().0, 0.0);
        assert_eq!(Degree::from(720.0).normalize().0, 0.0);

        assert_eq!(Degree::from(179.0).normalize().0, 179.0);
        assert_eq!(Degree::from(-179.0).normalize().0, -179.0);

        assert_eq!(Degree::from(340.0).normalize().0, -20.0);
        assert_eq!(Degree::from(10.0).normalize().0, 10.0);
        assert_eq!(Degree::from(400.0).normalize().0, 40.0);
        assert_eq!(Degree::from(-400.0).normalize().0, -40.0);
    }

    #[test]
    fn check_float_specs_of_abnormals() {
        let v: f64 = 1.23;
        let z: f64 = 0.0;
        assert_eq!(v.is_normal(), true);
        assert_eq!(z.is_normal(), false);

        assert_eq!((v / v).is_normal(), true);
        assert_eq!((v / v).is_finite(), true);
        assert_eq!((v / v).is_infinite(), false);
        assert_eq!((v / v).is_nan(), false);

        assert_eq!((v / z).is_normal(), false);
        assert_eq!((v / z).is_finite(), false);
        assert_eq!((v / z).is_infinite(), true);
        assert_eq!((v / z).is_nan(), false);
        assert_eq!((v / z), f64::INFINITY);
        assert_eq!((-v / z), f64::NEG_INFINITY);

        assert_eq!(
            f64::INFINITY.partial_cmp(&f64::INFINITY),
            Some(Ordering::Equal)
        );
        assert_eq!(
            f64::INFINITY.partial_cmp(&f64::NEG_INFINITY),
            Some(Ordering::Greater)
        );
        assert_eq!(
            f64::NEG_INFINITY.partial_cmp(&f64::INFINITY),
            Some(Ordering::Less)
        );
        assert_eq!(
            f64::NEG_INFINITY.partial_cmp(&f64::NEG_INFINITY),
            Some(Ordering::Equal)
        );

        assert_eq!((z / z).is_normal(), false);
        assert_eq!((z / z).is_finite(), false);
        assert_eq!((z / z).is_infinite(), false);
        assert_eq!((z / z).is_nan(), true);
        assert_eq!((z / z) == (z / z), false);
        assert_eq!((z / z) == f64::NAN, false);
        assert_eq!(f64::NAN == f64::NAN, false);

        let m: f64 = f64::MIN_POSITIVE;
        let h: f64 = m / 2.0;
        let h2: f64 = m / 2.0;
        let t: f64 = m / 3.0;

        assert_eq!(m.is_normal(), true);
        assert_eq!(m.is_subnormal(), false);
        assert_eq!(m.is_sign_positive(), true);

        assert_eq!(h.is_normal(), false);
        assert_eq!(h.is_subnormal(), true);
        assert_eq!(h.is_sign_positive(), true);

        assert_eq!(h.partial_cmp(&t), Some(Ordering::Greater));
        assert_eq!(t.partial_cmp(&h), Some(Ordering::Less));
        assert_eq!(h2.partial_cmp(&h), Some(Ordering::Equal));
    }
}
