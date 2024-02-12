use crate::*;

#[derive(Clone, Debug)]
pub struct Neuron {
    // Each neuron is like a node in the network
    // The bias and weight determine whether the node accepts given inputs to move to the next layer
    pub(crate) bias: f32,
    pub(crate) weights: Vec<f32>,
}

impl Neuron {
    pub fn new(bias: f32, weights: Vec<f32>) -> Self {
        assert!(!weights.is_empty());

        Self { bias, weights }
    }
    // Generate random neuron
    pub fn random(rng: &mut dyn rand::RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        Self { bias, weights }
    }
    // Each neuron accepts N inputs but returns only 1 output
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        // Inputs and weights should have the same quantity
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        // Activation function - rectified linear unit (ReLU)
        (self.bias + output).max(0.0)
    }
    pub fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");
        let weights = (0..input_size)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();
        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test() {
            // Seed random number so its consistent
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);
            assert_eq!(neuron.bias, -0.6255188);
            assert_eq!(
                neuron.weights,
                &[0.67383957, 0.8181262, 0.26284897, 0.5238807]
            );
        }
    }

    mod propagate {
        use super::*;
        #[test]
        fn test() {
            let neuron = Neuron {
                bias: 0.3,
                weights: vec![-0.3, 0.8],
            };
            // Test that max() in the ReLU activation function works
            approx::assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0);
            // Test bias and weight calculation
            approx::assert_relative_eq!(
                neuron.propagate(&[0.25, 0.75]),
                (-0.3 * 0.25) + (0.8 * 0.75) + 0.3
            );
        }
    }
}
