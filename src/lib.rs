#[cfg(test)]
mod tests {
    use super::core::*;
    use super::precision::*;

    #[test]
    fn vec3_magnitude() {
        let mut vec = Vec3::new(2., 1., -3.);
        vec.invert();
        assert_eq!((14.0 as Real).sqrt(), vec.magnitude());
    }

    #[test]
    fn vec3_inplace_mult() {
        let mut vec = Vec3::new(1., 2., 1.);
        vec *= 2.0 as Real;
        assert_eq!(4.0 as Real, vec.y);
    }

    #[test]
    fn random_vec3() {
        let vec = Vec3::from_random_range(4., 8.);
        assert!(vec.magnitude() > 3.99);
    }

    #[test]
    fn random_max_and_min_vec3() {
        let v1 = Vec3::from_random_range(2., 4.);
        let v2 = Vec3::from_random_range(3., 6.);

        let v3 = Vec3::from_random_range_vector(&v1, &v2);
        assert!(v3.magnitude() > 2.);
    }
}

pub mod precision {
    /// Defines the real number precision.
    pub type Real = f32;
}

pub mod core {
    use std::ops::*;
    use super::precision::*;
    use rand::Rng;

    // TODO: implement generic vector3
    
    #[allow(unused)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    /// Tri-dimensional vector composed of three elements
    /// for the x, y and z axes.
    pub struct Vec3 {
        pub x: Real,
        pub y: Real,
        pub z: Real,
        pad: Real
    }

    impl Vec3 {
        /// Returns a new Vector with all elements set to zero.
        pub fn from_origin() -> Self {
            Self::new(0., 0., 0.)
        }

        /// Creates a new Vec3, defining the the values for all axes
        /// 
        /// ### Arguments
        /// * `x` - value for the x axis
        /// * `y` - value for the y axis
        /// * `z` - value for the z axis
        /// 
        /// ### Examples
        /// ```
        /// use rusty_cyclone::core::Vec3;
        /// let vec = Vec3::new(1.2, 3.0, 1.0);
        /// ```
        pub fn new(x: Real, y: Real, z: Real) -> Self {
            Self {x, y, z, pad: 0.}
        }

        /// Creates a new vector as a copy of the given vector.
        pub fn copy(vector: &Vec3) -> Self {
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
        pub fn from_random_range_vector(min_vector: &Vec3, max_vector: &Vec3) -> Self {
            let mut rng = rand::thread_rng();

            Self::new(
                rng.gen_range(min_vector.x..max_vector.x),
                rng.gen_range(min_vector.y..max_vector.y),
                rng.gen_range(min_vector.z..max_vector.z),
            )
        }
    }

    impl Add for Vec3 {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self::new(
                self.x + other.x,
                self.y + other.y,
                self.z + other.z
            )
        }
    }

    impl AddAssign for Vec3{
        fn add_assign(&mut self, other: Self) {
            *self = Self::new(
                self.x + other.x,
                self.y + other.y,
                self.z + other.z
            );
        }
    }

    impl Sub for Vec3 {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self::new(
                self.x - other.x,
                self.y - other.y,
                self.z - other.z,
            )
        }
    }

    impl SubAssign for Vec3 {
        fn sub_assign(&mut self, other: Self) {
            *self = Self::new(
                self.x - other.x,
                self.y - other.y,
                self.z - other.z
            );
        }
    }

    impl Div<Real> for Vec3 {
        type Output = Self;

        fn div(self, other: Real) -> Self {
            if other == 0 as Real {
                panic!("Division by zero!");
            }

            Self::new(
                self.x / other,
                self.y / other,
                self.z / other
            )
        }
    }

    impl DivAssign<Real> for Vec3 {
        fn div_assign(&mut self, other: Real) {
            *self = Self::new(
                self.x / other,
                self.y / other,
                self.z / other
            );
        }
    }

    impl Mul<Real> for Vec3 {
        type Output = Self;

        fn mul(self, other: Real) -> Self {
            Self::new(
                self.x * other,
                self.y * other,
                self.z * other
            )
        }
    }

    impl MulAssign<Real> for Vec3 {
        fn mul_assign(&mut self, other: Real) {
            *self = Self::new(
                self.x * other,
                self.y * other,
                self.z * other
            )
        }
    }

    impl Neg for Vec3 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Vec3::new(-self.x, -self.y, -self.z)
        }
    }

    impl Vec3 {
        /// Invert all the elements of the vector
        pub fn invert(&mut self) {
            *self = -(*self);
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
                *self *= 1.0 / m;
            }
        }

        /// Adds elements of the vector to the scaled correspondend element of 
        /// another vector, given as arguments.
        /// 
        /// ### Arguments
        /// * `v` - The vector to be scaled and added
        /// * `scale` - The scale factor
        pub fn inplace_add_scaled_vector(&mut self, v: &Vec3, scale: Real) {
            self.x += v.x * scale;
            self.y += v.y * scale;
            self.z += v.z * scale;
        }

        /// Returns the element-wise multiplycation of this vector 
        /// and the argument vector.
        pub fn component_product(&self, v: &Vec3) -> Vec3 {
            Vec3::new(
                self.x * v.x,
                self.y * v.y,
                self.z * v.z,
            )
        }

        /// Element-wise operation between this vector and the 
        /// given argument vector, the result is then replaced
        /// to the repective elements of this vector.
        pub fn inplace_component_product(&mut self, v: &Vec3) {
            self.x *= v.x;
            self.y *= v.y;
            self.z *= v.z;
        }

        /// Returns the scalar (dot) product between this vector
        /// and the one give as argument.
        pub fn dot_product(&self, v: &Vec3) -> Real {
            self.x * v.x + self.y * v.y + self.z * v.z
        }

        /// Returns the vectorial product between this vector
        /// and the given one.
        pub fn vector_product(&self, v: &Vec3) -> Vec3 {
            Vec3::new(
                self.y * v.z - self.z * v.y,
                self.z * v.x - self.x * v.z,
                self.x * v.y - self.y * v.x
            )
        }

        /// Calculates the vectorial product between this vector
        /// and the one given as argument, and then replaces the
        /// elements of this vector to the result of the product.
        pub fn inplace_vector_product(&mut self, v: &Vec3) {
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
        position: Vec3,
        /// Holds the linear velocity of the particle in world space
        velocity: Vec3,
        /// Holds the acceleration of the particle.
        acceleration: Vec3,
        /// Holds the amount of damping applied to linear motion.
        damping: Real,
        /// Holds the inverse of the mass of the particle.
        inverse_mass: Real,
    }

    pub trait ParticleLike {                                                                                                                                                  
        fn integrate(&mut self, duration: Real);
        fn get_mass(&self) -> Real;
        fn set_mass(&mut self, mass: Real);
        fn get_position(&self) -> Vec3;
        fn set_position(&mut self, position: Vec3);
        fn get_velocity(&self) -> Vec3;
        fn set_velocity(&mut self, velocity: Vec3);
        fn get_acceleration(&self) -> Vec3;
        fn set_acceleration(&mut self, acceleration: Vec3);
        fn get_damping(&self) -> Real;
        fn set_damping(&mut self, damping: Real);
    }

    impl ParticleLike for Particle {
        /// Integrates the particle forward in time by the given amount.
        fn integrate(&mut self, duration: Real) {
            if duration > 0 as Real {
                // Update linear position
                self.position += self.velocity * duration;
    
                // Work out the acceleration from the force
                let mut acc = self.acceleration.clone();
                acc += self.acceleration * duration;
    
                // Update linear velocity from the acceleration
                self.velocity += acc * duration;
    
                // Impose drag.
                self.velocity *= self.damping.powf(duration);
            }
        }

        fn get_position(&self) -> Vec3 {
            self.position.clone()
        }

        fn get_velocity(&self) -> Vec3 {
            self.velocity.clone()
        }

        fn get_acceleration(&self) -> Vec3 {
            self.acceleration.clone()
        }

        fn get_mass(&self) -> Real {
            1.0 / self.inverse_mass
        }

        fn set_mass(&mut self, mass: Real) {
            self.inverse_mass = 1.0 / mass;
        }

        fn set_position(&mut self, position: Vec3) {
            self.position = position;
        }

        fn set_velocity(&mut self, velocity: Vec3) {
            self.velocity = velocity;
        }

        fn set_acceleration(&mut self, acceleration: Vec3) {
            self.acceleration = acceleration;
        }

        fn set_damping(&mut self, damping: Real) {
            self.damping = damping;
        }

        fn get_damping(&self) -> Real {
            self.damping
        }
    }
    
    impl Particle {
        /// Creates a new particle at the give position
        pub fn from_position(position: Vec3, mass: Real, velocity: Vec3, acceleration: Vec3, damping: Real) -> Self {
            let inverse_mass: Real = 1.0 / mass;
            Particle { position, velocity, acceleration, damping, inverse_mass }
        }

        /// Creates a new particles and set the position automatically to the origin.
        pub fn new(mass: Real, velocity: Vec3, acceleration: Vec3, damping: Real) -> Self {
            let position = Vec3::from_origin();
            Particle::from_position(position, mass, velocity, acceleration, damping)            
        }
    } 
}
