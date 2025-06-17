use rand::Rng;

pub fn generate_array(size: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..size)
        .map(|_| rng.random_range(i32::MIN..i32::MAX))
        .collect()
}

pub fn generate_matrix(n: usize, m: usize) -> Vec<Vec<i32>> {
    (0..n).map(|_| generate_array(m)).collect()
}
