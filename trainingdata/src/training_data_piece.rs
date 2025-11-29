use rand::{rng, Rng};
use std::fmt;

pub struct TrainingDataPiece {
    input: f32,
    correct_answer: f32,
}

impl Default for TrainingDataPiece {
    fn default() -> Self {
        let num = rng().random_range(0.0..=5000.0);
        Self {
            input: num,
            correct_answer: num.sin(),
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
