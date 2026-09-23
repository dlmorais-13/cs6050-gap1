#[cfg(test)]
mod tests {
  use super::super::algorithm::powersort;

  #[test]
  fn sorts_integers() {
    let input = vec![9, 3, 7, 1, 5, 2, 8, 4, 6];
    let sorted = powersort(input);
    assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
  }

  #[test]
  fn handles_empty_and_singleton_inputs() {
    assert_eq!(powersort(Vec::<i32>::new()), Vec::<i32>::new());
    assert_eq!(powersort(vec![42]), vec![42]);
  }

  #[test]
  fn sorts_values_with_duplicates() {
    let input = vec![5, 1, 5, 2, 3, 1, 4];
    let sorted = powersort(input);
    assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 5]);
  }
}
