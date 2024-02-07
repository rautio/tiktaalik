use crate::*;
use rand::RngCore;

#[derive(Clone, Debug)]

pub struct GaussianMethod {
    // Probability of changing a gene
    chance: f32,
    // Magnitude of that change
    coeff: f32,
}

impl GaussianMethod {
    fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);
        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn actual(chance: f32, coeff: f32) -> Vec<f32> {
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        GaussianMethod::new(chance, coeff).mutate(&mut rng, &mut child);

        child.into_iter().collect()
    }

    mod given_zero_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.0, coeff)
        }
        mod and_zero_coefficient {
            use super::*;
            #[test]
            fn does_not_change_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
        mod and_nonzero_coefficient {
            use super::*;
            #[test]
            fn does_not_change_original_chromosome() {
                let actual = actual(0.8);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_fifty_fifty_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.5, coeff)
        }
        mod and_zero_coefficient {
            use super::*;
            #[test]
            fn does_not_change_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
        mod and_nonzero_coefficient {
            use super::*;
            #[test]
            fn slightly_changes_original_chromosome() {
                let actual = actual(0.8);
                let expected = vec![1.0, 2.3590002, 3.0, 3.7445111, 5.];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_max_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(1.0, coeff)
        }
        mod and_zero_coefficient {
            use super::*;
            #[test]
            fn does_not_change_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
        mod and_nonzero_coefficient {
            use super::*;
            #[test]
            fn entirely_changes_original_chromosome() {
                let actual = actual(0.8);
                let expected = vec![0.27274954, 1.8140674, 3.3590002, 4.07918, 5.5780945];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }
}
