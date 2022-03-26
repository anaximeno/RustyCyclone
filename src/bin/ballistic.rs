
pub mod ballistic {
    use rusty_cyclone::{
        core::Vector3,
        particle::Particle,
        precision::Real
    };

    #[derive(Debug)]
    pub enum ShottingType {
        PISTOL,
        ARTILLERY,
        FIREBALL,
        LASER,
        UNUSED
    }

    #[derive(Debug)]
    pub struct Shot {
        pub shot_type: ShottingType,
        pub start_time: Real,
        pub particle: Particle
    }

    impl Shot {
        fn new(shot_type: ShottingType, mass: Real, velocity: Vector3, acceleration: Vector3, damping: Real) -> Self {
            let position: Vector3 = Vector3::new(0.0, 1.5, 0.0);
            let start_time: Real = 0.1; // FIXME: must get last frame timestamp
            let particle: Particle = Particle::from_position(
                    position,
                    mass,
                    velocity,
                    acceleration,
                    damping
            );

            Shot { shot_type, start_time, particle }
        }
        
        fn new_pistol() -> Self {
            let mass: Real = 2.0;
            let velocity: Vector3 = Vector3::new(0.0, 0.0, 35.0);
            let acceleration: Vector3 = Vector3::new(0.0, -1.0, 0.0);
            let damping: Real = 0.99;

            Shot::new(ShottingType::PISTOL, mass, velocity, acceleration, damping)
        }

        fn new_artillery() -> Self {
            let mass: Real = 200.0;
            let velocity: Vector3 = Vector3::new(0.0, 30.0, 40.0);
            let acceleration: Vector3 = Vector3::new(0.0, -20.0, 0.0);
            let damping: Real = 0.99;

            Shot::new(ShottingType::ARTILLERY, mass, velocity, acceleration, damping)
        }

        fn new_fireball() -> Self {
            let mass: Real = 1.0;
            let velocity: Vector3 = Vector3::new(0.0, 0.0, 10.0);
            let acceleration: Vector3 = Vector3::new(0.0, 0.6, 0.0);
            let damping: Real = 0.9;

            Shot::new(ShottingType::FIREBALL, mass, velocity, acceleration, damping)
        }

        fn new_laser() -> Self {
            let mass: Real = 0.1;
            let velocity: Vector3 = Vector3::new(0.0, 0.0, 100.0);
            let acceleration: Vector3 = Vector3::from_origin();
            let damping: Real = 0.99;

            Shot::new(ShottingType::LASER, mass, velocity, acceleration, damping)
        }

        fn new_unused() -> Self {
            let mass: Real = 0.0;
            let velocity: Vector3 = Vector3::from_origin();
            let acceleration: Vector3 = Vector3::from_origin();
            let damping: Real = 0.0;

            let mut shot = Shot::new(ShottingType::UNUSED, mass, velocity, acceleration, damping);

            // Set the position to the center
            shot.particle.set_position(Vector3::from_origin());

            // Returning the unused shot
            shot
        }

        pub fn from(shot_type: ShottingType) -> Self {
            use ShottingType::*;

            match shot_type {
                PISTOL    => Shot::new_pistol(),
                ARTILLERY => Shot::new_artillery(),
                FIREBALL  => Shot::new_fireball(),
                LASER     => Shot::new_laser(),
                UNUSED    => Shot::new_unused()
            }
        }
    }
}

fn main() {
    use ballistic::{ ShottingType, Shot };
    let shot = Shot::from(ShottingType::PISTOL);
    println!("{:#?}", shot);
}
