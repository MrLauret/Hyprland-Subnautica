const LEARNING_RATE: f32 = 0.1;

struct Neuron {
    pub weight_a: f32,
    pub weight_b: f32,
    pub bias: f32
}

fn main() {
    let inputs: Vec<Vec<f32>> = vec![
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 0.0],
    ];
}
