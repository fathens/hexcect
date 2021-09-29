use measure_units::*;

#[derive(Debug, Clone, Copy, PartialEq, CalcMix)]
#[calcmix(unit_name = "m".to_string())]
pub struct Meters<V>(V);
