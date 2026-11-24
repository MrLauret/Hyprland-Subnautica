const LEARNING_RATE: f32 = 0.1;

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

fn sigmoid_derivative(output: f32) -> f32 {
    output * (1.0 - output)
}

struct XorNetwork {
    or_gate: Neuron,
    nand_gate: Neuron,
    and_gate: Neuron,
}

struct Neuron {
    pub weight_a: f32,
    pub weight_b: f32,
    pub bias: f32
}

impl Neuron {
    pub fn predict(&self, a: f32, b: f32) -> f32 {
        let weighted_sum = (a * self.weight_a) + (b * self.weight_b);
        let raw_score = weighted_sum + self.bias;
        let prediction = sigmoid(raw_score);

        prediction
    }

    pub fn train(&mut self, inputs: &Vec<Vec<f32>>, epochs: usize) {
        for _ in 0..epochs {
            for row in inputs {
                let a = row[0];
                let b = row[1];
                let target = row[2];
                
                let prediction = self.predict(a, b);
                let error = target - prediction;
                let delta = sigmoid_derivative(prediction) * error;
                
                self.weight_a += a * delta * LEARNING_RATE;
                self.weight_b += b * delta * LEARNING_RATE;
                self.bias += delta * LEARNING_RATE;
            }
        }

        println!("\n");
        println!("TRAINING FINISHED");
        println!("Weight a: {:.2}", self.weight_a);
        println!("Weight b: {:.2}", self.weight_b);
        println!("Bias: {:.2}", self.bias);
    }
}

impl XorNetwork {
    pub fn predict(&self, a: f32, b: f32) -> f32 {
        let or_out = self.or_gate.predict(a, b);
        let nand_out = self.or_gate.predict(a, b);

        self.and_gate.predict(or_out, nand_out)
    }
}

fn main() {
    let or_input: Vec<Vec<f32>> = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
    ];

    let nand_input: Vec<Vec<f32>> = vec![
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 0.0],
    ];

    let and_input: Vec<Vec<f32>> = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 1.0],
    ];

    let mut or_neuron = Neuron {weight_a: 1.0, weight_b: 1.0, bias: 1.0};
    let mut nand_neuron = Neuron {weight_a: 1.0, weight_b: 1.0, bias: 1.0};
    let mut and_neuron = Neuron {weight_a: 1.0, weight_b: 1.0, bias: 1.0};
    or_neuron.train(&or_input, 10000000);
    nand_neuron.train(&nand_input, 10000000);
    and_neuron.train(&and_input, 10000000);

    let or_input: Vec<Vec<f32>> = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 0.0],
    ];

    let xor_network = XorNetwork {or_gate: or_neuron, nand_gate: nand_neuron, and_gate: and_neuron};

    for row in or_input {
        println!("{:.2}%", xor_network.predict((row[0], row[1]) * 100.0))
    }
}
