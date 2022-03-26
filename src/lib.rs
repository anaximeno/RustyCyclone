#[cfg(test)]
mod tests {
    use super::core::*;
    use super::precision::*;

    #[test]
    fn vec3_magnitude() {
        let mut vec = Vector3::from(2., 1., -3.);
        vec.invert();
        assert_eq!((14.0 as Real).sqrt(), vec.magnitude());
    }

    #[test]
    fn vec3_inplace_mult() {
        let mut vec = Vector3::from(1., 2., 1.);
        vec.inplace_mult(2.0);
        assert_eq!(4.0 as Real, vec.y);
    }
}

pub mod precision {
    /// Defines a real number precision.
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
        /// Returns a new Vector with all elements set to zero.
        pub fn new() -> Self {
            Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
                pad: 0.
            }
        }

        /// Returns a Vector with the elements as the arguments
        /// 
        /// # Arguments
        /// * `x` - value for the x axes
        /// * `y` - value for the y axes
        /// * `z` - value for the z axes
        /// 
        /// # Examples
        /// ```
        /// use rusty_cyclone::core::Vector3;
        /// let vec = Vector3::from(1.2, 3.0, 1.0);
        /// ```
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

        pub fn normalize(&mut self) {
            let m = self.magnitude();
            
            if m > 0.0 {
                self.inplace_mult(1.0 / m);
            }
        }

        pub fn inplace_mult(&mut self, value: Real) {
            self.x *= value;
            self.y *= value;
            self.z *= value;
        }

        pub fn mult(self, value: Real) -> Vector3 {
            Vector3::from(
                self.x * value,
                self.y * value,
                self.z * value,
            )
        }

        pub fn inplace_add(&mut self, v: Vector3) {
            self.x += v.x;
            self.y += v.y;
            self.z += v.z;
        }

        pub fn add(&mut self, v: Vector3) -> Vector3 {
            Vector3::from(
                self.x + v.x,
                self.y + v.y,
                self.z + v.z,
            )
        }

        pub fn inplace_sub(&mut self, v: Vector3) {
            self.x -= v.x;
            self.y -= v.y;
            self.z -= v.z;
        }

        pub fn sub(&mut self, v: Vector3) -> Vector3 {
            Vector3::from(
                self.x - v.x,
                self.y - v.y,
                self.z - v.z
            )
        }

        pub fn inplace_add_scaled_vector(&mut self, v: Vector3, scale: Real) {
            self.x += v.x * scale;
            self.y += v.y * scale;
            self.z += v.z * scale;
        }
    }
}
