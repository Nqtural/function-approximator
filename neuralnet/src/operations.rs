pub fn matrix_multiply(a: &[Vec<f32>], b: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let n = a.len();
    let m = a[0].len();
    let p = b[0].len();
    
    let mut r = vec![vec![0.0; p]; n];
    
    for i in 0..n {
        for j in 0..p {
            let mut sum = 0.0;
            for k in 0..m {
                sum += a[i][k] * b[k][j];
            }
            r[i][j] = sum;
        }
    }
    
    r
}

pub fn add_vectors(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter().zip(b).map(|(a, b)| a + b).collect()
}

// clamp values to avoid NaN
pub fn tanh(x: f32) -> f32 {
    if x > 20.0 { 1.0 }
    else if x < -20.0 { -1.0 }
    else { x.tanh() }
}

pub fn tanh_vec(x: &[f32]) -> Vec<f32> {
    x.iter().map(|&xi| tanh(xi)).collect()
}

pub fn tanh_derivative(x: f32) -> f32 {
    1.0 - (tanh(x) * tanh(x))
}
