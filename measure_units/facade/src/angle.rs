use super::FloatStatus;
use measure_units_derive::*;

use derive_more::{From, Into};
use num_derive::*;

#[derive(
    Debug, From, Into, Clone, Copy, PartialEq, PartialOrd, ToPrimitive, FromPrimitive, FloatStatus,
)]
pub struct Radian(f64);

#[cfg(test)]
mod tests {
    use super::*;
    use core::cmp::Ordering;

    #[test]
    fn cmp_radian() {
        let a: Radian = 0.01.into();
        let b: Radian = 1.02.into();
        let z: Radian = 0.0.into();
        let n: Radian = f64::NAN.into();
        let i_posi: Radian = f64::INFINITY.into();
        let i_nega: Radian = f64::NEG_INFINITY.into();

        assert_eq!(a.is_nan(), false);
        assert_eq!(b.is_nan(), false);
        assert_eq!(z.is_nan(), false);

        // assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
        // assert_eq!(a < b, true);
        // assert_eq!(a > b, false);
        // assert_eq!(z < a, true);
        // assert_eq!(z > a, false);

        assert_eq!(n.is_nan(), true);
        // assert_eq!(z.partial_cmp(&n), None);
        // assert_eq!(n.partial_cmp(&z), None);
        // assert_eq!(n == z || n < z || n > z || z < n || z > n, false);
        // assert_eq!(n == i_posi || n < i_posi || n > i_posi, false);
        // assert_eq!(n == i_nega || n < i_nega || n > i_nega, false);

        assert_eq!(i_posi.is_infinite(), true);
        assert_eq!(i_posi.is_sign_positive(), true);
        assert_eq!(i_posi.is_sign_negative(), false);

        assert_eq!(i_nega.is_infinite(), true);
        assert_eq!(i_nega.is_sign_positive(), false);
        assert_eq!(i_nega.is_sign_negative(), true);

        // assert_eq!(i_posi == i_nega, false);
        // assert_eq!(i_posi < i_nega, false);
        // assert_eq!(i_posi > i_nega, true);
        // assert_eq!(i_nega > i_posi, false);
        // assert_eq!(i_nega < i_posi, true);
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
