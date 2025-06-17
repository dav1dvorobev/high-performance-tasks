use rayon::prelude::*;

fn main() {
    let n_threads = rayon::current_num_threads();
    (0..n_threads).into_par_iter().for_each(|_| {
        let thread_id = rayon::current_thread_index().unwrap_or(usize::MAX);
        println!("Hello from thread #{}", thread_id);
    });
}
