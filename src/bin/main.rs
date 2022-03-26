use rusty_cyclone::core::Vector3;

fn main() {
    let mut v = Vector3::from(3.1, -2.3, 0.7);
    v.normalize();
    v.invert();
    v.inplace_mult(2.0);
    println!("{:#?}", v);
}
