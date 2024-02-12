pub use self::{animal::*, animal_individual::*, eye::*, food::*, world::*};
use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;

mod animal;
mod animal_individual;
mod eye;
mod food;
mod world;

const MIN_SPEED: f32 = 0.001;
const MAX_SPEED: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GENERATION_LENGTH: usize = 2500;
pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMethod::new(0.01, 0.3),
        );
        Self { world, ga, age: 0 }
    }
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            self.evolve(rng);
        }
    }
    pub fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);
            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
    pub fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);
                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }
    pub fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);
            let response = animal.brain.propagate(vision);
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);
            animal.speed = (animal.speed + speed).clamp(MIN_SPEED, MAX_SPEED);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
        }
    }
    pub fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        // Prep animals
        let current_population: Vec<_> = self
            .world
            .animals()
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        // Evolve animals
        let evolved_population = self.ga.evolve(rng, &current_population);

        // Set animals
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        // Reset food
        for food in &mut self.world.foods {
            food.position = rng.gen();
        }
    }
}
