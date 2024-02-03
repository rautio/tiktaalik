use crate::*;

#[derive(Clone, Debug)]
pub struct Layer {
    // Each layer consists of a set of neurons
    pub(crate) neurons: Vec<Neuron>,
}

impl Layer {
    pub(crate) fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());
        // Make sure all neurons are the same size
        assert!(neurons
            .iter()
            .all(|neuron| neuron.weights.len() == neurons[0].weights.len()));
        Self { neurons }
    }
    // Initialize layer with random neurons
    pub fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();
        Self { neurons }
    }
    // Pass the inputs through each neuron in the layer until we have a list of outputs to pass to the next layer
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
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
            let layer = Layer::random(&mut rng, 3, 2);

            let actual_biases: Vec<_> = layer.neurons.iter().map(|neuron| neuron.bias).collect();
            let expected_biases = vec![-0.6255188, 0.5238807];

            let actual_weights: Vec<_> = layer
                .neurons
                .iter()
                .map(|neuron| neuron.weights.as_slice())
                .collect();
            let expected_weights: Vec<&[f32]> = vec![
                &[0.67383957, 0.8181262, 0.26284897],
                &[-0.53516835, 0.069369674, -0.7648182],
            ];

            approx::assert_relative_eq!(actual_biases.as_slice(), expected_biases.as_slice());
            approx::assert_relative_eq!(actual_weights.as_slice(), expected_weights.as_slice());
        }
    }

    mod propagate {
        use super::*;
        #[test]
        fn test() {
            let neurons = vec![
                Neuron::new(0.3, vec![-0.3, 0.8, 0.11]),
                Neuron::new(-0.2, vec![0.3, 0.4, 0.5]),
            ];
            let layer = Layer::new(vec![neurons[0].clone(), neurons[1].clone()]);
            let inputs = &[-0.25, 0.25, 0.75];
            let actual = layer.propagate(inputs.to_vec());
            let expected = vec![neurons[0].propagate(inputs), neurons[1].propagate(inputs)];
            approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }
}
