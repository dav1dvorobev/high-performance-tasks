use high_performance_tasks::utils;
use rayon::prelude::*;

fn main() {
    // For simplification, take only square matrices
    for size in [10, 100, 1000] {
        let a = utils::generate_matrix(size, size);
        let b = utils::generate_matrix(size, size);
        let timestamp = std::time::Instant::now();
        let _: Vec<Vec<i32>> = (0..size)
            .into_par_iter()
            .map(|i| {
                (0..size)
                    .into_par_iter()
                    .map(|j| (0..size).map(|k| a[i][k] + b[k][j]).sum())
                    .collect::<Vec<i32>>()
            })
            .collect();
        println!(
            "Size: {}, Time: {} seconds",
            size,
            timestamp.elapsed().as_secs_f64()
        );
    }
}
