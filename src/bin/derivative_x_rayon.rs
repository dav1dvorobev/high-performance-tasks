use rayon::prelude::*;

fn f(x: f64, y: f64) -> f64 {
    x.powi(2) + y.powi(2)
}

fn main() {
    let h = 1.0;
    for size in [10, 100, 1000] {
        let a: Vec<_> = (0..size)
            .map(|i| {
                (0..size)
                    .map(|j| f(i as f64, j as f64))
                    .collect::<Vec<f64>>()
            })
            .collect();
        let timestamp = std::time::Instant::now();
        let _: Vec<_> = (0..size)
            .into_par_iter()
            .map(|i| {
                (0..size)
                    .into_par_iter()
                    .map(|j| {
                        if i == 0 {
                            return (a[i + 1][j] - a[i][j]) / h;
                        }
                        if i == size - 1 {
                            return (a[i][j] - a[i - 1][j]) / h;
                        }
                        (a[i + 1][j] - a[i - 1][j]) / (2.0 * h)
                    })
                    .collect::<Vec<f64>>()
            })
            .collect();
        println!(
            "Size: {}, Time: {} seconds",
            size,
            timestamp.elapsed().as_secs_f64()
        );
    }
}
