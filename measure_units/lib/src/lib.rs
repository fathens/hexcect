pub mod angle;

pub trait FloatStatus {
    fn is_nan(&self) -> bool;
}
