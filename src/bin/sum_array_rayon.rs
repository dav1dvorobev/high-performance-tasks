use high_performance_tasks::utils;
use rayon::prelude::*;

fn main() {
    for size in [10, 1000, 10_000_000] {
        let array = utils::generate_array(size);
        let timestamp = std::time::Instant::now();
        let _: i32 = array.par_iter().sum();
        println!(
            "Size: {}, Time: {} seconds",
            array.len(),
            timestamp.elapsed().as_secs_f64()
        );
    }
}
