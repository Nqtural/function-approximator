use display::plot_sin_and_pred;
use training::{TrainingConfig, TrainingDataPiece};
use neuralnet::NeuralNet;

fn train_epoch(net: &mut NeuralNet, batch_size: usize, lr: f32, cfg: &TrainingConfig) -> f32 {
    let mut total_loss = 0.0;
    for _ in 0..batch_size {
        let piece = TrainingDataPiece::new(cfg);
        let loss = net.train_step(piece.input(), piece.correct_answer(), lr);
        total_loss += loss;
    }
    total_loss / batch_size as f32
}

fn main() {
    let mut net = NeuralNet::new();
    let mut avg_loss = 0.0;
    let cfg = TrainingConfig::default();

    for epoch in 0..50000 {
        let lr = if epoch < 30000 { 0.0003 } 
                 else if epoch < 45000 { 0.0001 } 
                 else { 0.00003 };
        let epoch_loss = train_epoch(&mut net, 512, lr, &cfg);
        avg_loss = avg_loss * 0.9 + epoch_loss * 0.1;

        if epoch % 500 == 0 {
            plot_sin_and_pred(|x| net.predict(x), cfg.x_min, cfg.x_max);
        }
    }
}
