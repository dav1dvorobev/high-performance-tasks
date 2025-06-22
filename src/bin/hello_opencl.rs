use ocl::{Buffer, ProQue};

fn main() -> ocl::Result<()> {
    let src = r#"
        __kernel void hello(__global int *out) {
            out[get_global_id(0)] = get_global_id(0);
        }
    "#;
    let work_items = 32;
    let pro_que = ProQue::builder().src(src).dims(work_items).build()?;
    let buffer: Buffer<i32> = pro_que.create_buffer()?;
    let kernel = pro_que.kernel_builder("hello").arg(&buffer).build()?;
    unsafe {
        kernel.enq()?;
    }
    let mut output = vec![0; work_items];
    buffer.read(&mut output).enq()?;
    for id in output {
        println!("Hello from work-item #{}", id);
    }
    Ok(())
}
