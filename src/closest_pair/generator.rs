use crate::closest_pair::algorithm::Point;

/// Generates an array of random points of the specified size.
pub fn generate_random_vector(n: u64) -> Vec<Point> {
  (0..n)
    .map(|_| {
      Point::new(
        rand::random_range(0.0..10f64.powi(16)),
        rand::random_range(0.0..10f64.powi(16)),
      )
    })
    .collect()
}
