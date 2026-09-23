#[cfg(test)]
mod tests {
  use super::super::algorithm::{Point, closest_pair};

  #[test]
  fn closest_pair_basic() {
    let points = vec![
      Point::new(0.0, 0.0),
      Point::new(3.0, 4.0),
      Point::new(1.0, 1.0),
      Point::new(10.0, 10.0),
    ];

    let result = closest_pair(points);
    assert!(result.is_some());
    let (_, _, distance) = result.unwrap();
    assert!((distance - 1.41421356237).abs() < 1e-6);
  }

  #[test]
  fn closest_pair_in_strip() {
    let points = vec![
      Point::new(0.0, 0.0),
      Point::new(6.0, 6.0),
      Point::new(2.5, 2.5),
      Point::new(3.5, 2.5),
    ];

    let result = closest_pair(points);
    assert!(result.is_some());
    let (_, _, distance) = result.unwrap();
    assert_eq!(distance, 1.0);
  }

  #[test]
  fn closest_pair_in_strip_with_multiple() {
    let points = vec![
      Point::new(0.0, 0.0),
      Point::new(6.0, 6.0),
      Point::new(2.5, 12.5),
      Point::new(2.5, 22.6),
      Point::new(2.5, 32.7),
      Point::new(2.5, 42.8),
      Point::new(2.5, 52.9),
      Point::new(2.5, 62.10),
      Point::new(2.5, 72.11),
      Point::new(2.5, 82.12),
      Point::new(2.5, 92.13),
      Point::new(2.5, 2.14),
      Point::new(3.1, 2.14),
    ];

    let result = closest_pair(points);
    assert!(result.is_some());
    let (_, _, distance) = result.unwrap();
    assert_eq!(format!("{:.3}", distance), "0.600");
  }

  #[test]
  fn closest_pair_handles_two_points() {
    let points = vec![Point::new(0.0, 0.0), Point::new(5.0, 12.0)];
    let result = closest_pair(points);
    assert!(result.is_some());
    let (_, _, distance) = result.unwrap();
    assert!((distance - 13.0).abs() < 1e-6);
  }

  #[test]
  fn closest_pair_requires_two_points() {
    assert!(closest_pair(vec![Point::new(1.0, 1.0)]).is_none());
    assert!(closest_pair(vec![]).is_none());
  }
}
