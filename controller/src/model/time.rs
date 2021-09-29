use measure_units::*;

#[derive(Debug, Clone, Copy, PartialEq, CalcMix)]
#[calcmix(unit_name = "s".to_string())]
pub struct Seconds<V>(V);
