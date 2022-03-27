#[cfg(test)]
mod tests {
    use super::core::*;
    use super::precision::*;

    #[test]
    fn vec3_magnitude() {
        let mut vec = Vector3::new(2., 1., -3.);
        vec.invert();
        assert_eq!((14.0 as Real).sqrt(), vec.magnitude());
    }

    #[test]
    fn vec3_inplace_mult() {
        let mut vec = Vector3::new(1., 2., 1.);
        vec.inplace_mult(2.0);
        assert_eq!(4.0 as Real, vec.y);
    }

    #[test]
    fn random_vector3() {
        let vec = Vector3::from_random_range(4., 8.);
        assert!(vec.magnitude() > 3.99);
    }

    #[test]
    fn random_max_and_min_vector3() {
        let v1 = Vector3::from_random_range(2., 4.);
        let v2 = Vector3::from_random_range(3., 6.);

        let v3 = Vector3::from_random_range_vector(&v1, &v2);
        assert!(v3.magnitude() > 2.);
    }
}

pub mod precision {
    /// Defines the real number precision.
    pub type Real = f32;
}

pub mod core {
    use super::precision::*;
    use rand::Rng;

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
        pub fn from_origin() -> Self {
            Self::new(0., 0., 0.)
        }

        /// Creates a new Vector3, defining the the values for all axes
        /// 
        /// ### Arguments
        /// * `x` - value for the x axis
        /// * `y` - value for the y axis
        /// * `z` - value for the z axis
        /// 
        /// ### Examples
        /// ```
        /// use rusty_cyclone::core::Vector3;
        /// let vec = Vector3::new(1.2, 3.0, 1.0);
        /// ```
        pub fn new(x: Real, y: Real, z: Real) -> Self {
            Self {x, y, z, pad: 0.}
        }

        /// Creates a new vector as a copy of the given vector.
        pub fn copy(vector: &Vector3) -> Self {
            Self::new(vector.x, vector.y, vector.z)
        }

        /// Generates a vector with the value of the elements within a given range.
        /// 
        /// ### Arguments
        /// * `min` - The minimum value of the range
        /// * `max` - The maximum value of the range
        pub fn from_random_range(min: Real, max: Real) -> Self {
            assert!(min < max); // TODO: Use Rust error treatment
            
            let mut rng = rand::thread_rng();

            Self::new(
                rng.gen_range(min..max),
                rng.gen_range(min..max),
                rng.gen_range(min..max)
            )
        }

        /// Generates a vector with random values between the elements of the min and max vectors.
        /// 
        /// ### Arguments
        /// * `min_vector` - Vector with the minimum value for each dimension
        /// * `max_Vector` - Vector with the maximum value for each dimension
        pub fn from_random_range_vector(min_vector: &Vector3, max_vector: &Vector3) -> Self {
            let mut rng = rand::thread_rng();

            Self::new(
                rng.gen_range(min_vector.x..max_vector.x),
                rng.gen_range(min_vector.y..max_vector.y),
                rng.gen_range(min_vector.z..max_vector.z),
            )
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
            Vector3::new(
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
            Vector3::new(
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
            Vector3::new(
                self.x - v.x,
                self.y - v.y,
                self.z - v.z
            )
        }

        /// Adds elements of the vector to the scaled correspondend element of 
        /// another vector, given as arguments.
        /// 
        /// ### Arguments
        /// * `v` - The vector to be scaled and added
        /// * `scale` - The scale factor
        pub fn inplace_add_scaled_vector(&mut self, v: &Vector3, scale: Real) {
            self.x += v.x * scale;
            self.y += v.y * scale;
            self.z += v.z * scale;
        }

        /// Returns the element-wise multiplycation of this vector 
        /// and the argument vector.
        pub fn component_product(&self, v: &Vector3) -> Vector3 {
            Vector3::new(
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
            Vector3::new(
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

pub mod particle {
    use super::precision::*;
    use super::core::*;

    /// A particle is the simplest object that can be simulated in
    /// the physics system.
    #[allow(unused)]
    #[derive(Debug)]
    pub struct Particle {
        /// Holds the linear position of the particle in world space
        position: Vector3,
        /// Holds the linear velocity of the particle in world space
        velocity: Vector3,
        /// Holds the acceleration of the particle.
        acceleration: Vector3,
        /// Holds the amount of damping applied to linear motion.
        damping: Real,
        /// Holds the inverse of the mass of the particle.
        inverse_mass: Real,
    }

    pub trait ParticleLike {                                                                                                                                                  
        fn integrate(&mut self, duration: Real);
        fn mass(&self) -> Real;
        fn position(&self) -> &Vector3;
        fn velocity(&self) -> &Vector3;
        fn acceleration(&self) -> &Vector3;
        fn set_mass(&mut self, mass: Real);
        fn set_position(&mut self, position: Vector3);
        fn set_velocity(&mut self, velocity: Vector3);
        fn set_acceleration(&mut self, acceleration: Vector3);
    }

    impl ParticleLike for Particle {
        /// Integrates the particle forward in time by the given amount.
        fn integrate(&mut self, duration: Real) {
            assert!(duration > 0.0);
            
            // Update linear position
            self.position.inplace_add_scaled_vector(&self.velocity, duration);

            // Work out the acceleration from the force
            let mut resulting_acc = Vector3::copy(&self.acceleration);
            resulting_acc.inplace_add_scaled_vector(&self.acceleration, duration);

            // Update linear velocity from the acceleration
            self.velocity.inplace_add_scaled_vector(&resulting_acc, duration);

            // Impose drag.
            self.velocity.inplace_mult(self.damping.powf(duration));
        }

        fn position(&self) -> &Vector3 {
            &self.position
        }

        fn velocity(&self) -> &Vector3 {
            &self.velocity
        }

        fn acceleration(&self) -> &Vector3 {
            &self.acceleration
        }

        fn mass(&self) -> Real {
            1.0 / self.inverse_mass
        }

        fn set_mass(&mut self, mass: Real) {
            self.inverse_mass = 1.0 / mass;
        }

        fn set_position(&mut self, position: Vector3) {
            self.position = position;
        }

        fn set_velocity(&mut self, velocity: Vector3) {
            self.velocity = velocity;
        }

        fn set_acceleration(&mut self, acceleration: Vector3) {
            self.acceleration = acceleration;
        }
    }
    
    impl Particle {
        /// Creates a new particle at the give position
        pub fn from_position(position: Vector3, mass: Real, velocity: Vector3, acceleration: Vector3, damping: Real) -> Self {
            let inverse_mass: Real = 1.0 / mass;
            Particle { position, velocity, acceleration, damping, inverse_mass }
        }

        /// Creates a new particles and set the position automatically to the origin.
        pub fn new(mass: Real, velocity: Vector3, acceleration: Vector3, damping: Real) -> Self {
            let position = Vector3::from_origin();
            Particle::from_position(position, mass, velocity, acceleration, damping)            
        }
    } 
}
