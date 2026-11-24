const LEARNING_RATE: f32 = 0.1;

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

fn sigmoid_derivative(output: f32) -> f32 {
    output * (1.0 - output)
}

struct Neuron {
    pub weight_a: f32,
    pub weight_b: f32,
    pub bias: f32
}

impl Neuron {
    pub fn predict(&self, a: f32, b: f32, target: f32) -> f32 {
        let weighted_sum = (a * self.weight_a) + (b * self.weight_b);
        let raw_score = &weighted_sum + self.bias;
        let prediction = sigmoid(raw_score);

        prediction
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
