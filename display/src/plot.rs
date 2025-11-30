use terminal_size::{Width, Height, terminal_size};

fn map_y(y: f32, height: usize) -> usize {
    let ny = (1.0 - (y + 1.0) / 2.0) * (height as f32 - 1.0);
    ny as usize
}

pub fn plot_sin_and_pred<F>(predict: F, x_min: f32, x_max: f32)
where
    F: Fn(f32) -> f32,
{
    let size = terminal_size();
    let height;
    let width;
    if let Some((Width(w), Height(h))) = size {
        width = w as usize - 1;
        height = h as usize - 1;
    } else {
        println!("Unable to get terminal size");
        return;
    }
    let mut grid = vec![vec![" "; width]; height];

    for i in 0..width {
        let x = x_min + (i as f32) * (x_max - x_min) / (width - 1) as f32;

        let real = x.sin();
        let pred = predict(x);

        let real_row = map_y(real, height);
        let pred_row = map_y(pred, height);

        // real sin(x)
        grid[real_row][i] = "\x1b[33m*\x1b[0m";

        // prediction
        if real_row == pred_row {
            grid[pred_row][i] = "\x1b[32m@\x1b[0m"; // overlap marker
        } else {
            grid[pred_row][i] = "\x1b[31mo\x1b[0m";
        }
    }

    // print
    for row in 0..height {
        for col in 0..width {
            print!("{}", grid[row][col]);
        }
        println!();
    }
}
