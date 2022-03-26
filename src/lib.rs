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
    /// Defines the real number precision.
    pub type Real = f32;
}

pub mod core {
    use super::precision::*;

    #[derive(Debug)]
    #[allow(unused)]
    /// Tri-dimensional vector composed of three elements
    /// for the x, y and z axes.
    pub struct Vector3 {
        pub x: Real,
        pub y: Real,
        pub z: Real,
        pad: Real
    }


    impl Vector3 {
        /// Returns a new Vector with all elements set to zero.
        pub fn new() -> Self {
            Vector3::from(0., 0., 0.)
        }

        /// Creates a new Vector3, defining the the values for all axes
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

        /// Invert all the elements of the vector
        pub fn invert(&mut self) {
            self.x = -self.x;
            self.y = -self.y;
            self.z = -self.z;
        }

        /// Returns the magnitude of the vector
        pub fn magnitude(&self) -> Real {
            (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
        }

        /// Returns the square of the magnitude of the vector
        pub fn square_magnitude(&self) -> Real {
            self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
        }

        /// Normalize the elements of the vector
        pub fn normalize(&mut self) {
            let m = self.magnitude();
            
            if m > 0.0 {
                self.inplace_mult(1.0 / m);
            }
        }

        /// Multiply the elements of the vector to the given value
        pub fn inplace_mult(&mut self, value: Real) {
            self.x *= value;
            self.y *= value;
            self.z *= value;
        }

        /// Returns a new vector with the result of the multiplication
        /// of the elements of the vector to some value
        pub fn mult(&self, value: Real) -> Vector3 {
            Vector3::from(
                self.x * value,
                self.y * value,
                self.z * value,
            )
        }

        /// Adds the elements of the vector to the correspondent
        /// elements of the vector given as argument.
        pub fn inplace_add(&mut self, v: &Vector3) {
            self.x += v.x;
            self.y += v.y;
            self.z += v.z;
        }

        /// Returns a new vector with the result of the addition
        /// of the elements of the vector to the elements of the 
        /// vector given as argument.
        pub fn add(&self, v: &Vector3) -> Vector3 {
            Vector3::from(
                self.x + v.x,
                self.y + v.y,
                self.z + v.z,
            )
        }

        /// Subtracts the elements of the vector to the correspondent
        /// elements of the vector given as argument.
        pub fn inplace_sub(&mut self, v: &Vector3) {
            self.x -= v.x;
            self.y -= v.y;
            self.z -= v.z;
        }

        /// Returns a new vector with the result of the subtraction
        /// of the elements of the vector to the elements of the 
        /// vector given as argument.
        pub fn sub(&self, v: &Vector3) -> Vector3 {
            Vector3::from(
                self.x - v.x,
                self.y - v.y,
                self.z - v.z
            )
        }

        /// Adds elements of the vector to the scaled correspondend element of 
        /// another vector, given as arguments.
        /// 
        /// # Arguments
        /// * `v` - The vector to be scaled and added
        /// * `scale`- The scale factor
        pub fn inplace_add_scaled_vector(&mut self, v: &Vector3, scale: Real) {
            self.x += v.x * scale;
            self.y += v.y * scale;
            self.z += v.z * scale;
        }

        /// Returns the element-wise multiplycation of this vector 
        /// and the argument vector.
        pub fn component_product(&self, v: &Vector3) -> Vector3 {
            Vector3::from(
                self.x * v.x,
                self.y * v.y,
                self.z * v.z,
            )
        }

        /// Element-wise operation between this vector and the 
        /// given argument vector, the result is then replaced
        /// to the repective elements of this vector.
        pub fn inplace_component_product(&mut self, v: &Vector3) {
            self.x *= v.x;
            self.y *= v.y;
            self.z *= v.z;
        }

        /// Returns the scalar (dot) product between this vector
        /// and the one give as argument.
        pub fn dot_product(&self, v: &Vector3) -> Real {
            self.x * v.x + self.y * v.y + self.z * v.z
        }

        /// Returns the vectorial product between this vector
        /// and the given one.
        pub fn vector_product(&self, v: &Vector3) -> Vector3 {
            Vector3::from(
                self.y * v.z - self.z * v.y,
                self.z * v.x - self.x * v.z,
                self.x * v.y - self.y * v.x
            )
        }

        /// Calculates the vectorial product between this vector
        /// and the one given as argument, and then replaces the
        /// elements of this vector to the result of the product.
        pub fn inplace_vector_product(&mut self, v: &Vector3) {
            *self = self.vector_product(v);
        }
    }
}
