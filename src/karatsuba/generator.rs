/// Randomly generate an input string composed of n digits.
pub fn generate_numeric_string(n: u64) -> String {
  (0..n)
    .map(|_| {
      let digit = rand::random_range(0..10); // Generates 0 to 9
      (b'0' + digit) as char
    })
    .collect()
}
