use std::f64::consts::PI;

use ::glam::{DVec2, dvec2};
use ::rand::random_range;
use macroquad::prelude::*;
use particula_rs::{BaseParticleSystem, Particle, ParticleSystem};

fn window_conf() -> Conf {
    Conf {
        window_width: 500,
        window_height: 500,
        window_title: "Explosion".to_owned(),
        sample_count: 8,
        ..Default::default()
    }
}

pub struct BaseParticle {
    position: DVec2,
    velocity: DVec2,
    age: f64,
    max_age: f64,
}

impl BaseParticle {
    pub fn new(position: DVec2, velocity: DVec2, max_age: f64) -> Self {
        Self {
            position,
            velocity,
            age: 0.0,
            max_age,
        }
    }
}

impl Particle for BaseParticle {
    type Coordinate = DVec2;

    fn get_position(&self) -> DVec2 {
        self.position
    }

    fn update(&mut self, dt: f64) {
        self.position += self.velocity * dt;
        self.age += dt;
    }

    fn draw(&self) {
        draw_circle(
            self.get_position().x as f32,
            self.get_position().y as f32,
            1.0,
            WHITE,
        );
    }

    fn is_alive(&self) -> bool {
        self.age <= self.max_age
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut particle_system = BaseParticleSystem::<DVec2>::default();

    loop {
        particle_system.update(get_frame_time() as f64);

        clear_background(BLACK);

        particle_system.add_particle(Box::new(BaseParticle::new(
            dvec2(screen_width() as f64 / 2.0, screen_height() as f64 / 2.0),
            DVec2::from_angle(random_range(0.0..(2.0 * PI))) * random_range(100.0..=200.0),
            2.0,
        )));

        for particle in particle_system.iter_particles() {
            draw_circle(
                particle.get_position().x as f32,
                particle.get_position().y as f32,
                1.0,
                WHITE,
            );
        }

        next_frame().await;
    }
}
