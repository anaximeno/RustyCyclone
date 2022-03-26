#!allow(dead_code)

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::precision::Real;
        use super::core::Vector3;

        let mut vec = Vector3::from(2., 1., -3.);

        vec.invert();

        let magnitude: Real = 14_f32.sqrt();

        assert_eq!(magnitude, vec.magnitude());
    }
}

pub mod precision {
    pub type Real = f32;
}

pub mod core {
    use super::precision::*;

    #[derive(Debug)]
    pub struct Vector3 {
        pub x: Real,
        pub y: Real,
        pub z: Real,
        pad: Real
    }

    impl Vector3 {
        pub fn new() -> Self {
            Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
                pad: 0.
            }
        }

        pub fn from(x: Real, y: Real, z: Real) -> Self {
            Vector3 {x, y, z, pad: 0.}
        }

        pub fn invert(&mut self) {
            self.x = -self.x;
            self.y = -self.y;
            self.z = -self.z;
        }

        pub fn magnitude(&self) -> Real {
            (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
        }

        pub fn square_magnitude(&self) -> Real {
            self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
        } 

        pub fn multiply_inplace(&mut self, value: Real) {
            self.x *= value;
            self.y *= value;
            self.z *= value;
        }

        pub fn normalize(&mut self) {
            let m = self.magnitude();

            if m > 0_f32 {
                self.multiply_inplace(1_f32 / m);
            }
        }
    }
}
