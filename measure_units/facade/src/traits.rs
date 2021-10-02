pub trait FloatStatus {
    fn abs(&self) -> Self;
    fn is_nan(&self) -> bool;
    fn is_zero(&self) -> bool;
    fn is_normal(&self) -> bool;
    fn is_subnormal(&self) -> bool;
    fn is_finite(&self) -> bool;
    fn is_infinite(&self) -> bool;
    fn is_sign_positive(&self) -> bool;
    fn is_sign_negative(&self) -> bool;
}
