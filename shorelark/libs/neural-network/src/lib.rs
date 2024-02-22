use rand::prelude::*;

pub struct Network {
    layers: Vec<Layer>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

struct Layer {
    neurons: Vec<Neuron>
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // functional progreamming very Haskall-y
        // Use rust
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn random(layers: &[LayerTopology] ) -> Self {
        assert!(layers.len() > 1);

        // let mut built_layers = Vec::new();

         let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(layers[0].neurons, layers[1].neurons)
            })
            .collect();

        Self { layers }
    }
}

impl Neuron { 
    fn propagate(&self, inputs: &[f32]) -> f32 {
        // edge case
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }

    pub fn random(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();
    
        Self { bias, weights }
    }   
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
         self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn random(input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(input_size)) 
            .collect();
        
        Self { neurons }
    }
}