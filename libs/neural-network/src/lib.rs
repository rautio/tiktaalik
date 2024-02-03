use self::{layer::*, neuron::*};
use rand::{Rng, RngCore};
use rand_chacha::ChaCha8Rng;

mod layer;
mod neuron;

// The Neural Network mapping the evolution decisions
#[derive(Clone, Debug)]
pub struct Network {
    // Layers in the network
    layers: Vec<Layer>,
}

#[derive(Clone, Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub(crate) fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }
    // Initialize the network with random values
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        // Moving window looking at current layer and next layer to determine input and output sizes
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();
        Self { layers }
    }

    // Inputs are propagated through each layer until we get to the output
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
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
            let network = Network::random(
                &mut rng,
                &[
                    LayerTopology { neurons: 3 },
                    LayerTopology { neurons: 2 },
                    LayerTopology { neurons: 1 },
                ],
            );
            assert_eq!(network.layers.len(), 2);
            // Validate first layer
            assert_eq!(network.layers[0].neurons.len(), 2);

            // Validate biases
            approx::assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);
            approx::assert_relative_eq!(network.layers[0].neurons[1].bias, 0.5238807);

            // Validate weights
            approx::assert_relative_eq!(
                network.layers[0].neurons[0].weights.as_slice(),
                &[0.67383957, 0.8181262, 0.26284897].as_slice()
            );
            approx::assert_relative_eq!(
                network.layers[0].neurons[1].weights.as_slice(),
                &[-0.53516835, 0.069369674, -0.7648182].as_slice()
            );

            // Validate second layer
            assert_eq!(network.layers[1].neurons.len(), 1);
            approx::assert_relative_eq!(network.layers[1].neurons[0].bias, -0.102499366);
            approx::assert_relative_eq!(
                network.layers[1].neurons[0].weights.as_slice(),
                &[-0.48879617, -0.19277132].as_slice()
            );
        }
    }
}
