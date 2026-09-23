//! Implementation of Closest Pair algorithm to find the closest two points in a 2d environment.

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

impl Point {
  pub fn new(x: f64, y: f64) -> Self {
    Self { x, y }
  }

  pub fn distance_squared(&self, other: &Point) -> f64 {
    let dx = self.x - other.x;
    let dy = self.y - other.y;
    dx * dx + dy * dy
  }

  pub fn distance(&self, other: &Point) -> f64 {
    self.distance_squared(other).sqrt()
  }
}

pub fn closest_pair(points: Vec<Point>) -> Option<(Point, Point, f64)> {
  if points.len() < 2 {
    return None;
  }

  let mut by_x = points.to_vec();
  by_x.sort_by(|a, b| {
    a.x
      .partial_cmp(&b.x)
      .unwrap_or(Ordering::Equal)
      .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(Ordering::Equal))
  });

  let mut by_y = points.to_vec();
  by_y.sort_by(|a, b| {
    a.y
      .partial_cmp(&b.y)
      .unwrap_or(Ordering::Equal)
      .then_with(|| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal))
  });

  closest_pair_recursive(&by_x, &by_y)
}

fn closest_pair_recursive(px: &[Point], py: &[Point]) -> Option<(Point, Point, f64)> {
  let n = px.len();
  if n <= 3 {
    return brute_force_limited(px, None);
  }

  let mid = n / 2;
  let mid_x = px[mid].x;

  let left_px = &px[..mid];
  let right_px = &px[mid..];

  let left_py: Vec<_> = py.iter().copied().filter(|p| p.x <= mid_x).collect();
  let right_py: Vec<_> = py.iter().copied().filter(|p| p.x > mid_x).collect();

  let left_best = closest_pair_recursive(left_px, &left_py);
  let right_best = closest_pair_recursive(right_px, &right_py);
  let best = min_pair(left_best, right_best);

  let best_distance_sq = best.map_or(f64::INFINITY, |(_, _, d)| d * d);
  let mut strip = py
    .iter()
    .copied()
    .filter(|p| (p.x - mid_x).abs() < best_distance_sq.sqrt())
    .collect::<Vec<_>>();

  strip.sort_by(|a, b| {
    a.y
      .partial_cmp(&b.y)
      .unwrap_or(Ordering::Equal)
      .then_with(|| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal))
  });

  let best_strip = brute_force_limited(&strip, Some(best_distance_sq));
  min_pair(best, best_strip)
}

fn brute_force_limited(points: &[Point], best_distance_sq: Option<f64>) -> Option<(Point, Point, f64)> {
  let mut best = None;

  for i in 0..points.len() {
    let upper_bound = points
      .len()
      .min(if best_distance_sq.is_none() { usize::MAX } else { i + 8 });
    for j in (i + 1)..upper_bound {
      if best_distance_sq.is_some() {
        let dy = points[j].y - points[i].y;
        if dy * dy >= best_distance_sq.unwrap() {
          break;
        }
      }

      let distance = points[i].distance(&points[j]);
      let candidate = Some((points[i], points[j], distance));
      best = min_pair(best, candidate);
    }
  }

  best
}

fn min_pair(left: Option<(Point, Point, f64)>, right: Option<(Point, Point, f64)>) -> Option<(Point, Point, f64)> {
  match (left, right) {
    (None, None) => None,
    (Some(pair), None) | (None, Some(pair)) => Some(pair),
    (Some((a1, b1, d1)), Some((a2, b2, d2))) => {
      if d1 <= d2 {
        Some((a1, b1, d1))
      } else {
        Some((a2, b2, d2))
      }
    }
  }
}
