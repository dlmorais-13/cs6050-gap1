/// Generates a vector of random integers of the specified size.
pub fn generate_random_vector(n: u64) -> Vec<u64> {
  (0..n).map(|_| rand::random_range(0..=10u64.pow(n as u32))).collect()
}
