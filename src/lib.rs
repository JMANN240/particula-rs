use glam::DVec2;

pub trait ParticleSystem {
    fn iter_particles(&self) -> impl Iterator<Item = &Box<dyn Particle>>;

    fn add_particle(&mut self, particle: Box<dyn Particle>);
    fn add_emitter(&mut self, emitter: Box<dyn ParticleEmitter>);

    fn update_particles(&mut self, dt: f64);
    fn update_emitters(&mut self, dt: f64) -> Vec<Box<dyn Particle>>;

    fn clean_particles(&mut self);
    fn clean_emitters(&mut self);

    fn update(&mut self, dt: f64) {
        let new_particles = self.update_emitters(dt);

        for new_particle in new_particles {
            self.add_particle(new_particle);
        }

        self.update_particles(dt);

        self.clean_particles();
        self.clean_emitters();
    }
}

#[derive(Default)]
pub struct BaseParticleSystem {
    particles: Vec<Box<dyn Particle>>,
    emitters: Vec<Box<dyn ParticleEmitter>>,
}

impl ParticleSystem for BaseParticleSystem {
    fn iter_particles(&self) -> impl Iterator<Item = &Box<dyn Particle>> {
        self.particles.iter()
    }

    fn add_particle(&mut self, particle: Box<dyn Particle>) {
        self.particles.push(particle);
    }

    fn add_emitter(&mut self, emitter: Box<dyn ParticleEmitter>) {
        self.emitters.push(emitter);
    }

    fn update_particles(&mut self, dt: f64) {
        for particle in self.particles.iter_mut() {
            particle.update(dt);
        }
    }

    fn update_emitters(&mut self, dt: f64) -> Vec<Box<dyn Particle>> {
        self.emitters
            .iter_mut()
            .flat_map(|emitter| emitter.update(dt))
            .collect()
    }

    fn clean_particles(&mut self) {
        self.particles.retain(|particle| particle.is_alive());
    }

    fn clean_emitters(&mut self) {
        self.emitters.retain(|emitter| emitter.is_alive());
    }
}

pub trait ParticleEmitter {
    fn update(&mut self, dt: f64) -> Vec<Box<dyn Particle>>;
    fn is_alive(&self) -> bool;
}

pub trait ParticleDrawer {
    fn draw(&self, particle: &dyn Particle);
}

pub trait Particle {
    fn get_position(&self) -> DVec2;
    fn update(&mut self, dt: f64);
    fn draw(&self);
    fn is_alive(&self) -> bool;
}
