use rusty_cyclone::core::Vector3;
use rusty_cyclone::precision::Real;


fn main() {
    let mut v = Vector3::from(3.1, -2.3, 0.7);

    v.normalize();
    v.invert();
    v.inplace_mult(2.0);

    let j = Vector3::from(1.3, 0.5, 1.2);

    let k: Real = v.dot_product(&j);

    let h: Vector3 = v.vector_product(&j);

    println!("Dot Prod is = {:#?}\n", k);
    println!("Vect Prod is = {:#?}", h);
}
