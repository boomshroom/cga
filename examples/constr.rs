use cga::d3::Point;
use cga::Outer;

#[cfg(not(feature = "unstable"))]
fn main() {
    let p1 : Point<f32> = Point::new([1.0, 2.0, 4.0]);
    let p2 : Point<f32> = Point::new([0.0, 3.0, 0.0]);

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    let l = p1.outer(p2);

    println!("l: {:?}", l);
}

#[cfg(feature = "unstable")]
fn main() {
    use cga::F32;
    let p1 : Point<F32> = Point::new([F32::new(1.0), F32::new(2.0), F32::new(4.0)]);
    let p2 : Point<F32> = Point::new([F32::new(0.0), F32::new(3.0), F32::new(0.0)]);

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    let l = p1.outer(p2);

    println!("l: {:?}", l);
}
