use high_performance_tasks::utils;
use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let world_rank = world.rank() as usize;
    let world_size = world.size() as usize;
    for size in [10, 1000, 10_000_000] {
        let array = utils::generate_array(size);
        // Time measurement only for rank == 0
        let timestamp = if world_rank == 0 {
            Some(std::time::Instant::now())
        } else {
            None
        };
        let chunk_size = array.len() / world_size;
        let start = world_rank * chunk_size;
        let end = if world_rank != world_size - 1 {
            start + chunk_size
        } else {
            array.len()
        };
        let local_sum: i32 = array[start..end].iter().sum();
        let mut global_sum = 0;
        world.all_reduce_into(
            &local_sum,
            &mut global_sum,
            mpi::collective::SystemOperation::sum(),
        );
        world.barrier(); // Synchronization after calculations
        if let Some(timestamp) = timestamp {
            println!(
                "Size: {}, Time: {} seconds",
                array.len(),
                timestamp.elapsed().as_secs_f64()
            );
        }
    }
}
