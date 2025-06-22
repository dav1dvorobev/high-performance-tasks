use high_performance_tasks::utils;
use ocl::{Buffer, ProQue};

fn main() -> ocl::Result<()> {
    for size in [10, 1_000, 10_000_000] {
        let array = utils::generate_array(size);
        let src = r#"
            __kernel void sum_reduction(__global int* input,
                                        __global int* partial_sums,
                                        __local int* local_data,
                                        const unsigned int n) {
                int gid = get_global_id(0);
                int lid = get_local_id(0);
                int group_size = get_local_size(0);
                int sum = 0;
                int idx = gid;
                while (idx < n) {
                    sum += input[idx];
                    idx += get_global_size(0);
                }
                local_data[lid] = sum;
                barrier(CLK_LOCAL_MEM_FENCE);
                for (int stride = group_size / 2; stride > 0; stride /= 2) {
                    if (lid < stride) {
                        local_data[lid] += local_data[lid + stride];
                    }
                    barrier(CLK_LOCAL_MEM_FENCE);
                }
                if (lid == 0) {
                    partial_sums[get_group_id(0)] = local_data[0];
                }
            }
        "#;
        let local_work_size = 256;
        let global_work_size = ((size + local_work_size - 1) / local_work_size) * local_work_size;
        let pro_que = ProQue::builder().src(src).dims(global_work_size).build()?;
        let input_buffer = Buffer::<i32>::builder()
            .queue(pro_que.queue().clone())
            .flags(ocl::flags::MEM_READ_ONLY)
            .len(global_work_size)
            .copy_host_slice(&array)
            .build()?;
        let partial_sum_len = global_work_size / local_work_size;
        let partial_sums_buffer = Buffer::<i32>::builder()
            .queue(pro_que.queue().clone())
            .flags(ocl::flags::MEM_WRITE_ONLY)
            .len(partial_sum_len)
            .build()?;
        let kernel = pro_que
            .kernel_builder("sum_reduction")
            .arg(&input_buffer)
            .arg(&partial_sums_buffer)
            .arg_local::<i32>(local_work_size)
            .arg(&(size as u32))
            .global_work_size(global_work_size)
            .local_work_size(local_work_size)
            .build()?;
        let timestamp = std::time::Instant::now();
        unsafe {
            kernel.enq()?;
        }
        let mut partial_sums = vec![0; partial_sum_len];
        partial_sums_buffer.read(&mut partial_sums).enq()?;
        let _: i32 = partial_sums.iter().sum();
        println!(
            "Size: {}, Time: {} seconds",
            size,
            timestamp.elapsed().as_secs_f64(),
        );
    }
    Ok(())
}
