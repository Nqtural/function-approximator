use rand::Rng;
use crate::operations::{matrix_multiply, add_vectors, tanh_derivative, tanh_vec};

#[derive(Default, Debug, Clone)]
pub struct NeuralNet {
    w1: Vec<Vec<f32>>,
    b1: Vec<f32>,
    w2: Vec<Vec<f32>>,
    b2: f32,
}

impl NeuralNet {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let hidden_size = 8;

        // Xavier init: scale by sqrt(fan_in * fan_out)
        let w1_scale = 1.0 / (1.0f32.sqrt() * hidden_size as f32);
        let w2_scale = 1.0 / (hidden_size as f32).sqrt();

        let w1 = vec![(0..hidden_size).map(|_| rng.random_range(-w1_scale..w1_scale)).collect()];
        let b1 = vec![0.0; hidden_size];
        let w2: Vec<Vec<f32>> = (0..hidden_size).map(|_| vec![rng.random_range(-w2_scale..w2_scale)]).collect();
        let b2 = 0.0;

        Self { w1, b1, w2, b2 }
    }

    fn mse(prediction: f32, answer: f32) -> f32 {
        let diff = prediction - answer;
        diff * diff
    }

    pub fn train_step(&mut self, input: f32, target: f32, lr: f32) -> f32 {
        let (z1, a1, y_pred) = self.forward(input);
        let loss = Self::mse(y_pred, target);

        let dl_dy = 2.0 * (y_pred - target);

        let dl_dw2: Vec<Vec<f32>> = a1.iter()
            .map(|&a| vec![a * dl_dy])
            .collect();
        let dl_db2 = dl_dy;

        let dl_da1: Vec<f32> = self.w2.iter()
            .map(|w| w[0] * dl_dy)
            .collect();

        let dl_dz1: Vec<f32> = dl_da1.iter()
            .zip(z1.iter())
            .map(|(&da, &z)| da * tanh_derivative(z))
            .collect();

        // clip gradients
        let max_grad = 1.0;
        let dl_dz1_norm = dl_dz1.iter().map(|g| g*g).sum::<f32>().sqrt();
        let scale = if dl_dz1_norm > max_grad { max_grad / dl_dz1_norm } else { 1.0 };
        let dl_dz1_clipped: Vec<f32> = dl_dz1.iter().map(|&g| g * scale).collect();
        let dl_dw1 = vec![dl_dz1_clipped.iter().map(|&dz| input * dz).collect()];

        let dl_db1 = dl_dz1_clipped;

        self.update_weights(&dl_dw1, &dl_db1, &dl_dw2, dl_db2, lr);

        loss
    }

    pub fn predict(&self, input: f32) -> f32 {
        let (_, _, y_pred) = self.forward(input);
        y_pred
    }

    fn forward(&self, input: f32) -> (Vec<f32>, Vec<f32>, f32) {
        let input_mat = vec![vec![input]];

        // hidden layer
        let z1_pre = matrix_multiply(&input_mat, &self.w1);
        let z1_pre_flat: Vec<f32> = z1_pre[0].clone();
        let z1 = add_vectors(&z1_pre_flat, &self.b1);

        let a1 = tanh_vec(&z1);

        // output layer
        let a1_mat = vec![a1.clone()];  // shape: 1 x hidden_size
        let y_pred_pre = matrix_multiply(&a1_mat, &self.w2);
        let y_pred = y_pred_pre[0][0] + self.b2;

        (z1, a1, y_pred)
    }

    fn update_weights(
        &mut self,
        dl_dw1: &[Vec<f32>],
        dl_db1: &[f32],
        dl_dw2: &[Vec<f32>],
        dl_db2: f32,
        lr: f32,
    ) {
        // w1
        self.w1[0].iter_mut()
            .zip(dl_dw1[0].iter())
            .for_each(|(w, &grad)| *w -= lr * grad);

        // b1
        self.b1.iter_mut()
            .zip(dl_db1.iter())
            .for_each(|(b, &grad)| *b -= lr * grad);

        // w2
        self.w2.iter_mut()
            .zip(dl_dw2.iter())
            .for_each(|(w_row, grad_row)| w_row[0] -= lr * grad_row[0]);

        // b2
        self.b2 -= lr * dl_db2;
    }

    // GETTERS
    pub fn w1(&self) -> &[Vec<f32>] {
        &self.w1
    }
}
