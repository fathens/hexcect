mod angle;
mod dimensional;
mod distance;
mod time;

pub use angle::*;
pub use dimensional::*;
pub use distance::*;
pub use time::*;

use measure_units::*;

pub type Speed<V> = UnitsDiv<V, Meters<V>, Seconds<V>>;

pub type Accel<V> = UnitsDiv<V, Speed<V>, Seconds<V>>;

pub type AngleVelocity<V> = UnitsDiv<V, Degrees<V>, Seconds<V>>;
