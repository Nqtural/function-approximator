use rand::{rng, Rng};
use std::fmt;

pub fn function_to_approximate(x: f32) -> f32 {
    x.sin()
    //x.cos()
    //x
    //x.abs() - 1.0
    //(-x * x).exp()
    //x.sin() + 0.3 * (3.0 * x).sin()
}

pub struct TrainingConfig {
    pub x_min: f32,
    pub x_max: f32,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            x_min: -(1.0 * std::f32::consts::PI),
            x_max:  (1.0 * std::f32::consts::PI),
        }
    }
}

pub struct TrainingDataPiece {
    input: f32,
    correct_answer: f32,
}

impl TrainingDataPiece {
    pub fn new(cfg: &TrainingConfig) -> Self {
        let num = rng().random_range(cfg.x_min..=cfg.x_max);
        Self {
            input: num,
            correct_answer: function_to_approximate(num),
        }
    }
}

impl fmt::Display for TrainingDataPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\x1b[1;31m{:<9}\x1b[0m -> \x1b[1;32m{}\x1b[0m",
            self.input,
            self.correct_answer
        )
    }
}

impl TrainingDataPiece {
    pub fn print_with_pred(&self, pred: f32) {
        println!("{:<8.2} -> pred: {:.4}, true: {:.4} (err: {:.4})", 
            self.input, pred, self.correct_answer, pred - self.correct_answer);
    }

    // GETTERS
    pub fn input(&self) -> f32 { self.input }
    pub fn correct_answer(&self) -> f32 { self.correct_answer }
}
