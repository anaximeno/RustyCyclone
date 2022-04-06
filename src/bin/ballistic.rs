
pub mod ballistic {
    use rusty_cyclone::{
        core::Vec3,
        precision::Real,
        particle::*,
    };

    #[derive(Debug, PartialEq)]
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
        pub particle: Particle,
        pub radio: Real
    }

    impl Shot {
        fn new(shot_type: ShottingType, mass: Real, velocity: Vec3, acceleration: Vec3, radio: Real, damping: Real) -> Self {
            let position: Vec3 = Vec3::new(0.0, 1.5, 0.0);
            let start_time: Real = 0.0;
            let particle: Particle = Particle::from_position(
                    position,
                    mass,
                    velocity,
                    acceleration,
                    damping
            );

            Shot { shot_type, start_time, particle, radio }
        }

        fn new_pistol() -> Self {
            let mass: Real = 2.0;
            let velocity = Vec3::new(35.0, 0.0, 0.0);
            let acceleration = Vec3::new(0.0, 1.0, 0.0);
            let radio: Real = 5.0;
            let damping: Real = 0.99;

            Shot::new(ShottingType::PISTOL, mass, velocity, acceleration, radio, damping)
        }

        fn new_artillery() -> Self {
            let mass: Real = 200.0;
            let velocity = Vec3::new(40.0, 30.0, 0.0);
            let acceleration = Vec3::new(0.0, 20.0, 0.0);
            let radio: Real = 22.0;
            let damping: Real = 0.99;

            Shot::new(ShottingType::ARTILLERY, mass, velocity, acceleration, radio, damping)
        }

        fn new_fireball() -> Self {
            let mass: Real = 1.0;
            let velocity = Vec3::new(10.0, 0.0, 0.0);
            let acceleration = Vec3::new(0.0, -0.6, 0.0);
            let radio: Real = 10.0;
            let damping: Real = 0.9;

            Shot::new(ShottingType::FIREBALL, mass, velocity, acceleration, radio, damping)
        }

        fn new_laser() -> Self {
            let mass: Real = 0.1;
            let velocity = Vec3::new(100.0, 0.0, 00.0);
            let acceleration = Vec3::default();
            let radio: Real = 3.5;
            let damping: Real = 0.99;

            Shot::new(ShottingType::LASER, mass, velocity, acceleration, radio, damping)
        }

        fn new_unused() -> Self {
            let mass: Real = 0.0;
            let velocity = Vec3::default();
            let acceleration = Vec3::default();
            let radio: Real = 0.0;
            let damping: Real = 0.0;

            let mut shot = Shot::new(ShottingType::UNUSED, mass, velocity, acceleration, radio, damping);

            // Set the position to the center
            shot.particle.set_position(Vec3::default());

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

        pub fn set_start_time(&mut self, time: Real) {
            self.start_time = time;
        }

        pub fn set_shot_type(&mut self, shot_type: ShottingType) {
            self.shot_type = shot_type;
        }
    }
}

fn main() {
    use ballistic::{ ShottingType, Shot };
    use rusty_cyclone::precision::Real;
    use rusty_cyclone::particle::*;
    use raylib::prelude::*;

    let window_width: i32 = 480;
    let window_height: i32 = 740;

    let (mut rl, thd) = raylib::init()
        .size(window_width, window_height)
        .title("Ballistic Physic Demonstration")
        .build();

    rl.set_target_fps(60);

    let camera = Camera2D {
        target: Vector2 {
            x: 0.0,
            y: 0.0,
        },
        offset: Vector2 {
            x: 0.0,
            y: 0.0,
        },
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut shot = Shot::from(ShottingType::ARTILLERY);
    shot.set_start_time(rl.get_frame_time() as Real);

    while !rl.window_should_close() {
        let delta: Real = rl.get_frame_time() as Real;
        let mut d = rl.begin_drawing(&thd);
        d.clear_background(Color::GRAY);

        {
            let mut mode = d.begin_mode2D(&camera);
            let position = shot.particle.get_position();
            let x = position.x as i32;
            let y = position.y as i32;
            let radio = shot.radio as f32;
            mode.draw_circle(x, y, radio, Color::RAYWHITE);
        }

        match shot.shot_type {
            ShottingType::UNUSED => (),
            _ => {
                if delta > 0.0 {
                    shot.particle.integrate(delta as Real);
                }

                let win_y_limit: Real = (window_height as Real) - shot.radio;
                let win_x_limit: Real = (window_width as Real) + shot.radio;
                let time_limit:  Real = shot.start_time + 5000.0;
                let position = shot.particle.get_position();

                if position.y > win_y_limit ||
                   position.x > win_x_limit ||
                   time_limit < delta {
                    shot.set_shot_type(ShottingType::UNUSED);
                }
            }
        };
    }
}
