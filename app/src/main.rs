use trainingdata::TrainingDataPiece;

fn main() {
    for _ in 0..100 {
        println!("{}", TrainingDataPiece::default());
    }
}
