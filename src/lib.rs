/// A collection of particles and emitters
pub trait ParticleSystem {
    type Position;

    /// Returns an iterator over all currently alive particles in the system
    fn iter_particles(&self)
    -> impl Iterator<Item = &Box<dyn Particle<Position = Self::Position>>>;

    /// Returns a mutable iterator over all currently alive particles in the system
    fn iter_particles_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Box<dyn Particle<Position = Self::Position>>>;

    /// Returns an iterator over all currently alive particle emitters in the system
    fn iter_emitters(
        &self,
    ) -> impl Iterator<Item = &Box<dyn ParticleEmitter<Position = Self::Position>>>;

    /// Returns a mutable iterator over all currently alive particle emitters in the system
    fn iter_emitters_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Box<dyn ParticleEmitter<Position = Self::Position>>>;

    /// Adds a particle to the system
    fn add_particle(&mut self, particle: Box<dyn Particle<Position = Self::Position>>);

    /// Adds an emitter to the system
    fn add_emitter(&mut self, emitter: Box<dyn ParticleEmitter<Position = Self::Position>>);

    /// Iterates over all currently alive particles in the system and calls their update method
    fn update_particles(&mut self, dt: f64) {
        for particle in self.iter_particles_mut() {
            particle.update(dt);
        }
    }

    /// Iterates over all currently alive particle emitters in the system and calls their update method, returning the vector of new particles to add to the system
    fn update_emitters(&mut self, dt: f64) -> Vec<Box<dyn Particle<Position = Self::Position>>> {
        self.iter_emitters_mut()
            .flat_map(|emitter| emitter.update(dt))
            .collect()
    }

    /// Removes dead particles from the system
    fn clean_particles(&mut self);

    /// Removes dead emitters from the system
    fn clean_emitters(&mut self);

    /// Updates the particle system
    ///
    /// This method is comprised of 3 steps:
    /// 1. Update emitters and add the new particles to the system
    /// 2. Update all particles in the system
    /// 3. Remove dead particles and emitters from the system
    fn update(&mut self, dt: f64) {
        let new_particles = self.update_emitters(dt);

        for new_particle in new_particles {
            self.add_particle(new_particle);
        }

        self.update_particles(dt);

        self.clean_particles();
        self.clean_emitters();
    }

    /// Draws all particles in the system
    fn draw(&self) {
        for particle in self.iter_particles() {
            particle.draw();
        }
    }
}

/// A base particle system using vectors to store the particles and emitters
///
/// This should suffice for most particle system needs
#[derive(Default)]
pub struct BaseParticleSystem<P> {
    particles: Vec<Box<dyn Particle<Position = P>>>,
    emitters: Vec<Box<dyn ParticleEmitter<Position = P>>>,
}

impl<P> ParticleSystem for BaseParticleSystem<P> {
    /// The position type of the particles in the system
    type Position = P;

    fn iter_particles(
        &self,
    ) -> impl Iterator<Item = &Box<dyn Particle<Position = Self::Position>>> {
        self.particles.iter()
    }

    fn iter_particles_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Box<dyn Particle<Position = Self::Position>>> {
        self.particles.iter_mut()
    }

    fn iter_emitters(
        &self,
    ) -> impl Iterator<Item = &Box<dyn ParticleEmitter<Position = Self::Position>>> {
        self.emitters.iter()
    }

    fn iter_emitters_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Box<dyn ParticleEmitter<Position = Self::Position>>> {
        self.emitters.iter_mut()
    }

    fn add_particle(&mut self, particle: Box<dyn Particle<Position = Self::Position>>) {
        self.particles.push(particle);
    }

    fn add_emitter(&mut self, emitter: Box<dyn ParticleEmitter<Position = Self::Position>>) {
        self.emitters.push(emitter);
    }

    fn clean_particles(&mut self) {
        self.particles.retain(|particle| particle.is_alive());
    }

    fn clean_emitters(&mut self) {
        self.emitters.retain(|emitter| emitter.is_alive());
    }
}

/// Creates new particles
pub trait ParticleEmitter {
    /// The position type of the particles to be emitted
    type Position;

    /// Update the state of the emitter and return a vector of particles to add to the system
    fn update(&mut self, dt: f64) -> Vec<Box<dyn Particle<Position = Self::Position>>>;

    /// Returns false if the emitter should be removed from the system
    fn is_alive(&self) -> bool;
}

/// A representation of some particle in 2-dimensional space
pub trait Particle {
    /// The position type of the particle
    type Position;

    /// The position of the particle in space
    fn get_position(&self) -> Self::Position;

    /// Updates the state of the particle
    fn update(&mut self, dt: f64);

    /// Draws the particle
    fn draw(&self);

    /// Returns false if the particle should be removed from the system
    fn is_alive(&self) -> bool;
}
