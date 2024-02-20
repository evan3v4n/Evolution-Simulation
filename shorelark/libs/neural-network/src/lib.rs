
pub struct Network {
    layers: Vec<Layer>,
}

struct Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}

impl Layer {
    fn propagate(&self,inputs: Vec<f32>) -> Vec<f32> {
        let mut outputs = Vec::new();

        for neuron in &self.neurons {
            let output = neuron.propagate(&inputs);
            outputs.push(output);
        }

        outputs
    }
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Network {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}



