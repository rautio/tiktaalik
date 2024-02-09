pub use self::{chromosome::*, crossover::*, individual::*, mutation::*, selection::*};

use rand::seq::SliceRandom;
use rand::{Rng, RngCore};

mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());
        (0..population.len())
            .map(|_| {
                // Select two individuals to mate
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                // Create a child crossover from the two parents
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
                // Mutate the child by introducing new genes not present in the parent
                // This helps avoid a local optimum and explore new paths in the population
                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();

        TestIndividual::create(chromosome)
    }

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        // Create the algorithm
        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMethod::new(0.5, 0.5),
        );
        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[2.0, 4.0, 2.0]),
        ];
        // Evolve over generations
        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population);
        }
        let expected = vec![
            individual(&[2.4740927, 4.789879, 1.6941864]),
            individual(&[1.0839049, 4.1960397, -0.5140536]),
            individual(&[1.8600199, 4.588976, 0.34133443]),
            individual(&[1.3374946, 4.392836, 1.9752667]),
        ];
        assert_eq!(expected[0].fitness(), 8.958158);
        assert_eq!(expected[1].fitness(), 4.765891);
        assert_eq!(expected[2].fitness(), 6.79033);
        assert_eq!(expected[3].fitness(), 7.705597);

        assert_eq!(population, expected);
    }
}
