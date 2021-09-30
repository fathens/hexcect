use super::*;

#[derive(Clone, Copy, CalcMix)]
#[calcmix(unit_name = "m".to_string())]
pub struct Meter(f64);

#[derive(Clone, Copy, CalcMix)]
#[calcmix(unit_name = "s".to_string())]
pub struct Second(f64);

#[derive(Clone, Copy, CalcMix)]
#[calcmix(unit_name = "d".to_string())]
pub struct Degree(f64);

#[test]
fn simple_add() {
    let a = UnitsDiv::<f64, Meter, Second>::from(1.2);
    let b = UnitsDiv::<f64, Meter, Second>::from(3.4);
    let c = a + b;
    assert_eq!(c.0, 4.6);
    assert_eq!(c.to_string(), "4.6m/s");
}

#[test]
fn simple_sub() {
    let a = UnitsDiv::<f64, Meter, Second>::from(1.2);
    let b = UnitsDiv::<f64, Meter, Second>::from(3.5);
    let c = a - b;
    assert_eq!(c.0, -2.3);
    assert_eq!(c.to_string(), "-2.3m/s");
}

#[test]
fn commutative() {
    let d = Meter::from(10.0);
    let t = Second::from(2.0);
    let g = d * t;
    assert_eq!(g.0, 20.0);
    assert_eq!(g.to_string(), "20ms");

    let r = g.commutative();
    assert_eq!(r.0, 20.0);
    assert_eq!(r.to_string(), "20sm");
}

#[test]
fn associative() {
    let d = Meter::from(10.0);
    let t = Second::from(2.0);
    let o = Meter::from(3.0);
    let g: UnitsMul<f64, UnitsMul<f64, Meter, Second>, Meter> = d * t * o;
    assert_eq!(g.0, 60.0);
    assert_eq!(g.to_string(), "60msm");

    let r: UnitsMul<f64, Meter, UnitsMul<f64, Second, Meter>> = g.associative();
    assert_eq!(r.0, 60.0);
    assert_eq!(r.to_string(), "60msm");
}

#[test]
fn reduction_mul() {
    let distance = Meter::from(10.0);
    let time = Second::from(2.0);
    let takes = Second::from(3.0);

    let speed = distance / time;
    let goal = speed * takes;
    assert_eq!(goal.0, 15.0);
    assert_eq!(goal.to_string(), "15m/ss");

    let s = goal.reduction();
    assert_eq!(s.0, 15.0);
    assert_eq!(s.to_string(), "15m");
}

#[test]
fn reduction_div() {
    let a = Second::from(3.0);
    let b = Second::from(1.5);

    let c = a / b;
    assert_eq!(c.0, 2.0);
    assert_eq!(c.to_string(), "2s/s");

    let s = c.reduction();
    assert_eq!(s.0, 2.0);
    assert_eq!(s.to_string(), "2");
}

#[test]
fn reduction_right() {
    let distance = Meter::from(10.0);
    let time = Second::from(2.0);
    let takes = Second::from(3.0);

    let a = distance * takes;
    let b = a / time;
    assert_eq!(b.0, 15.0);
    assert_eq!(b.to_string(), "15ms/s");

    let s = b.reduction_right();
    assert_eq!(s.0, 15.0);
    assert_eq!(s.to_string(), "15m");
}

#[test]
fn reduction_left() {
    let distance = Meter::from(10.0);
    let time = Second::from(2.0);
    let takes = Second::from(3.0);

    let a = takes * distance;
    let b = a / time;
    assert_eq!(b.0, 15.0);
    assert_eq!(b.to_string(), "15sm/s");

    let s = b.reduction_left();
    assert_eq!(s.0, 15.0);
    assert_eq!(s.to_string(), "15m");
}

#[test]
fn infuse_extract_nmr() {
    type Extracted = UnitsMul<f64, Meter, UnitsDiv<f64, Second, Degree>>;
    type Infused = UnitsDiv<f64, UnitsMul<f64, Meter, Second>, Degree>;

    let a = Meter::from(2.0);
    let b = Second::from(3.0);
    let c = Degree::from(5.0);

    let x: Extracted = a * (b / c);
    let y: Infused = x.infuse_nmr();
    let z: Extracted = y.extract_nmr();

    assert_eq!(x.to_string(), "1.2ms/d");
    assert_eq!(y.to_string(), "1.2ms/d");
    assert_eq!(z.to_string(), "1.2ms/d");
}

#[test]
fn infuse_extract_dnm() {
    type Extracted = UnitsDiv<f64, UnitsDiv<f64, Meter, Second>, Degree>;
    type Infused = UnitsDiv<f64, Meter, UnitsMul<f64, Second, Degree>>;

    let a = Meter::from(3.0);
    let b = Second::from(2.0);
    let c = Degree::from(5.0);

    let x: Extracted = a / b / c;
    let y: Infused = x.infuse_dnm();
    let z: Extracted = y.extract_dnm();

    assert_eq!(x.to_string(), "0.3m/s/d");
    assert_eq!(y.to_string(), "0.3m/sd");
    assert_eq!(z.to_string(), "0.3m/s/d");
}

#[test]
fn mul_inner_right_reduction_left() {
    let distance = Meter::from(10.0);
    let time = Second::from(2.0);
    let takes = Second::from(3.0);

    let a = takes * distance;
    let b = a / time;
    let c = Second::from(5.0) * b;
    assert_eq!(c.0, 75.0);
    assert_eq!(c.to_string(), "75ssm/s");

    let s = c.inner_right(|a| a.reduction_left());
    assert_eq!(s.0, 75.0);
    assert_eq!(s.to_string(), "75sm");
}

#[test]
fn mul_inner_left_reduction_left() {
    let distance = Meter::from(10.0);
    let time = Second::from(2.0);
    let takes = Second::from(3.0);

    let a = takes * distance;
    let b = a / time;
    let c = b * Second::from(5.0);
    assert_eq!(c.0, 75.0);
    assert_eq!(c.to_string(), "75sm/ss");

    let s = c.inner_left(|a| a.reduction_left());
    assert_eq!(s.0, 75.0);
    assert_eq!(s.to_string(), "75ms");
}

#[test]
fn div_inner_right_reduction_left() {
    let distance = Meter::from(10.0);
    let time = Second::from(2.0);
    let takes = Second::from(3.0);

    let a = takes * distance;
    let b = a / time;
    let c = Second::from(30.0) / b;
    assert_eq!(c.0, 2.0);
    assert_eq!(c.to_string(), "2s/sm/s");

    let s = c.inner_right(|a| a.reduction_left());
    assert_eq!(s.0, 2.0);
    assert_eq!(s.to_string(), "2s/m");
}

#[test]
fn div_inner_left_reduction_left() {
    let distance = Meter::from(10.0);
    let time = Second::from(2.0);
    let takes = Second::from(3.0);

    let a = takes * distance;
    let b = a / time;
    let c = b / Second::from(5.0);
    assert_eq!(c.0, 3.0);
    assert_eq!(c.to_string(), "3sm/s/s");

    let s = c.inner_left(|a| a.reduction_left());
    assert_eq!(s.0, 3.0);
    assert_eq!(s.to_string(), "3m/s");
}
