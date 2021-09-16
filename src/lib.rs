pub mod hardware;
mod measure_units;
pub mod model;
pub mod util;
pub mod walk;

pub fn check() {
    walk::pos();
}
