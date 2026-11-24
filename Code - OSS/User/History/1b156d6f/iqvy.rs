const LEARNING_RATE: f32 = 0.1;

struct Neuron {
    pub weight_a: f32,
    pub weight_b: f32,
    pub bias: f32
}

impl Neuron {
    pub fn predict(&self, a: f32, b: f32, target: f32) -> f32 {

    }
}

fn main() {
    let inputs: Vec<Vec<f32>> = vec![
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 0.0],
    ];
}
