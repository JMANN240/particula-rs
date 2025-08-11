/// A collection of particles and emitters
pub trait ParticleSystem {
    /// The type of particle that this system will contain
    type ParticleType: Particle + ?Sized;

    /// The type of emitter that this system will contain
    type EmitterType: ParticleEmitter<ParticleType = Self::ParticleType> + ?Sized;

    /// Returns an iterator over all currently alive particles in the system
    fn iter_particles(&self)
    -> impl Iterator<Item = &Box<Self::ParticleType>>;

    /// Returns a mutable iterator over all currently alive particles in the system
    fn iter_particles_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Box<Self::ParticleType>>;

    /// Returns an iterator over all currently alive particle emitters in the system
    fn iter_emitters(
        &self,
    ) -> impl Iterator<Item = &Box<Self::EmitterType>>;

    /// Returns a mutable iterator over all currently alive particle emitters in the system
    fn iter_emitters_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Box<Self::EmitterType>>;

    /// Adds a particle to the system
    fn add_particle(&mut self, particle: Box<Self::ParticleType>);

    /// Adds an emitter to the system
    fn add_emitter(&mut self, emitter: Box<Self::EmitterType>);

    /// Iterates over all currently alive particles in the system and calls their update method
    fn update_particles(&mut self, dt: f64) {
        for particle in self.iter_particles_mut() {
            particle.update(dt);
        }
    }

    /// Iterates over all currently alive particle emitters in the system and calls their update method, returning the vector of new particles to add to the system
    fn update_emitters(&mut self, dt: f64) -> Vec<Box<Self::ParticleType>> {
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
    emitters: Vec<Box<dyn ParticleEmitter<ParticleType = dyn Particle<Position = P>>>>,
}

impl<P> ParticleSystem for BaseParticleSystem<P> {
    /// This system can hold any particle that implements `Particle` with the same `Position` type
    type ParticleType = dyn Particle<Position = P>;

    /// This system can hold any emitter that emits any particle that implements `Particle` with the same `Position` type
    type EmitterType = dyn ParticleEmitter<ParticleType = Self::ParticleType>;

    fn iter_particles(
        &self,
    ) -> impl Iterator<Item = &Box<Self::ParticleType>> {
        self.particles.iter()
    }

    fn iter_particles_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Box<Self::ParticleType>> {
        self.particles.iter_mut()
    }

    fn iter_emitters(
        &self,
    ) -> impl Iterator<Item = &Box<Self::EmitterType>> {
        self.emitters.iter()
    }

    fn iter_emitters_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Box<Self::EmitterType>> {
        self.emitters.iter_mut()
    }

    fn add_particle(&mut self, particle: Box<Self::ParticleType>) {
        self.particles.push(particle);
    }

    fn add_emitter(&mut self, emitter: Box<Self::EmitterType>) {
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
    /// The type of the particles to be emitted
    type ParticleType: Particle + ?Sized;

    /// Update the state of the emitter and return a vector of particles to add to the system
    fn update(&mut self, dt: f64) -> Vec<Box<Self::ParticleType>>;

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

/// Tracks age in a particle
pub trait Aging {
    /// Gets the current age of the particle
    fn get_age(&self) -> f64;

    /// Sets the current age of the particle
    fn set_age(&mut self, age: f64);
}

/// Represents a particle that dies after a set amount of time
pub trait MaxAging: Aging {
    /// Gets the max age of the particle
    fn get_max_age(&self) -> f64;

    /// Gets the age of the particle from 0.0 to 1.0
    fn get_age_percent(&self) -> f64 {
        self.get_age() / self.get_max_age()
    }

    /// Returns false if the particle's age percent is greater than or equal to 1.0
    fn is_alive(&self) -> bool {
        self.get_age_percent() < 1.0
    }
}
